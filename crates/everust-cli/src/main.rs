use clap::{Parser, Subcommand};
use everust_analyzer::{Analyzer, AnalyzerConfig};
use everust_core::finding::FindingSeverity;
use everust_recorder::read_recording;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "everust")]
#[command(about = "Everust: Visual systems flight recorder and investigation workbench for Rust", long_about = None)]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Inspect an .evr flight recording and run deterministic analysis
    Inspect {
        /// Path to the .evr recording file
        #[arg(value_name = "FILE")]
        path: PathBuf,
    },
    /// Run a built-in demo workload to generate a reference flight recording
    Demo {
        /// Name of workload demo (e.g. "lock-contention")
        #[arg(default_value = "lock-contention")]
        workload: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Inspect { path } => {
            println!("🔍 Inspecting Everust Flight Recording: {}\n", path.display());

            let recording = read_recording(&path)?;

            println!("--- HEADER METADATA ---");
            println!("  Magic:        {}", recording.header.magic);
            println!("  Format Ver:   {}", recording.header.version);
            println!("  Service:      {}", recording.header.service_name);
            println!("  PID:          {}", recording.header.pid);
            println!("  Total Events: {}", recording.header.total_events);
            println!(
                "  Timespan:     {} ns -> {} ns",
                recording.header.start_timestamp_ns, recording.header.end_timestamp_ns
            );
            println!();

            println!("--- DETERMINISTIC ANALYZER FINDINGS ---");
            let analyzer = Analyzer::new(AnalyzerConfig::default());
            let report = analyzer.analyze(&recording);

            if report.findings.is_empty() {
                println!("  ✅ No anomalies or stalls detected in this recording.");
            } else {
                println!(
                    "  Found {} incident finding(s) across {} events:\n",
                    report.findings.len(),
                    report.total_events
                );

                for (idx, f) in report.findings.iter().enumerate() {
                    let badge = match f.severity {
                        FindingSeverity::Critical => "🔴 [CRITICAL]",
                        FindingSeverity::Warning => "🟡 [WARNING]",
                        FindingSeverity::Info => "🔵 [INFO]",
                    };

                    println!("  {}. {} {}: {}", idx + 1, badge, f.code, f.title);
                    println!("     {}", f.description);
                    if let Some(dur) = f.duration_ns {
                        println!("     Duration: {:.2} ms", dur as f64 / 1_000_000.0);
                    }
                    if let Some(tid) = f.task_id {
                        println!("     Target Task ID: #{}", tid);
                    }
                    println!();
                }
            }

            println!("Analysis complete.");
        }

        Commands::Demo { workload } => {
            println!("🚀 Executing Everust Demo Workload: {}", workload);
            println!("Run: cargo run -p lock-contention to generate a live .evr recording.");
        }
    }

    Ok(())
}
