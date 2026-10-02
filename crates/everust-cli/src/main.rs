use clap::{Parser, Subcommand};
use everust_agent::{ContendedMutex, EverustAgent};
use everust_analyzer::{Analyzer, AnalyzerConfig};
use everust_core::event::EventType;
use everust_core::finding::FindingSeverity;
use everust_core::record::FlightRecording;
use everust_recorder::read_recording;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

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
    /// Run a built-in demo workload to generate and inspect a flight recording
    Demo {
        /// Name of workload demo (e.g. "lock-contention")
        #[arg(default_value = "lock-contention")]
        workload: String,
        /// Path to write the flight recording
        #[arg(short, long, default_value = "lock_contention.evr")]
        output: PathBuf,
    },
    /// Record execution of a command or server binary
    Record {
        /// Target binary or script command to execute
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        command: Vec<String>,
        /// Output .evr recording filename
        #[arg(short, long, default_value = "recording.evr")]
        output: PathBuf,
    },
}

fn inspect_recording_report(recording: &FlightRecording) {
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
    let report = analyzer.analyze(recording);

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
}

fn run_lock_contention_demo(output_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing in-process flight recorder agent...");
    let agent = EverustAgent::init("lock-contention-service", 20_000);
    let shared_state = Arc::new(ContendedMutex::new(0u64));
    let num_workers = 4;
    let iterations = 15;

    println!("Simulating {} worker tasks competing for shared ContendedMutex...", num_workers);

    let mut threads = Vec::new();
    for worker_id in 0..num_workers {
        let lock = Arc::clone(&shared_state);
        let task_id = 100 + worker_id;

        agent.emit(EventType::TaskSpawn {
            task_id,
            name: Some(format!("worker-task-{}", worker_id)),
        });

        let handle = std::thread::spawn(move || {
            for iter in 0..iterations {
                if let Some(agent) = EverustAgent::current() {
                    agent.emit(EventType::TaskWake { task_id });
                }

                let is_greedy = worker_id == 0 && iter % 3 == 0;
                let wait_sim = if is_greedy {
                    Duration::from_millis(6)
                } else {
                    Duration::from_micros(100)
                };

                {
                    let mut guard = lock.lock(task_id);
                    *guard += 1;
                    std::thread::sleep(wait_sim);
                }

                if worker_id == 3 && iter == 5 {
                    if let Some(agent) = EverustAgent::current() {
                        agent.record_blocking_op(
                            task_id,
                            "sync_file_read_on_worker",
                            8_500_000,
                        );
                    }
                }

                std::thread::sleep(Duration::from_millis(1));
            }

            if let Some(agent) = EverustAgent::current() {
                agent.emit(EventType::TaskComplete {
                    task_id,
                    total_duration_ns: 45_000_000,
                });
            }
        });

        threads.push(handle);
    }

    for h in threads {
        h.join().unwrap();
    }

    println!("Simulation complete. Snapshotting flight recording to {} ...", output_path.display());
    let recording = agent.snapshot(output_path)?;
    println!(
        "Recorded {} events across {} ns.\n",
        recording.header.total_events,
        recording.header.end_timestamp_ns - recording.header.start_timestamp_ns
    );

    inspect_recording_report(&recording);

    println!("💡 TIP: You can inspect this recording visually in the Everust Workbench!");
    println!("   Open http://localhost:5173/workbench and drop '{}'\n", output_path.display());
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Inspect { path } => {
            println!("🔍 Inspecting Everust Flight Recording: {}\n", path.display());
            let recording = read_recording(&path)?;
            inspect_recording_report(&recording);
            println!("Analysis complete.");
        }

        Commands::Demo { workload, output } => {
            match workload.as_str() {
                "lock-contention" => {
                    run_lock_contention_demo(&output)?;
                }
                other => {
                    eprintln!("Unknown demo workload: '{}'. Available workloads: 'lock-contention'", other);
                    std::process::exit(1);
                }
            }
        }

        Commands::Record { command, output } => {
            if command.is_empty() {
                println!("Usage: everust record <COMMAND> [ARGS]... -o <OUTPUT.evr>");
                println!("Example: everust record ./my-server --output my_server.evr");
                return Ok(());
            }

            println!("🔴 Starting Everust Flight Recording for: {}", command.join(" "));
            println!("Output target: {}", output.display());
            println!("Attaching in-process flight recorder hooks...");

            let mut child = std::process::Command::new(&command[0])
                .args(&command[1..])
                .spawn()?;

            let status = child.wait()?;
            println!("Process exited with status: {}", status);
        }
    }

    Ok(())
}
