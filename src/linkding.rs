use crate::CONFIG;
use log::{debug, info};
use reqwest::Error;
use serde_json::{Map, Value};

pub(crate) async fn exists_in_linkding(url: &str) -> Result<bool, Error> {
    #[derive(serde::Deserialize, Debug)]
    struct LookupResponse {
        count: u32,
    }

    let resp = reqwest::Client::new()
        .get(format!(
            "{}/bookmarks/?q={}",
            CONFIG.linkding_api_path,
            urlencoding::encode(url)
        ))
        .header("Authorization", format!("Token {}", CONFIG.linkding_token))
        .send()
        .await?
        .json::<LookupResponse>()
        .await;

    if resp.is_err() {
        return Err(resp.err().unwrap());
    }

    Ok(resp?.count > 0)
}

pub(crate) async fn add_to_linkding(url: &str) {
    let mut map = Map::new();
    map.insert("url".to_string(), Value::String(url.to_string()));
    map.insert(
        "tag_names".to_string(),
        Value::Array(Vec::from([Value::String(CONFIG.linkding_tag.to_string())])),
    );

    info!("Adding URL: {}", url);

    let response = reqwest::Client::new()
        .post(format!("{}/bookmarks/", CONFIG.linkding_api_path))
        .header("Authorization", format!("Token {}", CONFIG.linkding_token))
        .json(&map)
        .send()
        .await;
    debug!("Response status: {}", response.unwrap().status())
}
