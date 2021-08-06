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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new(
            "HTTP/1.1".to_string(),
            "200".to_string(),
            "OK".to_string(),
            "Contenido del Response".to_string(),
        );
        let response_expected = HttpResponse {
            version: "HTTP/1.1".to_string(),
            status_code: "200".to_string(),
            status_text: "OK".to_string(),
            content: "Contenido del Response".to_string(),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_http_response_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1".to_string(),
            status_code: "404".to_string(),
            status_text: "Not Found".to_string(),
            content: "Contenido del Response".to_string(),
        };
        let http_string: String = response_expected.into();
        let response_actual =
            "HTTP/1.1 404 Not Found\r\nContent-Length: 22\r\n\r\nContenido del Response";
        assert_eq!(http_string, response_actual);
    }
}
