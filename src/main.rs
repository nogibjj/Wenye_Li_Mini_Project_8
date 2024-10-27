use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;
use sysinfo::{ProcessExt, System, SystemExt};

fn get_memory_usage() -> f64 {
    let mut sys = System::new_all();
    sys.refresh_all();

    let pid = sysinfo::get_current_pid().expect("Failed to get PID");
    if let Some(process) = sys.process(pid) {
        process.memory() as f64 / 1024.0 / 1024.0 // Convert to MB
    } else {
        0.0
    }
}

fn main() {
    // Initialize performance metrics
    let initial_memory = get_memory_usage();
    let total_start = Instant::now();

    // URLs and paths
    let url = "https://raw.githubusercontent.com/fivethirtyeight/data/master/drug-use-by-age/drug-use-by-age.csv";
    let file_path = "data/drug-use-by-age.csv";

    // Extract data
    println!("Extracting data...");
    let extract_start = Instant::now();
    if let Err(e) = drug_use_analysis::extract(url, file_path) {
        eprintln!("Error extracting data: {}", e);
        return;
    }
    let extract_duration = extract_start.elapsed().as_secs_f64();

    // Calculate means
    println!("Calculating means...");
    let calc_start = Instant::now();
    let means = match drug_use_analysis::calculate_means(file_path) {
        Ok(means) => means,
        Err(e) => {
            eprintln!("Error calculating means: {}", e);
            return;
        }
    };
    let calc_duration = calc_start.elapsed().as_secs_f64();

    // Create performance report
    let mut report = String::from("# Rust Performance Report\n\n");

    // Add total execution time
    let total_duration = total_start.elapsed().as_secs_f64();
    report.push_str(&format!(
        "Total Execution Time: {:.6} seconds\n\n",
        total_duration
    ));

    // Add means to report
    report.push_str("## Column Means\n\n");
    for (column, mean) in &means {
        report.push_str(&format!("- {}: {:.2}\n", column, mean));
    }

    // Add performance metrics
    report.push_str("\n## Performance Metrics\n\n");
    report.push_str("| Operation | Time (s) | Memory (MB) |\n");
    report.push_str("|-----------|----------|-------------|\n");
    report.push_str(&format!(
        "| Extract | {:.6} | {:.2} |\n",
        extract_duration,
        get_memory_usage() - initial_memory
    ));
    report.push_str(&format!(
        "| Calculate Means | {:.6} | {:.2} |\n",
        calc_duration,
        get_memory_usage() - initial_memory
    ));

    // Write report to file
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("Rust_Performance.md")
        .expect("Failed to create report file");

    file.write_all(report.as_bytes())
        .expect("Failed to write report");

    println!("Performance results have been written to Rust_Performance.md");
}
