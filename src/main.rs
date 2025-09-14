use std::{collections::HashMap, thread::sleep, time::Duration};

use crate::{
    config::{WatcherConfiguration, WatcherConfigurationJSON},
    request::{retrieve_site, webhook_message},
};

mod config;
mod request;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv::dotenv().unwrap();
    env_logger::init();

    let config_json: WatcherConfigurationJSON =
        serde_json::from_str(&std::fs::read_to_string("config.json").unwrap()).unwrap();

    let mut config = WatcherConfiguration::default().frequency(config_json.frequency);

    for (name, url) in config_json.urls {
        config = config.url(name, &url)
    }

    for webhook in config_json.webhooks {
        config = config.webhook(&webhook)
    }

    let mut previous_states: HashMap<String, String> = HashMap::new();

    for (name, url) in &config.urls {
        previous_states.insert(
            name.to_owned(),
            retrieve_site(&url).await.expect(&format!(
                "Failed to retrieve initial site content for '{}'",
                name
            )),
        );
        log::info!("Retrieved initial site content for '{}'", name);
    }

    loop {
        sleep(Duration::from_secs(config.frequency));

        for (name, url) in &config.urls {
            let previous_state = previous_states.get(name).unwrap();
            let current_state = retrieve_site(&url).await?;

            log::info!("Retrieved site content for '{}'", name);

            if previous_state != &current_state {
                for webhook in &config.webhooks {
                    webhook_message(&webhook, format!("{} was updated @everyone", name)).await?;
                }
            }

            previous_states.insert(name.to_owned(), current_state);
        }
    }
}
