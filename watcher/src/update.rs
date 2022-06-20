use crate::compound;
use reqwest;

pub fn update() -> Result<compound::Response, reqwest::Error> {
    println!("{}", compound::ACCOUNTS_URL);
    use reqwest::blocking::Client;
    let client = Client::new();
    let resp = client.get(compound::ACCOUNTS_URL).send()?;

    let accounts: compound::Response = serde_json::from_str(&resp.text()?).unwrap();

    Ok(accounts)
}
