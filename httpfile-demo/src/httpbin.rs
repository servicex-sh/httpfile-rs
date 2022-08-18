use std::collections::HashMap;
use reqwest::{Response, Result};
use reqwest::header::HeaderMap;
// this would include code generated for package hello from .http file
httpfile::include_http!("index");

pub async fn my_ip2() -> Result<Response> {
    reqwest::get("https://httpbin.org/ip").await
}

pub async fn post_test(params: &HashMap<String, String>) -> Result<Response> {
    let client = reqwest::Client::new();
    // url
    let url = format!("https://{host}/post", host = params.get("host").unwrap());
    // http headers
    let mut headers = HeaderMap::new();
    // http text body
    let body = r#"{"id",1}"#;
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let mut resp = client.post(url)
        .headers(headers)
        .body(body)
        .send()
        .await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_post_test() -> Result<()> {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert("host".to_string(), "httpbin.org".to_string());
        let response = post_test(&params).await?;
        println!("{:?}", response.text().await?);
        Ok(())
    }
}
