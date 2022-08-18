use std::collections::HashMap;
use reqwest::{Response, Result};
// this would include code generated for package hello from .http file
httpfile::include_http!("index");

pub async fn my_ip2() -> Result<Response> {
    reqwest::get("https://httpbin.org/ip").await
}

pub async fn post_test(params: &HashMap<String, String>) -> Result<Response> {
    let client = reqwest::Client::new();
    let url = format!("https://{host}/post", host = params.get("host").unwrap());
    let mut resp = client.post(url)
        .header("Content-Type", "application/json")
        .json(params)
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
