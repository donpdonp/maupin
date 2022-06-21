use crate::compound;
use redis;
use reqwest;

pub fn update() -> Result<compound::Response, reqwest::Error> {
    use reqwest::blocking::Client;
    let client = Client::new();
    let url = reqwest::Url::parse(compound::ACCOUNTS_URL).unwrap();
    println!("{}", url);
    let accounts_options = compound::AccountOptions {
        page_size: 15,
        max_health: compound::Value {
            value: "1.1".to_string(),
        },
    };
    let options_json = serde_json::to_string(&accounts_options).unwrap();
    let resp = client.post(url).body(options_json).send()?;

    let accounts: compound::Response = serde_json::from_str(&resp.text()?).unwrap();

    Ok(accounts)
}

pub fn merge(redis: redis::Client, response: compound::Response) {
    println!("{}", response.accounts.len());
}
