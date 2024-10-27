use chrono::Local;
use csv::ReaderBuilder;
use reqwest::blocking::get;
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;

const DB_FILE: &str = "DrugUseDB.db";
const LOG_FILE: &str = "db_operations.md";

// Define a type alias for the complex drug data type
type DrugData = (
    String,
    i32,
    f64,
    f64,
    f64,
    f64,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    f64,
    f64,
    f64,
    Option<f64>,
    f64,
    f64,
    f64,
    Option<f64>,
    f64,
    f64,
    f64,
    f64,
    Option<f64>,
    Option<f64>,
    f64,
    f64,
);

// Helper function for logging
fn log_operation(operation: &str, details: &str) -> Result<(), Box<dyn Error>> {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_entry = format!(
        "### {}\n**Operation:** {}\n**Details:** {}\n\n",
        timestamp, operation, details
    );

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)?;

    file.write_all(log_entry.as_bytes())?;
    Ok(())
}

#[derive(Debug)]
struct DrugRow {
    age: String,
    n: i32,
    alcohol_use: f64,
    alcohol_frequency: f64,
    marijuana_use: f64,
    marijuana_frequency: f64,
    cocaine_use: Option<f64>,
    cocaine_frequency: Option<f64>,
    crack_use: Option<f64>,
    crack_frequency: Option<f64>,
    heroin_use: Option<f64>,
    heroin_frequency: Option<f64>,
    hallucinogen_use: f64,
    hallucinogen_frequency: f64,
    inhalant_use: f64,
    inhalant_frequency: Option<f64>,
    pain_releiver_use: f64,
    pain_releiver_frequency: f64,
    oxycontin_use: f64,
    oxycontin_frequency: Option<f64>,
    tranquilizer_use: f64,
    tranquilizer_frequency: f64,
    stimulant_use: f64,
    stimulant_frequency: f64,
    meth_use: Option<f64>,
    meth_frequency: Option<f64>,
    sedative_use: f64,
    sedative_frequency: f64,
}

impl From<DrugData> for DrugRow {
    fn from(data: DrugData) -> Self {
        DrugRow {
            age: data.0,
            n: data.1,
            alcohol_use: data.2,
            alcohol_frequency: data.3,
            marijuana_use: data.4,
            marijuana_frequency: data.5,
            cocaine_use: data.6,
            cocaine_frequency: data.7,
            crack_use: data.8,
            crack_frequency: data.9,
            heroin_use: data.10,
            heroin_frequency: data.11,
            hallucinogen_use: data.12,
            hallucinogen_frequency: data.13,
            inhalant_use: data.14,
            inhalant_frequency: data.15,
            pain_releiver_use: data.16,
            pain_releiver_frequency: data.17,
            oxycontin_use: data.18,
            oxycontin_frequency: data.19,
            tranquilizer_use: data.20,
            tranquilizer_frequency: data.21,
            stimulant_use: data.22,
            stimulant_frequency: data.23,
            meth_use: data.24,
            meth_frequency: data.25,
            sedative_use: data.26,
            sedative_frequency: data.27,
        }
    }
}

impl From<DrugRow> for DrugData {
    fn from(row: DrugRow) -> Self {
        (
            row.age,
            row.n,
            row.alcohol_use,
            row.alcohol_frequency,
            row.marijuana_use,
            row.marijuana_frequency,
            row.cocaine_use,
            row.cocaine_frequency,
            row.crack_use,
            row.crack_frequency,
            row.heroin_use,
            row.heroin_frequency,
            row.hallucinogen_use,
            row.hallucinogen_frequency,
            row.inhalant_use,
            row.inhalant_frequency,
            row.pain_releiver_use,
            row.pain_releiver_frequency,
            row.oxycontin_use,
            row.oxycontin_frequency,
            row.tranquilizer_use,
            row.tranquilizer_frequency,
            row.stimulant_use,
            row.stimulant_frequency,
            row.meth_use,
            row.meth_frequency,
            row.sedative_use,
            row.sedative_frequency,
        )
    }
}

pub fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    log_operation("Extract", &format!("Downloading file from {}", url))?;

    if let Some(parent) = Path::new(file_path).parent() {
        fs::create_dir_all(parent)?;
    }

    let response = get(url)?;
    if response.status().is_success() {
        let mut file = File::create(file_path)?;
        file.write_all(&response.bytes()?)?;
        log_operation(
            "Extract",
            &format!("File successfully downloaded to {}", file_path),
        )?;
    } else {
        let error_msg = format!("Failed to retrieve the file from {}", url);
        log_operation("Extract Error", &error_msg)?;
        return Err(error_msg.into());
    }

    Ok(())
}

pub fn load_transform(dataset: &str) -> Result<(), Box<dyn Error>> {
    log_operation(
        "Load and transform",
        &format!("Loading and transforming data from {}", dataset),
    )?;

    let mut conn = Connection::open(DB_FILE)?;
    conn.execute("DROP TABLE IF EXISTS DrugUse", [])?;

    conn.execute(
        "CREATE TABLE DrugUse (
            age TEXT,
            n INTEGER,
            alcohol_use REAL,
            alcohol_frequency REAL,
            marijuana_use REAL,
            marijuana_frequency REAL,
            cocaine_use REAL,
            cocaine_frequency REAL,
            crack_use REAL,
            crack_frequency REAL,
            heroin_use REAL,
            heroin_frequency REAL,
            hallucinogen_use REAL,
            hallucinogen_frequency REAL,
            inhalant_use REAL,
            inhalant_frequency REAL,
            pain_releiver_use REAL,
            pain_releiver_frequency REAL,
            oxycontin_use REAL,
            oxycontin_frequency REAL,
            tranquilizer_use REAL,
            tranquilizer_frequency REAL,
            stimulant_use REAL,
            stimulant_frequency REAL,
            meth_use REAL,
            meth_frequency REAL,
            sedative_use REAL,
            sedative_frequency REAL
        )",
        [],
    )?;

    let file = File::open(dataset)?;
    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_reader(file);

    let mut row_count = 0;
    let tx = conn.transaction()?;
    for result in rdr.records() {
        let record = result?;
        tx.execute(
            "INSERT INTO DrugUse VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                &record[0],
                record[1].parse::<i32>()?,
                record[2].parse::<f64>()?,
                record[3].parse::<f64>()?,
                record[4].parse::<f64>()?,
                record[5].parse::<f64>()?,
                parse_float(&record[6])?,
                parse_float(&record[7])?,
                parse_float(&record[8])?,
                parse_float(&record[9])?,
                parse_float(&record[10])?,
                parse_float(&record[11])?,
                record[12].parse::<f64>()?,
                record[13].parse::<f64>()?,
                record[14].parse::<f64>()?,
                parse_float(&record[15])?,
                record[16].parse::<f64>()?,
                record[17].parse::<f64>()?,
                record[18].parse::<f64>()?,
                parse_float(&record[19])?,
                record[20].parse::<f64>()?,
                record[21].parse::<f64>()?,
                record[22].parse::<f64>()?,
                record[23].parse::<f64>()?,
                parse_float(&record[24])?,
                parse_float(&record[25])?,
                record[26].parse::<f64>()?,
                record[27].parse::<f64>()?
            ],
        )?;
        row_count += 1;
    }
    tx.commit()?;

    log_operation(
        "Load and transform",
        &format!(
            "Successfully loaded and transformed {} rows into database",
            row_count
        ),
    )?;
    Ok(())
}

fn parse_float(value: &str) -> Result<Option<f64>, Box<dyn Error>> {
    Ok(if value == "-" {
        None
    } else {
        Some(value.parse::<f64>()?)
    })
}

pub fn insert_row(data: DrugData) -> Result<(), Box<dyn Error>> {
    log_operation(
        "Insert",
        &format!("Inserting new row for age group: {}", data.0),
    )?;
    let drug_row: DrugRow = data.into();

    let conn = Connection::open(DB_FILE)?;
    conn.execute(
        "INSERT INTO DrugUse VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            drug_row.age, drug_row.n, drug_row.alcohol_use, drug_row.alcohol_frequency,
            drug_row.marijuana_use, drug_row.marijuana_frequency, drug_row.cocaine_use,
            drug_row.cocaine_frequency, drug_row.crack_use, drug_row.crack_frequency,
            drug_row.heroin_use, drug_row.heroin_frequency, drug_row.hallucinogen_use,
            drug_row.hallucinogen_frequency, drug_row.inhalant_use, drug_row.inhalant_frequency,
            drug_row.pain_releiver_use, drug_row.pain_releiver_frequency, drug_row.oxycontin_use,
            drug_row.oxycontin_frequency, drug_row.tranquilizer_use, drug_row.tranquilizer_frequency,
            drug_row.stimulant_use, drug_row.stimulant_frequency, drug_row.meth_use,
            drug_row.meth_frequency, drug_row.sedative_use, drug_row.sedative_frequency
        ],
    )?;

    log_operation("Insert", "Row inserted successfully")?;
    Ok(())
}

pub fn select_rows() -> Result<(), Box<dyn Error>> {
    log_operation("Select", "Retrieving all rows from database")?;

    let conn = Connection::open(DB_FILE)?;
    let mut stmt = conn.prepare("SELECT * FROM DrugUse")?;

    let drug_iter = stmt.query_map([], |row| {
        Ok(DrugRow {
            age: row.get(0)?,
            n: row.get(1)?,
            alcohol_use: row.get(2)?,
            alcohol_frequency: row.get(3)?,
            marijuana_use: row.get(4)?,
            marijuana_frequency: row.get(5)?,
            cocaine_use: row.get(6)?,
            cocaine_frequency: row.get(7)?,
            crack_use: row.get(8)?,
            crack_frequency: row.get(9)?,
            heroin_use: row.get(10)?,
            heroin_frequency: row.get(11)?,
            hallucinogen_use: row.get(12)?,
            hallucinogen_frequency: row.get(13)?,
            inhalant_use: row.get(14)?,
            inhalant_frequency: row.get(15)?,
            pain_releiver_use: row.get(16)?,
            pain_releiver_frequency: row.get(17)?,
            oxycontin_use: row.get(18)?,
            oxycontin_frequency: row.get(19)?,
            tranquilizer_use: row.get(20)?,
            tranquilizer_frequency: row.get(21)?,
            stimulant_use: row.get(22)?,
            stimulant_frequency: row.get(23)?,
            meth_use: row.get(24)?,
            meth_frequency: row.get(25)?,
            sedative_use: row.get(26)?,
            sedative_frequency: row.get(27)?,
        })
    })?;

    let mut row_count = 0;
    for row in drug_iter {
        println!("{:?}", row?);
        row_count += 1;
    }

    log_operation("Select", &format!("Retrieved {} rows", row_count))?;
    Ok(())
}

pub fn update_row(
    age: &str,
    updates: Vec<(&str, &dyn rusqlite::ToSql)>,
) -> Result<(), Box<dyn Error>> {
    log_operation("Update", &format!("Updating row for age group: {}", age))?;

    let conn = Connection::open(DB_FILE)?;
    let set_clause = updates
        .iter()
        .map(|(col, _)| format!("{} = ?", col))
        .collect::<Vec<_>>()
        .join(", ");
    let query = format!("UPDATE DrugUse SET {} WHERE age = ?", set_clause);

    let mut params = updates
        .into_iter()
        .map(|(_, value)| value)
        .collect::<Vec<_>>();
    params.push(&age);

    let rows_affected = conn.execute(query.as_str(), params.as_slice())?;
    log_operation("Update", &format!("Updated {} rows", rows_affected))?;
    Ok(())
}

pub fn delete_row(age: &str) -> Result<(), Box<dyn Error>> {
    log_operation("Delete", &format!("Deleting row for age group: {}", age))?;

    let conn = Connection::open(DB_FILE)?;
    let rows_affected = conn.execute("DELETE FROM DrugUse WHERE age = ?", params![age])?;

    log_operation("Delete", &format!("Deleted {} rows", rows_affected))?;
    Ok(())
}
