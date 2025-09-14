use std::{collections::HashMap, path::Path};

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
    pub fn from_json<P: AsRef<Path>>(path: P) -> Self {
        let config_json: WatcherConfigurationJSON =
            serde_json::from_str(&std::fs::read_to_string(path.as_ref()).unwrap()).unwrap();

        let mut config = WatcherConfiguration::default().frequency(config_json.frequency);

        for (name, url) in config_json.urls {
            config = config.url(name, &url)
        }

        for webhook in config_json.webhooks {
            config = config.webhook(&webhook)
        }

        config
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_webhooks() {
        let config = WatcherConfiguration::from_json("config.json");

        for webhook in config.webhooks {
            crate::request::webhook_message(&webhook, format!("This is a test"))
                .await
                .unwrap();
        }
    }
}
