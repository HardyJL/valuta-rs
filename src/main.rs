use clap::Parser;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct FileStorage {
    conversion_rate: f64,
    time_last_update_unix: i64,
    from: String,
    to: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Conversion {
    conversion_rate: f64,
    time_last_update_unix: i64,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    amount: f64,

    #[arg(short, long, default_value_t = String::from("DKK"))]
    from: String,

    #[arg(short, long, default_value_t = String::from("EUR"))]
    to: String,
}

const BASE_URL: &str = "https://v6.exchangerate-api.com/v6";

fn main() -> Result<()> {
    let args = Args::parse();
    let api_key = dotenv::var("API_KEY")?;

    let amount = args.amount;
    let from = args.from;
    let to = args.to;

    let storage = fetch_conversion_rates(from, to.clone(), api_key)?;
    let final_amount = storage.conversion_rate * amount;

    println!("{:.2} {}", final_amount, to);
    Ok(())
}

fn fetch_conversion_rates(from: String, to: String, api_key: String) -> Result<FileStorage> {
    let url = format!("{}/{}/pair/{}/{}", BASE_URL, api_key, from, to);
    let body = reqwest::blocking::get(url)?.text();

    let payload: Conversion = serde_json::from_str(&body?)?;

    Ok(FileStorage {
        to,
        from,
        conversion_rate: payload.conversion_rate,
        time_last_update_unix: payload.time_last_update_unix,
    })
}
