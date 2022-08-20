mod models;
mod parser;

use std::env;
use std::fs::File;
use std::io::{Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Builder {
    pub httpfile_path: String,
    pub http_client: String,
    out_dir: Option<PathBuf>,
}

pub fn configure() -> Builder {
    Builder {
        httpfile_path: "index.http".to_string(),
        http_client: "reqwest".to_string(),
        out_dir: Some(PathBuf::from(env::var("OUT_DIR").unwrap())),
    }
}

impl Builder {
    pub fn httpfile(mut self, httpfile_path: impl AsRef<Path>) -> Self {
        self.httpfile_path = httpfile_path.as_ref().to_str().unwrap().to_string();
        self
    }
    pub fn out_dir(mut self, out_dir: impl AsRef<Path>) -> Self {
        self.out_dir = Some(out_dir.as_ref().to_path_buf());
        self
    }

    pub fn compile(self) -> std::io::Result<()> {
        let http_file_path = Path::new(&self.httpfile_path);
        let httpfile_text = std::fs::read_to_string(http_file_path)?;
        let request_targets = parser::parse_http_code(&httpfile_text);
        let mut file_lines: Vec<String> = Vec::new();
        file_lines.push("use std::collections::HashMap;".to_owned());
        file_lines.push("use reqwest::{Client, Response, Result};".to_owned());
        file_lines.push("use reqwest::header::HeaderMap;".to_owned());
        file_lines.push("use handlebars::Handlebars;".to_owned());
        file_lines.push("".to_owned());
        // lazy_static block
        file_lines.push("lazy_static::lazy_static! {".to_owned());
        file_lines.push("  static ref CLIENT: Client = reqwest::Client::new();".to_owned());
        file_lines.push("  static ref HANDLEBARS: Handlebars<'static> = {".to_owned());
        file_lines.push("    let mut reg = Handlebars::new();".to_owned());
        // include http body template
        for request_target in &request_targets {
            if let Some(body) = &request_target.body {
                if body.contains("{{") {
                    let template_name = format!("{}_body", request_target.name);
                    file_lines.push(format!("    reg.register_template_string(\"{}\", r#\"{}\"#).unwrap();", template_name, body));
                }
            }
        }
        file_lines.push("    reg".to_owned());
        file_lines.push("    };".to_owned());
        file_lines.push("  }".to_owned());
        for request_target in &request_targets {
            file_lines.push("".to_owned());
            file_lines.push(request_target.to_rust_code());
        }
        let rust_file_code = file_lines.join("\n");
        let rust_file_name = http_file_path.file_name().unwrap().to_str().unwrap().replace(".http", ".rs");
        let dest_path = self.out_dir.unwrap().join(rust_file_name);
        println!("dest_path = {:?}", dest_path);
        let mut file = File::create(dest_path)?;
        file.write_all(rust_file_code.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let path = env::current_dir().unwrap().join("temp");
        env::set_var("OUT_DIR", path.to_str().unwrap());
        configure()
            .httpfile("index.http")
            .compile()
            .unwrap();
    }
}

