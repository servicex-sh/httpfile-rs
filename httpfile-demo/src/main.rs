mod httpbin;

use std::collections::HashMap;
use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MyIp {
    origin: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let json: MyIp = httpbin::my_ip().await?.json().await?;
    println!("{:?}", json);
    let mut params: HashMap<String, String> = HashMap::new();
    params.insert("nick".to_owned(), "rust".to_owned());
    let text: String = httpbin::graphql_demo(&params).await?.text().await?;
    println!("{}", text);
    Ok(())
}
