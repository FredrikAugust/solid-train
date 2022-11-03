use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse<TData> {
    pub data: TData,
}

#[derive(Debug, Deserialize)]
pub struct Budget {
    pub id: String,
    pub name: String,
    pub last_modified_on: String,
    pub first_month: String,
    pub last_month: String,
}

#[derive(Debug, Deserialize)]
pub struct BudgetApiResponse {
    pub budgets: Vec<Budget>,
}

#[derive(Debug, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub budgeted: i32,
    pub balance: i32,
    pub activity: i32,
}

#[derive(Debug, Deserialize)]
pub struct CategoryGroup {
    pub id: String,
    pub name: String,
    pub categories: Vec<Category>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionApiResponse {
    pub category_groups: Vec<CategoryGroup>,
}

#[derive(Debug, Deserialize)]
pub struct PayeeGeolocation {
    pub id: String,
    pub payee_id: String,
    pub latitude: String,
    pub longitude: String,
}

#[derive(Debug, Deserialize)]
pub struct PayeeGeolocationApiResponse {
    pub payee_locations: Vec<PayeeGeolocation>,
}

#[derive(Debug, Deserialize)]
pub struct Payee {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct PayeeApiResponse {
    pub payees: Vec<Payee>,
}
