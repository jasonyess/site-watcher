use std::collections::HashMap;

use reqwest::Url;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WatcherConfigurationJSON {
    pub frequency: u64,
    pub urls: HashMap<String, String>,
    pub webhooks: Vec<String>,
}

#[derive(Default)]
pub struct WatcherConfiguration {
    pub frequency: u64,
    pub urls: Vec<(String, Url)>,
    pub webhooks: Vec<Url>,
}

impl WatcherConfiguration {
    pub fn frequency(mut self, frequency: u64) -> Self {
        self.frequency = frequency;
        self
    }

    pub fn url(mut self, name: String, url: &str) -> Self {
        self.urls.push((
            name,
            Url::parse(url).expect(&format!("Site URL '{}' is not a valid URL", url)),
        ));
        self
    }

    pub fn webhook(mut self, url: &str) -> Self {
        self.webhooks
            .push(Url::parse(url).expect(&format!("Webhook URL '{}' is not a valid URL", url)));
        self
    }
}
