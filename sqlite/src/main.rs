use csv::ReaderBuilder;
use rusqlite::{Connection, Result};
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use sysinfo::{ProcessExt, System, SystemExt};

fn load_data_in_memory(
    file_path: &str,
    age_threshold: u32,
) -> Result<f64, Box<dyn std::error::Error>> {
    let mut reader = ReaderBuilder::new()
        .buffer_capacity(1024)
        .has_headers(true)
        .from_reader(File::open(file_path)?);

    let mut age_sum = 0u32;
    let mut count = 0u32;

    for record in reader.records() {
        let record = record?;
        if let Ok(age) = record[1].parse::<u32>() {
            if age > age_threshold {
                age_sum += age;
                count += 1;
            }
        }
    }

    let average_age = if count > 0 {
        age_sum as f64 / count as f64
    } else {
        0.0
    };

    println!("Loaded data and calculated average age: {:.2}", average_age); // Print statement
    Ok(average_age)
}

fn generate_performance_report(
    exec_time: f64,
    memory_usage_kb: u64,
    average_age: f64,
) -> std::io::Result<()> {
    let mut file = File::create("../rust_performance_report.md")?;
    writeln!(file, "# Rust Performance Report")?;
    writeln!(file, "| Metric            | Rust                  |")?;
    writeln!(file, "|-------------------|-----------------------|")?;
    writeln!(
        file,
        "| Execution Time    | {:.2} ms               |",
        exec_time
    )?;
    writeln!(
        file,
        "| Memory Usage      | {} KB                |",
        memory_usage_kb
    )?;
    writeln!(
        file,
        "| Average Age       | {:.2}                 |",
        average_age
    )?;
    println!("Performance report generated at rust_performance_report.md"); // Print statement
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "../data/titanic.csv"; // Correct relative path
    let age_threshold = 30;

    // Get current process information for memory usage
    let mut sys = System::new_all();
    let pid = sysinfo::get_current_pid().unwrap();
    sys.refresh_process(pid);
    let memory_before = sys.process(pid).unwrap().memory();

    let start = Instant::now();

    // Call the function to load data and calculate average age
    let average_age = load_data_in_memory(file_path, age_threshold)?;

    let exec_time = start.elapsed().as_secs_f64() * 1000.0;

    // Get memory usage after data processing
    sys.refresh_process(pid);
    let memory_after = sys.process(pid).unwrap().memory();
    let memory_usage_kb = if memory_after > memory_before {
        memory_after - memory_before
    } else {
        0
    };

    // Generate performance report
    generate_performance_report(exec_time, memory_usage_kb, average_age)?;

    Ok(())
}
