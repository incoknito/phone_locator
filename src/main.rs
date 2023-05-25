use serde_json::Value;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("./src/location.json")?;

    let json: Value = serde_json::from_str(&contents)?;

    if let (Some(latitude), Some(longitude)) = (
        json.get("latitude").and_then(Value::as_f64),
        json.get("longitude").and_then(Value::as_f64),
    ) {
        println!("Latitude: {}", latitude);
        println!("Longitude: {}", longitude);
    } else {
        println!("Location coordinates not available");
    }

    Ok(())
}