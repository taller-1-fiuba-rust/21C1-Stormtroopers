use std::io::Write;
use std::net::TcpStream;

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse {
    version: String,
    status_code: String,
    status_text: String,
    pub content: String,
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status_code: "200".to_string(),
            status_text: "OK".to_string(),
            content: "".to_string(),
        }
    }
}

impl HttpResponse {
    pub fn new(version: String, status_code: String, status_text: String, content: String) -> Self {
        Self {
            version,
            status_code,
            status_text,
            content,
        }
    }

    pub fn response_with_content(mut http_response: HttpResponse, content: String) -> Self {
        http_response.content = content;
        http_response
    }

    pub fn send_response(&self, stream: &mut TcpStream) {
        let res = self.clone();
        let response_string: String = String::from(res);
        stream.write_all(response_string.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

impl HttpResponse {
    fn version(&self) -> String {
        self.version.clone()
    }

    fn status_code(&self) -> String {
        self.status_code.clone()
    }

    fn status_text(&self) -> String {
        self.status_text.clone()
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }
}

impl From<HttpResponse> for String {
    fn from(res: HttpResponse) -> String {
        let res1 = res.clone();
        format!(
            "{} {} {}\r\nContent-Length: {}\r\n\r\n{}",
            res1.version(),
            res1.status_code(),
            res1.status_text(),
            res.content().len(),
            res1.content()
        )
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new(
            "200",
            None,
            Some("Item was shipped on 21st Dec 2020".into()),
        );
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Item was shipped on 21st Dec 2020".into()),
        };
        assert_eq!(response_actual, response_expected);
    }
    #[test]
    fn test_response_struct_creation_404() {
        let response_actual = HttpResponse::new(
            "404",
            None,
            Some("Item was shipped on 21st Dec 2020".into()),
        );
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Item was shipped on 21st Dec 2020".into()),
        };
        assert_eq!(response_actual, response_expected);
    }
    #[test]
    fn test_http_response_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Item was shipped on 21st Dec 2020".into()),
        };
        let http_string: String = response_expected.into();
        let response_actual = "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 33\r\n\r\nItem was shipped on 21st Dec 2020";
        assert_eq!(http_string, response_actual);
    }
}
*/
