use csv::ReaderBuilder;
use reqwest::blocking::get;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    // Create directory if it doesn't exist
    if let Some(parent) = Path::new(file_path).parent() {
        fs::create_dir_all(parent)?;
    }

    // Download file
    let response = get(url)?;
    let mut file = File::create(file_path)?;
    file.write_all(&response.bytes()?)?;

    Ok(())
}

pub fn calculate_means(file_path: &str) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    let mut means = HashMap::new();
    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_path(file_path)?;

    // Get headers
    let headers = rdr.headers()?.clone();

    // Create accumulators for each column
    let mut sums: HashMap<String, f64> = HashMap::new();
    let mut counts: HashMap<String, i32> = HashMap::new();

    // Process each record
    for result in rdr.records() {
        let record = result?;
        for (i, field) in record.iter().enumerate() {
            let header = headers[i].to_string();
            if header == "age" {
                continue;
            }

            if let Ok(value) = field.replace("-", "0").parse::<f64>() {
                *sums.entry(header.clone()).or_insert(0.0) += value;
                *counts.entry(header).or_insert(0) += 1;
            }
        }
    }

    // Calculate means
    for (column, sum) in sums {
        if let Some(&count) = counts.get(&column) {
            if count > 0 {
                means.insert(column, sum / count as f64);
            }
        }
    }

    Ok(means)
}

pub fn validate_file(file_path: &str) -> Result<bool, Box<dyn Error>> {
    let path = Path::new(file_path);
    Ok(path.exists() && path.metadata()?.len() > 0)
}
