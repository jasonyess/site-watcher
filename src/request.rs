use std::sync::LazyLock;

use reqwest::{Client, Error, Url};

static CLIENT: LazyLock<Client> = LazyLock::new(|| Client::new());

pub async fn retrieve_site(url: &Url) -> Result<String, Error> {
    log::info!("Attempting to retrieve state of '{}'", url);

    Ok(reqwest::get(url.clone()).await?.text().await?)
}

pub async fn webhook_message(webhook: &Url, message: String) -> Result<(), Error> {
    log::info!("Attempting to send message via webhook at '{}'", webhook);

    CLIENT
        .post(webhook.clone())
        .json(&serde_json::json!({
            "content": message
        }))
        .send()
        .await?;

    Ok(())
}
