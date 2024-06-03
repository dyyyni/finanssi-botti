use log::error;
use reqwest::Error;
use std::collections::HashMap;

pub async fn get_token() -> Result<String, Error> {
    let url = "https://s-pankki-api-sandbox.crosskey.io/open-banking/v2.0/oidc/token";

    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");
    params.insert("scope", "scope1");
    params.insert("client_id", "TODO_get_from_env");

    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .form(&params)
        .send()
        .await?;

    if let Err(err) = response.error_for_status_ref() {
        let status = err.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        let error_message = format!("Failed to get token: {}. Status: {:?}", error_text, status);
        error!("{}", error_message);
        return Err(err);
    }

    let response_text = response.text().await?;
    Ok(response_text)
}