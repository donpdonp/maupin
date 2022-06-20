pub static ACCOUNTS_URL: &str = "https://api.compound.finance/api/v2/account";

pub struct Response {
    accounts: Vec<Account>,
}

pub struct Account {
    address: String,
    health: Value,
    total_borrow_value_in_eth: Value,
    total_collateral_value_in_eth: Value,
}

pub struct Value {
    value: f64,
}
