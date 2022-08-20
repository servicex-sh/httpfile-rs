use crate::models::{is_legal_method, HttpRequest, HttpHeader};

pub fn is_request_line(line: &str) -> bool {
    let space_include = line.find(' ');
    if let Some(offset) = space_include {
        let method = line[..offset].to_string();
        return is_legal_method(&method);
    }
    false
}

pub fn parse_http_code(http_code: &str) -> Vec<HttpRequest> {
    let lines: Vec<String> = http_code.lines().map(|x| x.to_string()).collect();
    let mut requests = Vec::new();
    let mut request_index = 1;
    let mut http_request = HttpRequest::new(request_index);
    let mut index: u32 = 0;
    let mut line_number: u32 = 1;
    for raw_line in lines {
        let line = raw_line.trim();
        // skip shebang of first line
        if index == 0 && line.starts_with("#!") {
            index += 1;
            line_number += 1;
            continue;
        }
        index += 1;
        // empty line between requests
        if !http_request.is_filled() && line.is_empty() {
            line_number += 1;
            http_request.add_line_number(line_number);
            continue;
        }
        // comment for httpRequest or new HttpRequest separator
        if line.starts_with("###") {
            let comment = line.strip_prefix("###").unwrap().to_string();
            if !http_request.is_filled() {
                // set comment for request
                http_request.comment = Some(comment);
            } else {
                //start new request
                http_request.cleanup_metadata();
                http_request.cleanup_body();
                requests.push(http_request);
                request_index += 1;
                http_request = HttpRequest::new(request_index);
                http_request.comment = Some(comment);
            }
        } else if !http_request.body_started {
            // comment for httpRequest
            if line.starts_with('#') || line.starts_with("//") {
                let comment = if line.starts_with('#') {
                    line.strip_prefix('#').unwrap().trim().to_string()
                } else {
                    line.strip_prefix("//").unwrap().trim().to_string()
                };
                if comment.starts_with('@') {
                    let tag = comment.strip_prefix('@').unwrap().trim().to_string();
                    let parts: Vec<&str> = if tag.contains(' ') {
                        tag.split(' ').collect()
                    } else {
                        tag.split('=').collect()
                    };
                    if parts[0] == "name" && parts.len() > 1 {
                        http_request.name = parts[1].replace("-", "_").to_string();
                    }
                    http_request.tags.push(tag);
                } else {
                    // normal comment
                    if http_request.comment.is_none() {
                        http_request.comment = Some(line[2..].trim().to_string());
                    }
                }
            } else if is_request_line(line) {
                // request line alike `POST /xxx HTTP/1.1`
                let offset = line.find(' ').unwrap();
                http_request.method = line[..offset].to_string();
                http_request.request_line = line[(offset + 1)..].to_string();
            } else if raw_line.starts_with("  ") || raw_line.starts_with('\t') {
                //append request line parts in multi lines
                http_request.append_request_line(line);
            } else if line.contains(':') && http_request.body_started == false {
                // header line
                let parts = line.splitn(2, ':').map(|s| s.trim()).collect::<Vec<&str>>();
                if parts[0].contains(' ') {
                    http_request.body_lines.push(line.to_string());
                    http_request.body_started = true;
                } else {
                    http_request
                        .headers
                        .push(HttpHeader::new(parts[0].to_string(), parts[1].to_string()));
                }
            } else if !line.is_empty() {
                // ignore lines between headers and body
                http_request.body_lines.push(line.to_string());
            } else {
                http_request.body_started = true;
            }
        } else {
            // body line
            http_request.body_lines.push(line.to_string());
        }
        http_request.add_line_number(line_number);
        line_number += 1;
    }

    if http_request.is_filled() {
        //add last httpRequest
        http_request.cleanup_metadata();
        http_request.cleanup_body();
        requests.push(http_request);
    }
    requests
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_code() {
        let http_code = r#"
### test
//@name my-ip
GET https://httpbing.org/ip HTTP/1.1
Host: httpbing.org
"#;
        let mut requests = parse_http_code(http_code);
        let request = requests.remove(0);
        println!("{:?}", request);
    }

    #[test]
    fn test_parse_post() {
        let http_code = r#"
### test
//@name post-test
POST https://httpbing.org/ip HTTP/1.1
Host: httpbing.org
Content-Type: application/json

{"id":1}

### test
//@name my-ip
GET https://httpbing.org/ip HTTP/1.1
Host: httpbing.org
"#;
        let mut requests = parse_http_code(http_code);
        let request = requests.remove(0);
        println!("{:?}", request);
    }
}
