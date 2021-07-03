use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::{env, vec::Vec};

#[derive(Deserialize, Debug)]
struct CityData {
    name: String,
}

#[derive(Deserialize, Debug)]
struct AQIData {
    aqi: u32,
    city: CityData,
}

#[derive(Deserialize, Debug)]
struct APIData {
    data: AQIData,
}

fn fetch_aqi(token: String, city: &str) -> Result<APIData> {
    let url = format!(
        "http://api.waqi.info/feed/{}/?token={}",
        city.to_string(),
        token,
    );
    let resp = Client::new().get(&url).send()?;

    match resp.json::<APIData>() {
        Ok(d) => Ok(d),
        Err(_) => Err(anyhow!("Failed to process the data")),
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let city = args
        .get(1)
        .ok_or_else(|| anyhow!("Please enter a city name"))?;
    let fetched = fetch_aqi(env::var("TOKEN")?, city)?;

    println!(
        "{}: {} ({})",
        fetched.data.city.name,
        fetched.data.aqi,
        match fetched.data.aqi {
            0..=50 => "优",
            51..=100 => "良",
            101..=150 => "轻度污染",
            151..=200 => "中度污染",
            201..=300 => "重度污染",
            _ => "严重污染",
        },
    );
    Ok(())
}
