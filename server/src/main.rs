mod api;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let budgets = api::get_budgets().await?;
    let payees = api::get_payees(&budgets.first().expect("No budget found").id).await?;

    let geolocations = api::get_all_geolocations(&budgets.first().unwrap().id).await?;

    // Print CSV for all geolocations I have saved with the associated payee
    for geolocation in geolocations {
        let payee_option = payees.iter().find(|payee| payee.id == geolocation.payee_id);

        let name = match payee_option {
            Some(payee) => &payee.name,
            None => "<unknown>",
        };

        println!(
            "{},{},{},{}",
            name,
            format!("Lagret posisjon for {}", name),
            geolocation.latitude,
            geolocation.longitude
        )
    }

    Ok(())
}
