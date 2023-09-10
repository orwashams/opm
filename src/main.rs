use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error as StdError};

// Define a struct to deserialize the JSON data into
#[derive(Debug, Deserialize, Serialize)]
struct NpmPackageInfo {
    name: String,
    repository: HashMap<String, String>,
    versions: serde_json::Map<String, serde_json::Value>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    // Define the URL you want to fetch JSON data from
    let url = "https://registry.npmjs.org/react";

    // Send an HTTP GET request using Reqwest
    let response = reqwest::get(url).await?;

    let data: serde_json::Value = response.json().await?;

    println!("{:?}", data);

    // Check if the request was successful (status code 200 OK)
    Ok(())
}
