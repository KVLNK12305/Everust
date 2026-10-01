use everust_agent::{ContendedMutex, EverustAgent};
use everust_analyzer::{Analyzer, AnalyzerConfig};
use everust_core::event::EventType;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Everust Workload: Lock Contention & Starvation ===");
    println!("Initializing in-process flight recorder agent...");

    let agent = EverustAgent::init("lock-contention-service", 20_000);

    let shared_state = Arc::new(ContendedMutex::new(0u64));
    let num_workers = 4;
    let iterations_per_worker = 15;

    let mut handles = Vec::new();

    println!(
        "Spawning {} worker tasks competing for shared ContendedMutex...",
        num_workers
    );

    for worker_id in 0..num_workers {
        let lock = Arc::clone(&shared_state);
        let task_id = 100 + worker_id;

        agent.emit(EventType::TaskSpawn {
            task_id,
            name: Some(format!("worker-task-{}", worker_id)),
        });

        let handle = tokio::spawn(async move {
            for iter in 0..iterations_per_worker {
                if let Some(agent) = EverustAgent::current() {
                    agent.emit(EventType::TaskWake { task_id });
                }

                // Simulate greedy thread holding the lock longer on worker 0
                let is_greedy = worker_id == 0 && iter % 3 == 0;

                let wait_sim = if is_greedy {
                    Duration::from_millis(6) // greedy lock hold
                } else {
                    Duration::from_micros(100)
                };

                // Acquire contended lock
                {
                    let mut guard = lock.lock(task_id);
                    *guard += 1;
                    std::thread::sleep(wait_sim);
                }

                // Simulate occasional synchronous blocking call on worker 3
                if worker_id == 3 && iter == 5 {
                    if let Some(agent) = EverustAgent::current() {
                        agent.record_blocking_op(
                            task_id,
                            "sync_file_read_on_worker",
                            8_500_000, // 8.5ms
                        );
                    }
                }

                tokio::time::sleep(Duration::from_millis(2)).await;
            }

            if let Some(agent) = EverustAgent::current() {
                agent.emit(EventType::TaskComplete {
                    task_id,
                    total_duration_ns: 50_000_000,
                });
            }
        });

        handles.push(handle);
    }

    for h in handles {
        h.await?;
    }

    let final_val = *shared_state.lock(999);
    println!("Workload complete. Total lock mutations: {}", final_val);

    // Snapshot flight recording to disk
    let output_file = "lock_contention.evr";
    println!("Snapshotting flight recording to {} ...", output_file);
    let recording = agent.snapshot(output_file)?;
    println!(
        "Recorded {} events across {} ns.",
        recording.header.total_events,
        recording.header.end_timestamp_ns - recording.header.start_timestamp_ns
    );

    // Run deterministic analyzer
    println!("\n--- Running Deterministic Incident Analyzer ---");
    let analyzer = Analyzer::new(AnalyzerConfig::default());
    let report = analyzer.analyze(&recording);

    println!(
        "Observed {} tasks, {} contested locks, and identified {} finding(s):",
        report.total_tasks_observed,
        report.contested_locks_count,
        report.findings.len()
    );

    for (i, f) in report.findings.iter().enumerate() {
        println!("  {}. [{:?}] {}: {}", i + 1, f.severity, f.code, f.title);
        println!("     {}", f.description);
    }

    println!("\nFlight recording is ready for Everust Workbench inspection!");
    Ok(())
}
