use rss::{Channel, Item};
use std::collections::HashSet;

pub(crate) async fn get_feed(url: &str) -> HashSet<String> {
    let feed = reqwest::get(url).await.unwrap().text().await;
    let channel = Channel::read_from(feed.unwrap().as_bytes()).unwrap();
    return channel
        .items()
        .into_iter()
        .map(Item::link)
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .map(|x| x.to_string())
        .collect();
}
