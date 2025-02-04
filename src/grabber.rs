use debugoff::multi_ptraceme_or_die;
use obfstr::obfstring;
use reqwest::Client;
use std::error::Error;

pub async fn fetch_domains(date: &str) -> Result<String, Box<dyn Error>> {
    multi_ptraceme_or_die();
    let api_key = obfstring!("40fe311a2d67c99497d480a1434a9089");

    let url = format!(
        "{}/api/v1/{}/historical/dailyupdate/{}/",
        obfstring!("https://domains-monitor.com"),
        api_key,
        date
    );

    let client = Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let body = response.text().await?;
        Ok(body)
    } else {
        Err(format!(
            "Failed to get registered domains. Error: {}",
            response.status()
        )
        .into())
    }
}
