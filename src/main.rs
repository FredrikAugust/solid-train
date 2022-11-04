use api::get_wind_data;

use crate::{csv::read_file_to_entries, json::get_json_data};

mod api;
mod csv;
mod json;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let entries = read_file_to_entries();

    println!("timestamp,lat,lon,windspeed,winddir");

    // for entry in entries {
    //     let wind_data = get_wind_data(&entry).await?;
    //     println!(
    //         "{},{},{},{},{}",
    //         entry.timestamp, entry.lat, entry.lon, wind_data.windspeed, wind_data.winddir
    //     );
    // }

    let full_json_data = get_json_data().unwrap();

    let mut full_json_data_with_wind = crate::types::FullDataJsonWithWind { teams: vec![] };

    for team in full_json_data.teams {
        println!("{}", team.name)
    }

    Ok(())
}
