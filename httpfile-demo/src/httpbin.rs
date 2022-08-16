use reqwest::{Response, Result};
// this would include code generated for package hello from .http file
httpfile::include_http!("index");

pub async fn my_ip2() -> Result<Response> {
    reqwest::get("https://httpbin.org/ip").await
}
