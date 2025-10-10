use anyhow::{Context, Result, bail};
use clap::Parser;
use serde_json::{Map, Value};
use std::fs;
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(name = "jgetset")]
#[command(about = "Get or set values in a JSON file", long_about = None)]
struct Cli {
    /// Path to the JSON file
    file: PathBuf,

    /// Key to get, or key=value to set
    operation: String,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    let file_path = cli.file.to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?;

    // Determine if this is a GET or SET operation
    if let Some(equals_pos) = cli.operation.find('=') {
        // SET operation: key=value
        let key = &cli.operation[..equals_pos];
        let value = &cli.operation[equals_pos + 1..];
        set_value(file_path, key, value)?;
    } else {
        // GET operation: key
        let key = &cli.operation;
        get_value(file_path, key)?;
    }

    Ok(())
}

fn load_json_object(file_path: &str) -> Result<Map<String, Value>> {
    let path = std::path::Path::new(file_path);

    if !path.exists() {
        // File doesn't exist, return empty object
        return Ok(Map::new());
    }

    // Read the file
    let content =
        fs::read_to_string(file_path).context(format!("Failed to read file: {}", file_path))?;

    // Parse as JSON
    let json_value: Value = serde_json::from_str(&content)
        .context(format!("File contains invalid JSON: {}", file_path))?;

    // Ensure it's an object (dictionary)
    match json_value {
        Value::Object(obj) => Ok(obj),
        _ => bail!(
            "File does not contain a JSON object (dictionary): {}",
            file_path
        ),
    }
}

fn save_json_object(file_path: &str, obj: &Map<String, Value>) -> Result<()> {
    let json_string = serde_json::to_string_pretty(obj).context("Failed to serialize JSON")?;

    fs::write(file_path, json_string).context(format!("Failed to write to file: {}", file_path))?;

    Ok(())
}

fn get_value(file_path: &str, key: &str) -> Result<()> {
    let obj = load_json_object(file_path)?;

    match obj.get(key) {
        Some(value) => {
            // Print the value in JSON format
            println!("{}", serde_json::to_string(value)?);
            Ok(())
        }
        None => bail!("Key not found: {}", key),
    }
}

fn set_value(file_path: &str, key: &str, value_str: &str) -> Result<()> {
    let mut obj = load_json_object(file_path)?;

    // Store the value as a JSON string
    obj.insert(key.to_string(), Value::String(value_str.to_string()));

    save_json_object(file_path, &obj)?;

    Ok(())
}
