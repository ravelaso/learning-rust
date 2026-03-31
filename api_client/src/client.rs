#![allow(unused)]

#[derive(Deserialize, Debug)]
struct AuthResponse {
    token: String,
}

use reqwest::{Error, Response, Url};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct LoginBody<'a> {
    username: &'a str,
    password: &'a str,
}

#[derive(Debug)]
pub struct ApiClient {
    client: reqwest::Client,
    base_url: Url,
    username: Option<String>,
    password: Option<String>,
    token: Option<String>,
}
impl ApiClient {
    pub fn new(url: String, user: String, pass: String) -> Self {
        ApiClient {
            client: reqwest::Client::new(),
            base_url: Url::parse(&url).unwrap(),
            username: Some(user),
            password: Some(pass),
            token: None,
        }
    }
    pub async fn authenticate(&mut self) {
        let body = LoginBody {
            username: self.username.as_deref().unwrap(),
            password: self.password.as_deref().unwrap(),
        };
        let res = self
            .client
            .post(format!("{}/authenticate", &self.base_url))
            .json(&body)
            .send()
            .await
            .unwrap();

        match res.status().is_success() {
            true => {
                let authres = res.json::<AuthResponse>().await.unwrap();
                self.token = Some(authres.token);
                println!("Authentication successful");
            }
            false => {
                println!("Authentication failed: {}", res.text().await.unwrap());
            }
        }
    }
    pub async fn get_company_info(&self, country: &str, regno: &str) -> Result<Response, Error> {
        let res = self
            .client
            .get(format!(
                "{}companies?countries={}&regNo={}",
                &self.base_url, country, regno
            ))
            .bearer_auth(self.get_token().unwrap())
            .send()
            .await?
            .error_for_status()?;
        Ok(res)
    }

    pub fn get_token(&self) -> Option<&str> {
        self.token.as_deref()
    }
}
