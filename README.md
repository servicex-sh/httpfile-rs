httpfile-rs
======================

# How to get started?

* Add dependencies in `Cargo.toml`:

```toml
[dependencies]
httpfile = "0.1"

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

# References

* Environment variables Cargo sets for build
  scripts: https://doc.rust-lang.org/cargo/reference/environment-variables.html
* reqwest: an ergonomic, batteries-included HTTP Client for Rust - https://github.com/seanmonstar/reqwest
