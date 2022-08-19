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
    let params = HashMap::from([
        ("nick", "Rust"),
    ]);
    let text: String = httpbin::graphql_demo(&params).await?.text().await?;
    println!("{}", text);
    Ok(())
}
