use chrono::Local;
use drug_use_sqlite::{delete_row, extract, insert_row, load_transform, select_rows, update_row};
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    // Initialize log file with header
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("db_operations.md")
        .expect("Failed to create log file");

    let header = format!(
        "# Database Operations Log\nStarted on: {}\n\n",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );
    file.write_all(header.as_bytes())
        .expect("Failed to write header");

    // Extract the CSV file from the URL and save it to a local path
    let url = "https://raw.githubusercontent.com/fivethirtyeight/data/master/drug-use-by-age/drug-use-by-age.csv";
    let file_path = "data/drug-use-by-age.csv";

    println!("Starting data extraction...");
    if let Err(e) = extract(url, file_path) {
        eprintln!("Error in extract: {}", e);
        return;
    }

    println!("Loading and transforming data into database...");
    if let Err(e) = load_transform(file_path) {
        eprintln!("Error in load: {}", e);
        return;
    }

    println!("Selecting and displaying all rows...");
    if let Err(e) = select_rows() {
        eprintln!("Error in select_rows: {}", e);
        return;
    }

    println!("Inserting new test data...");
    let new_row = (
        String::from("75+"), // age
        1000,                // n
        50.0,                // alcohol_use
        40.0,                // alcohol_frequency
        30.0,                // marijuana_use
        20.0,                // marijuana_frequency
        Some(5.0),           // cocaine_use
        Some(1.0),           // cocaine_frequency
        Some(0.0),           // crack_use
        Some(0.0),           // crack_frequency
        Some(0.0),           // heroin_use
        Some(0.0),           // heroin_frequency
        10.0,                // hallucinogen_use
        5.0,                 // hallucinogen_frequency
        3.0,                 // inhalant_use
        Some(2.0),           // inhalant_frequency
        5.0,                 // pain_releiver_use
        3.0,                 // pain_releiver_frequency
        0.5,                 // oxycontin_use
        Some(0.5),           // oxycontin_frequency
        1.0,                 // tranquilizer_use
        1.0,                 // tranquilizer_frequency
        2.0,                 // stimulant_use
        1.0,                 // stimulant_frequency
        Some(0.0),           // meth_use
        Some(0.0),           // meth_frequency
        0.5,                 // sedative_use
        0.3,                 // sedative_frequency
    );

    if let Err(e) = insert_row(new_row) {
        eprintln!("Error in insert_row: {}", e);
        return;
    }

    println!("Updating existing data...");
    let updates = vec![
        ("alcohol_use", &70.0 as &dyn rusqlite::ToSql),
        ("marijuana_use", &15.0 as &dyn rusqlite::ToSql),
    ];
    if let Err(e) = update_row("30-34", updates) {
        eprintln!("Error in update_row: {}", e);
        return;
    }

    println!("Deleting test data...");
    if let Err(e) = delete_row("26-29") {
        eprintln!("Error in delete_row: {}", e);
        return;
    }

    println!("\nFinal state of database after all operations:");
    if let Err(e) = select_rows() {
        eprintln!("Error in final select_rows: {}", e);
        return;
    }

    println!("All operations completed successfully!");
}
