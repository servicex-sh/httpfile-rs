use std::collections::HashMap;
use reqwest::{Response, Result};
use reqwest::header::HeaderMap;
// this would include code generated for package hello from .http file
httpfile::include_http!("index");
