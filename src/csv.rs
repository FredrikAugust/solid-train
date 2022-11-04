pub fn read_file_to_entries() -> Vec<crate::types::CsvEntry> {
    let reader = csv::Reader::from_path("src/vasket_2021.csv");
    let mut binding = reader.expect("Could not deserialize reader");
    let entries = binding
        .deserialize::<crate::types::CsvEntry>()
        .map(|entry| entry.expect("Could not deserialize entry"));

    entries.collect()
}
