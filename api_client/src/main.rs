mod client;
use client::ApiClient;

use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let url = env::var("API_URL").expect("URL environment variable must be set");
    let user = env::var("API_USERNAME").expect("USERNAME environment variable must be set");
    let pass = env::var("API_PASSWORD").expect("PASSWORD environment variable must be set");

    let mut client = ApiClient::new(url, user, pass);
    client.authenticate().await;

    let res1 = client.get_company_info("NL", "34177641").await;
    let res2 = client.get_company_info("NL", "52393518").await;
    println!("{:?}", res1?.text().await?);
    println!("{:?}", res2?.text().await?);
    Ok(())
}
