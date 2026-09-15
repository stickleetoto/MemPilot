use std::{env, process::ExitCode, thread, time::Duration};

use mempilot::{
    adaptive::AdaptiveTracker, optimizer, policy, pressure::PressureEngine, telemetry, NAME,
    VERSION,
};

fn print_help() {
    println!("{NAME} {VERSION}");
    println!("Adaptive memory pressure manager for Windows\n");
    println!("USAGE:");
    println!("  mempilot <COMMAND> [OPTIONS]\n");
    println!("COMMANDS:");
    println!("  status [--json]                  Show live system memory pressure");
    println!("  analyze [--top N] [--json]       Rank background reclaim candidates");
    println!("  watch [--interval S] [--count N] Continuously sample pressure");
    println!("  optimize --dry-run [--top N]     Build a safe optimization plan");
    println!("  version                          Show version");
    println!("  help                             Show this help");
    println!("\nMemPilot v0.2 never mutates process memory. Optimization remains dry-run only.");
}

fn main() -> ExitCode {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let result = match args.first().map(String::as_str) {
        None | Some("help") | Some("--help") | Some("-h") => {
            print_help();
            Ok(())
        }
        Some("version") | Some("--version") | Some("-V") => {
            println!("{NAME} {VERSION}");
            Ok(())
        }
        Some("status") => command_status(&args[1..]),
        Some("analyze") => command_analyze(&args[1..]),
        Some("watch") => command_watch(&args[1..]),
        Some("optimize") => command_optimize(&args[1..]),
        Some(other) => Err(format!("unknown command: {other}")),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::from(2)
        }
    }
}

fn command_status(args: &[String]) -> Result<(), String> {
    reject_unknown_flags(args, &["--json"])?;
    let json = has_flag(args, "--json");
    let snapshot = telemetry::system_snapshot().map_err(|error| error.to_string())?;
    let mut engine = PressureEngine::default();
    let assessment = engine.update(&snapshot);

    if json {
        let output = serde_json::json!({
            "system": snapshot,
            "pressure": assessment,
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&output).map_err(|error| error.to_string())?
        );
        return Ok(());
    }

    println!("{NAME} {VERSION}");
    println!("Pressure: {} ({:.1}/100)", assessment.level, assessment.smoothed_score);
    println!(
        "Physical: {:.1}% used | {:.1} GiB available / {:.1} GiB total",
        assessment.physical_pressure * 100.0,
        gib(snapshot.available_physical_bytes),
        gib(snapshot.total_physical_bytes),
    );
    println!(
        "Commit: {:.1}% | {:.1} GiB / {:.1} GiB",
        assessment.commit_pressure * 100.0,
        gib(snapshot.commit_total_bytes),
        gib(snapshot.commit_limit_bytes),
    );
    println!("System cache: {:.1} GiB", gib(snapshot.system_cache_bytes));
    println!(
        "Kernel pools: paged {:.1} MiB | nonpaged {:.1} MiB",
        mib(snapshot.paged_pool_bytes),
        mib(snapshot.nonpaged_pool_bytes),
    );
    println!(
        "Low-memory signal: {}",
        snapshot
            .low_memory_signal
            .map(|value| if value { "active" } else { "clear" })
            .unwrap_or("unavailable")
    );
    Ok(())
}

fn command_analyze(args: &[String]) -> Result<(), String> {
    let top = parse_usize_option(args, "--top", 10)?;
    reject_unknown_flags_with_values(args, &["--json"], &["--top"])?;
    let json = has_flag(args, "--json");

    let processes = telemetry::process_snapshots().map_err(|error| error.to_string())?;
    let mut tracker = AdaptiveTracker::default();
    let deltas = tracker.observe(&processes);
    let ranked = policy::rank_candidates(&processes, &deltas, &tracker);

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&ranked.iter().take(top).collect::<Vec<_>>())
                .map_err(|error| error.to_string())?
        );
        return Ok(());
    }

    println!("Visible processes: {}", processes.len());
    println!("Top reclaim candidates (observation-only):");
    for candidate in ranked.into_iter().take(top) {
        println!(
            "  {:>5.1}  PID {:>6}  {:<28}  expected <= {:>7.0} MiB  risk {:>3.0}%",
            candidate.score,
            candidate.pid,
            truncate(&candidate.image_name, 28),
            mib(candidate.expected_reclaim_bytes),
            candidate.refault_risk * 100.0,
        );
    }
    Ok(())
}

fn command_watch(args: &[String]) -> Result<(), String> {
    let interval = parse_u64_option(args, "--interval", 2)?.max(1);
    let count = parse_usize_option(args, "--count", 0)?;
    reject_unknown_flags_with_values(args, &[], &["--interval", "--count"])?;

    let mut engine = PressureEngine::default();
    let mut tracker = AdaptiveTracker::default();
    let mut iteration = 0usize;

    loop {
        let snapshot = telemetry::system_snapshot().map_err(|error| error.to_string())?;
        let assessment = engine.update(&snapshot);
        let processes = telemetry::process_snapshots().unwrap_or_default();
        let deltas = tracker.observe(&processes);
        let candidates = policy::rank_candidates(&processes, &deltas, &tracker);

        println!(
            "{:>8} {:>5.1}/100 | avail {:>6.1} GiB | commit {:>5.1}% | candidates {}",
            assessment.level,
            assessment.smoothed_score,
            gib(snapshot.available_physical_bytes),
            assessment.commit_pressure * 100.0,
            candidates.len(),
        );

        iteration = iteration.saturating_add(1);
        if count != 0 && iteration >= count {
            break;
        }
        thread::sleep(Duration::from_secs(interval));
    }

    Ok(())
}

fn command_optimize(args: &[String]) -> Result<(), String> {
    if !has_flag(args, "--dry-run") {
        return Err("v0.2 safety gate: only `optimize --dry-run` is available".into());
    }

    let top = parse_usize_option(args, "--top", 10)?;
    reject_unknown_flags_with_values(args, &["--dry-run", "--json"], &["--top"])?;
    let json = has_flag(args, "--json");

    let snapshot = telemetry::system_snapshot().map_err(|error| error.to_string())?;
    let processes = telemetry::process_snapshots().map_err(|error| error.to_string())?;

    let mut pressure_engine = PressureEngine::default();
    let assessment = pressure_engine.update(&snapshot);

    let mut tracker = AdaptiveTracker::default();
    let deltas = tracker.observe(&processes);
    let candidates = policy::rank_candidates(&processes, &deltas, &tracker);
    let plan = optimizer::dry_run(assessment, candidates, top);

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&plan).map_err(|error| error.to_string())?
        );
        return Ok(());
    }

    println!("Pressure: {} ({:.1}/100)", plan.pressure.level, plan.pressure.smoothed_score);
    println!("Plan: {:?}", plan.action);
    println!("Reason: {}", plan.reason);
    println!("System mutation: NO (v0.2 safety gate)");

    for candidate in plan.candidates {
        println!(
            "  PID {:>6} {:<28} score {:>5.1} expected <= {:>7.0} MiB",
            candidate.pid,
            truncate(&candidate.image_name, 28),
            candidate.score,
            mib(candidate.expected_reclaim_bytes),
        );
    }

    Ok(())
}

fn parse_usize_option(args: &[String], flag: &str, default: usize) -> Result<usize, String> {
    match option_value(args, flag)? {
        Some(value) => value
            .parse::<usize>()
            .map_err(|_| format!("{flag} expects a positive integer")),
        None => Ok(default),
    }
}

fn parse_u64_option(args: &[String], flag: &str, default: u64) -> Result<u64, String> {
    match option_value(args, flag)? {
        Some(value) => value
            .parse::<u64>()
            .map_err(|_| format!("{flag} expects a positive integer")),
        None => Ok(default),
    }
}

fn option_value<'a>(args: &'a [String], flag: &str) -> Result<Option<&'a str>, String> {
    let Some(index) = args.iter().position(|arg| arg == flag) else {
        return Ok(None);
    };

    args.get(index + 1)
        .map(String::as_str)
        .map(Some)
        .ok_or_else(|| format!("{flag} requires a value"))
}

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|arg| arg == flag)
}

fn reject_unknown_flags(args: &[String], boolean_flags: &[&str]) -> Result<(), String> {
    reject_unknown_flags_with_values(args, boolean_flags, &[])
}

fn reject_unknown_flags_with_values(
    args: &[String],
    boolean_flags: &[&str],
    value_flags: &[&str],
) -> Result<(), String> {
    let mut index = 0usize;
    while index < args.len() {
        let arg = &args[index];
        if boolean_flags.contains(&arg.as_str()) {
            index += 1;
            continue;
        }
        if value_flags.contains(&arg.as_str()) {
            if index + 1 >= args.len() {
                return Err(format!("{arg} requires a value"));
            }
            index += 2;
            continue;
        }
        return Err(format!("unknown option: {arg}"));
    }
    Ok(())
}

fn gib(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0 * 1024.0)
}

fn mib(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0)
}

fn truncate(value: &str, max_chars: usize) -> String {
    let mut output = value.chars().take(max_chars).collect::<String>();
    if value.chars().count() > max_chars {
        output.push('…');
    }
    output
}
