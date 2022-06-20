use crate::compound::*;
use reqwest;

pub fn update() -> Result<String, reqwest::Error> {
    println!("{}", ACCOUNTS_URL);
    use reqwest::blocking::Client;
    let client = Client::new();
    let resp = client.get(ACCOUNTS_URL).send()?;

    let accounts = serde_json::from_str(&resp.text()?).unwrap();

    Ok(accounts)
}
