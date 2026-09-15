use std::{env, process::ExitCode};

use mempilot::{NAME, VERSION};

fn print_help() {
    println!("{NAME} {VERSION}");
    println!("Adaptive memory pressure manager for Windows\n");
    println!("USAGE:");
    println!("  mempilot <COMMAND>\n");
    println!("COMMANDS:");
    println!("  status    Show current implementation status");
    println!("  version   Show version");
    println!("  help      Show this help");
}

fn main() -> ExitCode {
    match env::args().nth(1).as_deref() {
        None | Some("help") | Some("--help") | Some("-h") => {
            print_help();
            ExitCode::SUCCESS
        }
        Some("version") | Some("--version") | Some("-V") => {
            println!("{NAME} {VERSION}");
            ExitCode::SUCCESS
        }
        Some("status") => {
            println!("{NAME} {VERSION} (pre-alpha)");
            if cfg!(target_os = "windows") {
                println!("Platform: Windows");
                println!("Live telemetry backend: not wired yet");
                println!("Optimizer actions: disabled");
            } else {
                println!("Platform: unsupported for live optimization");
                println!("Pure pressure/policy logic remains testable.");
            }
            ExitCode::SUCCESS
        }
        Some(other) => {
            eprintln!("Unknown command: {other}\n");
            print_help();
            ExitCode::from(2)
        }
    }
}
