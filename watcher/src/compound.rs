use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

pub static ACCOUNTS_URL: &str = "https://api.compound.finance/api/v2/account";

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub accounts: Vec<Account>,
}

#[derive(Serialize, Deserialize)]
pub struct AccountOptions {
    pub max_health: Value,
    pub page_size: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub address: String,
    pub health: Option<Value>,
    pub total_borrow_value_in_eth: Option<Value>,
    pub total_collateral_value_in_eth: Option<Value>,
    pub tokens: Vec<Token>,
}

#[derive(Serialize, Deserialize)]
pub struct Value {
    pub value: String,
}

impl Value {
    fn value(&self) -> f64 {
        self.value.parse::<f64>().unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub symbol: String,
    pub borrow_balance_underlying: Value,
    pub supply_balance_underlying: Value,
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

        let mut map: HashMap<&str, (f64, f64)> = HashMap::new();

        for token in &self.tokens {
            let supply = token.supply_balance_underlying.value();
            let borrow = token.borrow_balance_underlying.value();
            map.insert(&token.symbol, (supply, borrow));
        }

        let dollarcoins = vec!["cUSDT", "cDAI", "cUSDC"];
        for token in &self.tokens {
            let supply = token.supply_balance_underlying.value();
            let borrow = token.borrow_balance_underlying.value();
            if dollarcoins.contains(&token.symbol.as_str()) {
            match map.get("cETH") {
                Some(c_eth) => {
                    let eth_supply = c_eth.0;
                    if eth_supply > 0.0 && borrow > 0.0001 {
                        let liq_price = borrow / eth_supply;
                        let m = format!(
                            "{}:{:.4} ${:.4}",
                            token.symbol.to_owned(),
                            supply,
                            liq_price,
                        );
                        parts.push(m)
                    }
                }
                None => (),
            }
            }
        }
        write!(f, "{}", parts.join(" "))
    }
}
