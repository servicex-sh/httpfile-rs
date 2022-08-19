mod httpbin;

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
    Ok(())
}
