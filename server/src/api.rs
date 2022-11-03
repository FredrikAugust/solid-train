use std::future::Future;

use reqwest::Response;

use crate::types::{
    ApiResponse, Budget, BudgetApiResponse, Category, Payee, PayeeApiResponse, PayeeGeolocation,
    PayeeGeolocationApiResponse, TransactionApiResponse,
};

const API_KEY: &str = "";
const BASE_URL: &str = "https://api.youneedabudget.com/v1";

fn get(path: &str) -> impl Future<Output = Result<Response, reqwest::Error>> {
    let client = reqwest::Client::new();

    client
        .get(format!("{}{}", BASE_URL, path))
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", API_KEY),
        )
        .send()
}

pub async fn get_budgets() -> Result<Vec<Budget>, Box<dyn std::error::Error>> {
    let budgets = get("/budgets")
        .await?
        .json::<ApiResponse<BudgetApiResponse>>()
        .await?;

    return Ok(budgets.data.budgets);
}

pub async fn get_categories(budget_id: &str) -> Result<Vec<Category>, Box<dyn std::error::Error>> {
    let category_groups = get(&format!("/budgets/{}/categories", budget_id))
        .await?
        .json::<ApiResponse<TransactionApiResponse>>()
        .await?;

    let categories = category_groups
        .data
        .category_groups
        .into_iter()
        .flat_map(|category_group| category_group.categories)
        .collect();

    return Ok(categories);
}

pub async fn get_all_geolocations(
    budget_id: &str,
) -> Result<Vec<PayeeGeolocation>, Box<dyn std::error::Error>> {
    let payee_geolocations = get(&format!("/budgets/{}/payee_locations", budget_id))
        .await?
        .json::<ApiResponse<PayeeGeolocationApiResponse>>()
        .await?;

    return Ok(payee_geolocations.data.payee_locations);
}

pub async fn get_payees(budget_id: &str) -> Result<Vec<Payee>, Box<dyn std::error::Error>> {
    let payees = get(&format!("/budgets/{}/payees", budget_id))
        .await?
        .json::<ApiResponse<PayeeApiResponse>>()
        .await?;

    return Ok(payees.data.payees);
}
