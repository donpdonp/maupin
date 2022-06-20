use serde::{Deserialize, Serialize};

pub static ACCOUNTS_URL: &str = "https://api.compound.finance/api/v2/account";

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub accounts: Vec<Account>,
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub address: String,
    pub health: Option<Value>,
    pub total_borrow_value_in_eth: Option<Value>,
    pub total_collateral_value_in_eth: Option<Value>,
}

#[derive(Serialize, Deserialize)]
pub struct AccountOptions {
    pub max_health: Value,
    pub page_size: i32
}

#[derive(Serialize, Deserialize)]
pub struct Value {
    pub value: String,
}
