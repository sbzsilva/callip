use clap::{Arg, Command};
use reqwest::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = Command::new("callip")
        .version("0.1.0")
        .about("Gets the public IP address of the host")
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .help("Prints help information")
                .action(clap::ArgAction::Help),
        )
        .get_matches();

    if matches.contains_id("help") {
        return Ok(());
    }

    let ip_address = get_public_ip().await?;
    println!("Your public IP address is: {}", ip_address);
    Ok(())
}

async fn get_public_ip() -> Result<String, Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.ipify.org?format=json")
        .send()
        .await?;

    let ip_data = response.json::<serde_json::Value>().await?;
    let ip_address = ip_data["ip"].as_str().unwrap_or("Unknown IP");

    Ok(ip_address.to_string())
}