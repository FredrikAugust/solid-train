use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CsvEntry {
    pub timestamp: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Deserialize)]
pub struct Position {
    pub sogKmph: f64,
    pub txAt: String,
    pub gpsAtMillis: i64,
    pub altitude: f64,
    pub dtfNm: f64,
    pub latitude: f64,
    pub battery: i16,
    pub dtfKm: f64,
    pub sogKnots: f64,
    pub alert: bool,
    pub cog: i32,
    pub id: i32,
    pub gpsAt: String,
    pub longitude: f64,
}

#[derive(Debug, Deserialize)]
pub struct PositionWithWind {
    pub sogKmph: f64,
    pub txAt: String,
    pub gpsAtMillis: i64,
    pub altitude: f64,
    pub dtfNm: f64,
    pub latitude: f64,
    pub battery: i16,
    pub dtfKm: f64,
    pub sogKnots: f64,
    pub alert: bool,
    pub cog: i32,
    pub id: i32,
    pub gpsAt: String,
    pub longitude: f64,
    pub wind_speed_in_kmph: f64,
    pub wind_direction_in_deg: f64,
}

#[derive(Debug, Deserialize)]
pub struct Team {
    pub name: String,
    pub serial: i32,
    pub marker: i32,
    pub positions: Vec<Position>,
}

#[derive(Debug, Deserialize)]
pub struct TeamWithWind {
    pub name: String,
    pub serial: i32,
    pub marker: i32,
    pub positions: Vec<PositionWithWind>,
}

#[derive(Debug, Deserialize)]
pub struct FullDataJson {
    pub teams: Vec<Team>,
}

#[derive(Debug, Deserialize)]
pub struct FullDataJsonWithWind {
    pub teams: Vec<TeamWithWind>,
}
