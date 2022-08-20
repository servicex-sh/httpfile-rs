#[derive(Debug)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

impl HttpHeader {
    pub fn new(name: String, value: String) -> HttpHeader {
        HttpHeader {
            name,
            value,
        }
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    pub index: u32,
    pub comment: Option<String>,
    pub tags: Vec<String>,
    pub name: String,
    pub method: String,
    pub request_line: String,
    pub path: String,
    pub protocol: Option<String>,
    pub uri: Option<String>,
    pub body_started: bool,
    pub headers: Vec<HttpHeader>,
    pub body_lines: Vec<String>,
    pub line_numbers: Vec<u32>,
    pub body: Option<String>,
    pub js_code: Option<String>,
    pub redirect_response: Option<String>,
    pub variable_names: Vec<String>,
}

impl HttpRequest {
    pub fn new(index: u32) -> HttpRequest {
        HttpRequest {
            index,
            comment: None,
            tags: Vec::new(),
            name: format!("http{}", index),
            method: String::new(),
            request_line: String::new(),
            path: String::new(),
            protocol: None,
            uri: None,
            body_started: false,
            headers: Vec::new(),
            body_lines: Vec::new(),
            line_numbers: Vec::new(),
            body: None,
            js_code: None,
            redirect_response: None,
            variable_names: Vec::new(),
        }
    }

    pub fn is_filled(&self) -> bool {
        !self.method.is_empty()
    }

    pub fn add_line_number(&mut self, line_number: u32) {
        self.line_numbers.push(line_number);
    }

    pub fn append_request_line(&mut self, part: &str) {
        self.request_line.push_str(part);
    }

    pub fn get_header_value(&self, name: &str) -> Option<&str> {
        for header in &self.headers {
            if header.name == name {
                return Some(&header.value);
            }
        }
        None
    }

    pub fn cleanup_metadata(&mut self) {
        let parts = self.request_line.split_whitespace().collect::<Vec<&str>>();
        if !parts.is_empty() {
            self.path = parts[0].to_string();
            if self.path.starts_with("http://") || self.path.starts_with("https://") {
                self.uri = Some(self.path.clone());
            }
        }
        if parts.len() > 1 {
            self.protocol = Some(parts[1].to_string());
        }
        if self.uri.is_none() {
            if is_http_method(&self.method) {
                if let Some(host) = self.get_header_value("Host") { self.uri = Some(format!("http://{}", host)); }
            } else if let Some(uri) = self.get_header_value("URI") {
                self.uri = Some(uri.to_string());
            }
        }
    }

    pub fn cleanup_body(&mut self) {
        if !self.body_lines.is_empty() {
            let mut lines: Vec<String> = Vec::new();
            for line in self.body_lines.iter() {
                if !line.starts_with("<>") {
                    lines.push(line.clone());
                }
            }
            //javascript code block
            let mut js_start_offset = lines.len();
            let mut js_end_offset = 0;
            for (pos, line) in lines.iter().enumerate() {
                if line.starts_with("> {%") {
                    js_start_offset = pos;
                }
                if line == "%}" && pos > js_start_offset {
                    js_end_offset = pos;
                    break;
                }
            }
            if js_end_offset > 0 {
                self.js_code = Some(lines[(js_start_offset + 1)..js_end_offset].join("\n"));
                lines.drain(js_start_offset..(js_end_offset + 1));
            }
            // clean js handler:  drain_filter is not stable
            let mut i = 0;
            while i < lines.len() {
                if lines[i].starts_with("> ") && lines[i].ends_with(".js") {
                    lines.remove(i);
                } else if lines[i].starts_with(">> ") || lines[i].starts_with(">>! ") {
                    self.redirect_response = Some(lines.remove(i));
                } else {
                    i += 1;
                }
            }
            //clean tail empty line
            while lines.len() > 0 && lines[lines.len() - 1].is_empty() {
                lines.pop();
            }
            // set http body
            if !lines.is_empty() {
                if self.method == "GRAPHQL" {
                    let json_offset = lines.iter().position(|r| r == "{").unwrap_or(0);
                    let last_line = lines.get(lines.len() - 1).unwrap();
                    let mut doc = String::new();
                    doc.push_str(r#"{"query": ""#);
                    if json_offset > 0 && last_line == "}" {
                        let query = lines[0..json_offset].join("\n");
                        let variables = lines[json_offset..].join("\n");
                        doc.push_str(query.replace("\"", "\\\"").replace("\n", "\\n").as_str());
                        doc.push_str("\"");
                        doc.push_str(r#","variables": "#);
                        doc.push_str(&variables);
                        doc.push_str("\n}");
                    } else {
                        let query = lines.join("\n");
                        doc.push_str(query.replace("\"", "\\\"").replace("\n", "\\n").as_str());
                        doc.push_str("\"}");
                    };
                    self.body = Some(doc);
                } else {
                    self.body = Some(lines.join("\n"));
                }
            }
        }
    }

    pub fn to_rust_code(&self) -> String {
        let mut code_lines: Vec<String> = Vec::new();
        let mut variables_included = false;
        //url code
        if let Some(uri) = &self.uri {
            if uri.contains("{{") {
                code_lines.push(format!(r#"  let url = {};"#, to_place_holder(uri)));
                variables_included = true;
            } else {
                code_lines.push(format!(r#"  let url = "{}";"#, uri));
            }
        }
        // headers
        code_lines.push("  let mut headers = HeaderMap::new();".to_owned());
        for header in &self.headers {
            let name = &header.name;
            if self.method != "GRAPHQL" && name == "Content-Type" {
                continue;
            }
            let value = &header.value;
            if value.contains("{{") {
                variables_included = true;
                code_lines.push(format!(r#"  headers.insert("{}", {}.parse().unwrap());"#, header.name, to_place_holder(value)));
            } else {
                code_lines.push(format!(r#"  headers.insert("{}", "{}".parse().unwrap());"#, header.name, header.value));
            }
        }
        if self.method == "GRAPHQL" {
            code_lines.push(r#"headers.insert("Content-Type", "application/json".parse().unwrap());"#.to_owned());
        }
        // http body for POST, PUT and GRAPHQL
        if self.method == "POST" || self.method == "PUT" || self.method == "GRAPHQL" {
            if let Some(body) = &self.body {
                if body.contains("{{") {
                    variables_included = true;
                    let template_name = format!("{}_body", self.name);
                    code_lines.push(format!(r#"  let body = HANDLEBARS.render("{}", &params).unwrap();"#, template_name));
                } else {
                    code_lines.push(format!("  let body = r#\"{}\"#;", body));
                }
            }
        }
        if self.method == "GET" {
            code_lines.push("  CLIENT.get(url).headers(headers).send().await".to_owned());
        } else if self.method == "POST" || self.method == "GRAPHQL" {
            code_lines.push("  CLIENT.post(url).headers(headers).body(body).send().await".to_owned());
        } else {
            code_lines.push(format!("  CLIENT.{}(url).headers(headers).body(body).send().await", self.method));
        }
        code_lines.push("}".to_owned());
        if variables_included {
            code_lines.insert(0, format!("pub async fn {}(params: &HashMap<&str, &str>) -> Result<Response> {{", self.name));
        } else {
            code_lines.insert(0, format!("pub async fn {}() -> Result<Response> {{", self.name));
        }
        code_lines.join("\n")
    }
}

pub fn is_legal_method(method: &str) -> bool {
    matches!(method,
        "GET" | "POST" | "PUT" | "DELETE"| "GRAPHQL"
    )
}

pub fn is_http_method(method: &str) -> bool {
    matches!(method,
        "GET" | "POST" | "PUT" | "DELETE" | "HEAD" | "OPTIONS" | "PATCH" | "TRACE"
    )
}

fn replace_variables(text: &str) -> (String, Vec<String>) {
    let mut result = String::new();
    let mut variable_names: Vec<String> = Vec::new();
    let mut offset = text.find("{{").unwrap();
    let code_length = text.len();
    result.push_str(&text[..offset]);
    offset += 2;
    while offset < code_length {
        let temp_offset = text[offset..].find("}}");
        if temp_offset.is_none() {
            break;
        }
        let mut name = text[offset..(offset + temp_offset.unwrap())]
            .trim()
            .to_string();
        if name.starts_with('$') {
            name = name[1..].to_string();
        }
        name = name.replace("-", "_");
        variable_names.push(name.clone());
        result.push_str(&format!("{{{}}}", name));
        offset += temp_offset.unwrap() + 2;
        if let Some(value) = text[offset..].find("{{") {
            result.push_str(text[offset..(offset + value)].to_string().as_str());
            offset += value + 2;
        } else {
            result.push_str(text[offset..].to_string().as_str());
            break;
        }
    }
    (result, variable_names)
}

pub fn to_place_holder(text: &str) -> String {
    let result = replace_variables(text);
    let params_declare = result.1.into_iter().map(|name| format!(r#"{} = params.get("{}").unwrap_or(&"")"#, name, name)).collect::<Vec<String>>().join(", ");
    format!("format!(r#\"{}\"#, {})", result.0, params_declare)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_http_code;
    use handlebars::Handlebars;
    use std::collections::HashMap;
    lazy_static::lazy_static! {
        static ref HANDLEBARS: Handlebars<'static> = {
            let mut reg = Handlebars::new();
            reg.register_template_string("tpl_1", "Hello {{name}}").unwrap();
            reg
        };
    }


    #[test]
    fn test_to_rust_code() {
        let http_code = r#"
### test
//@name my-ip
GET https://{{host}}/ip HTTP/1.1
Host: {{host}}
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.149 Safari/537.36

Hi {{jackiye}}!
good morning
goood
"#;
        let mut requests = parse_http_code(http_code);
        let mut request = requests.remove(0);
        request.cleanup_body();
        println!("{}", request.to_rust_code());
    }

    #[test]
    fn test_graphql_to_rust_code() {
        let http_code = r#"
### GraphQL demo with variables
//@name graphql-demo
GRAPHQL https://httpbin.org/post

query {
    ip
}

{
  "id": 1,
  "name": "{{nick}}"
}



"#;
        let mut requests = parse_http_code(http_code);
        let request = requests.remove(0);
        println!("{:?}", request);
        println!("{}", request.to_rust_code());
    }

    #[test]
    fn test_replace_variables() {
        // language=http_request
        let http_code = r#"
### test
//@name myip
GET https://{{host}}/ip HTTP/1.1

hi {{name}}, nice to meet you! {{$uuid}}
"#;
        let result = replace_variables(http_code);
        println!("{:?}", result);
    }

    #[test]
    fn test_handlerbars() {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("name", "Jackie");
        // register template using given name
        let result = HANDLEBARS.render("tpl_1", &params).unwrap();
        println!("{}", result);
    }
}
