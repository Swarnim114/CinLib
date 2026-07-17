use anyhow::Result;
use once_cell::sync::Lazy;
use reqwest::{Client, ClientBuilder};
use std::time::Duration;

pub static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    ClientBuilder::new()
        .timeout(Duration::from_secs(10))
        .user_agent("CinLib/1.0")
        .build()
        .expect("Failed to build reqwest client")
});

pub async fn get_bytes(url: &str) -> Result<bytes::Bytes> {
    let res = HTTP_CLIENT.get(url).send().await?.error_for_status()?;
    Ok(res.bytes().await?)
}
