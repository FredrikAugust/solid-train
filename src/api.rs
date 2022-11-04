use std::future::Future;

use reqwest::Response;
use serde::Deserialize;

const API_KEY: &str = "";
const BASE_URL: &str = "https://weather.visualcrossing.com/VisualCrossingWebServices/rest";

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Entry {
    pub temp: f64,
    pub windspeed: f64,
    pub winddir: f64,
}

#[derive(Debug, Deserialize)]
pub struct ApiResult {
    pub days: Vec<Entry>,
}

pub async fn get_wind_data(csv_entry: &crate::types::CsvEntry) -> Result<Entry, reqwest::Error> {
    let url = format!(
        "{}/services/timeline/{},{}/{}?unitGroup=metric&key={}&contentType=json&include=current&unitGroup=metric",
        BASE_URL, csv_entry.lat, csv_entry.lon, csv_entry.timestamp, API_KEY
    );

    let response = reqwest::get(url).await?.json::<ApiResult>().await?;

    return Ok(response
        .days
        .first()
        .expect("Could not find first day")
        .clone());
}
