#![doc = include_str!("../README.md")]

mod schema;

use std::error::Error;
use std::fs;

use schema::TakeoutData;

/// Only read the json file for now.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let takeout_json_path = "Tasks.json";
    let json_content = fs::read_to_string(takeout_json_path)
        .map_err(|e| format!("Failed to read {}: {}", takeout_json_path, e))?;

    // Parse as TakeoutData
    let takeout_data_root: TakeoutData = serde_json::from_str(&json_content).map_err(|e| {
        format!(
            "Failed to parse JSON data: {}. Check TakeoutData/TakeoutTaskList/TakeoutTask struct definitions.",
            e
        )
    })?;

    println!("{takeout_data_root:#?}");

    Ok(())
}
