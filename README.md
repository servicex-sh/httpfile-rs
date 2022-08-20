httpfile-rs
======================

# How to get started?

* Add dependencies in `Cargo.toml`:

```toml
[dependencies]
httpfile = "0.1"
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
handlebars = "4"
lazy_static="1.4"

[build-dependencies]
httpfile-build = "0.1"
```

* Create http file, such as `index.http` with following content:

```
### get my ip
//@name my-ip
GET https://httpbin.org/ip
User-Agent: curl/7.47.0
```

* Create a Rust module, such as `httpbin.rs` with following content:

```
// this would include code generated for package hello from .http file
httpfile::include_http!("index");
```

* Add following code to build.rs:

```
httpfle::configure()
        .httpfile("index.http")
        .compile()
        .unwrap();
```

* Call http requests in your code:

```
let response = httpbin::my_ip().await?;
```

Please refer [httpfile-demo](./httpfile-demo) for details.

# References

* Environment variables Cargo sets for build
  scripts: https://doc.rust-lang.org/cargo/reference/environment-variables.html
* reqwest: an ergonomic, batteries-included HTTP Client for Rust - https://github.com/seanmonstar/reqwest
