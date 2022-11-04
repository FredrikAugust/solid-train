use std::{fs::File, io::BufReader};

pub fn get_json_data() -> Result<crate::types::FullDataJson, Box<dyn std::error::Error>> {
    let file = File::open("src/allteams.json")?;
    let reader = BufReader::new(file);

    let data: crate::types::FullDataJson = serde_json::from_reader(reader)?;

    Ok(data)
}
