use crate::compound::*;
use hyper::Client;
use hyper_tls::HttpsConnector;

#[tokio::main]
pub async fn update(
) -> Result<hyper::Response<hyper::Body>, Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Parse an `http::Uri`...
    let uri = ACCOUNTS_URL.parse()?;
    println!("{}", uri);

    // Await the response...
    let resp = client.get(uri).await?;

    Ok(resp)
}
