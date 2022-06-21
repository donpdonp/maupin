use serde::{Deserialize, Serialize};
use std::fmt;

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
    pub page_size: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Value {
    pub value: String,
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = vec![];

        parts.push(self.address.to_owned());
        match &self.health {
            Some(h) => parts.push(h.value[..6].to_owned()),
            None => (),
        }
        match &self.total_borrow_value_in_eth {
            Some(borrow) => {
                let m = format!("bow: {}", borrow.value[..6].to_string());
                parts.push(m)
            }
            None => (),
        }
        match &self.total_collateral_value_in_eth {
            Some(collateral) => {
                let m = format!("col: {}", collateral.value[..6].to_string());
                parts.push(m)
            }
            None => (),
        }
        write!(f, "{}", parts.join(" "))
    }
}
