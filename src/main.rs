mod instapaper;
mod linkding;

#[macro_use]
extern crate lazy_static;

use log::info;
use reqwest::Url;
use std::env;
use std::time::Duration;

pub struct Config {
    pub instapaper_feed: String,
    pub linkding_api_path: String,
    pub linkding_token: String,
}

lazy_static! {
    static ref CONFIG: Config = init_and_validate_config();
}

#[tokio::main]
async fn main() {

    env_logger::init();

    loop {
        let feed = instapaper::get_feed(&CONFIG.instapaper_feed).await;
        info!("Retrieved {} starred URLs", feed.len());

        for link in feed {
            if !linkding::exists_in_linkding(&link).await {
                linkding::add_to_linkding(&link).await;
            }
        }

        tokio::time::sleep(Duration::from_secs(600)).await;
    }
}

fn init_and_validate_config() -> Config {
    let config = Config {
        instapaper_feed: get_env_var("INSTAPAPER_FEED_URL"),
        linkding_api_path: get_env_var("LINKDING_API_PATH"),
        linkding_token: get_env_var("LINKDING_TOKEN"),
    };

    if Url::parse(&*config.linkding_api_path).is_err() {
        panic!("INSTAPAPER_FEED_URL is an invalid URL")
    }

    if Url::parse(&*config.linkding_api_path).is_err() {
        panic!("LINKDING_API_PATH is an invalid URL")
    }
    if str::ends_with(&*config.linkding_api_path, "/") {
        panic!("Please specify LINKDING_API_PATH without a trailing slash")
    }

    config
}

fn get_env_var(env_var_name: &str) -> String {
    let env_var =
        env::var(env_var_name).expect(&*format!("{} env variable is undefined", env_var_name));

    if env_var.len() < 1 {
        panic!("{}  env variable is empty", env_var_name)
    }

    env_var
}
