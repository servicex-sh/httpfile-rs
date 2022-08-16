mod httpbin;

use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MyIp {
    origin: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    httpbin::hello();
    let json: MyIp = httpbin::my_ip2().await?.json().await?;
    println!("{:?}", json);
    Ok(())
}
