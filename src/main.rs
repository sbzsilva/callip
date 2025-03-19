use clap::Command;
use reqwest::Client;
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _matches = Command::new("callip")
        .version("0.1.0")
        .about("Gets the public IP address of the host")
        .get_matches();

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let ip_address = get_public_ip(&client).await?;
    println!("Your public IP address is: {}", ip_address);
    Ok(())
}

async fn get_public_ip(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    let response = client
        .get("https://api.ipify.org?format=json")
        .send()
        .await?;

    let ip_data = response.json::<serde_json::Value>().await?;
    let ip_address = ip_data["ip"]
        .as_str()
        .ok_or_else(|| "Invalid IP format in response".to_string())?;

    Ok(ip_address.to_string())
}