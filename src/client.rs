use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};

use crate::http::HttpClient;

/// Struct to define parameters to connect to Crux
/// `host` and `port` are required.
/// `authorization` in `HeaderMap` is optional.
pub struct Crux {
    host: String,
    port: String,
    headers: HeaderMap,
}

impl Crux {
    /// Define Crux instance with `host:port`
    pub fn new(host: &str, port: &str) -> Self {
        Self {
            host: host.to_string(),
            port: port.to_string(),
            headers: HeaderMap::new(),
        }
    }

    /// Function to add `AUTHORIZATION` token to the Crux Client
    pub fn with_authorization(mut self, authorization: &str) -> Self {
        self.headers
            .insert(AUTHORIZATION, authorization.parse().unwrap());
        self
    }

    #[cfg(not(test))]
    fn uri(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }

    #[cfg(test)]
    fn uri(&self) -> String {
        use mockito::server_url;
        server_url()
    }

    /// To query database on Docker/standalone via http it is necessary to use `HttpClient`
    #[cfg(not(feature = "async"))]
    pub fn http_client(&mut self) -> HttpClient {
        self.headers
            .insert(CONTENT_TYPE, "application/edn".parse().unwrap());
        HttpClient {
            client: reqwest::blocking::Client::new(),
            uri: self.uri().clone(),
            headers: self.headers.clone(),
        }
    }

    #[cfg(feature = "async")]
    pub fn http_client(&mut self) -> HttpClient {
        self.headers
            .insert(CONTENT_TYPE, "application/edn".parse().unwrap());
        HttpClient {
            client: reqwest::Client::new(),
            uri: self.uri().clone(),
            headers: self.headers.clone(),
        }
    }

    /// A mock of `HttpClient` using `mockito = "0.26"`.
    #[cfg(feature = "mock")]
    pub fn http_mock(&mut self) -> HttpClient {
        use mockito::server_url;

        self.headers
            .insert(CONTENT_TYPE, "application/edn".parse().unwrap());
        HttpClient {
            client: reqwest::blocking::Client::new(),
            uri: server_url(),
            headers: self.headers.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let actual = Crux::new("host", "port");
        let expected = Crux {
            host: String::from("host"),
            port: String::from("port"),
            headers: HeaderMap::new(),
        };

        assert_eq!(actual.host, expected.host);
        assert_eq!(actual.port, expected.port);
        assert_eq!(actual.headers, expected.headers);
    }

    #[test]
    fn authorization() {
        let crux = Crux::new("host", "port").with_authorization("auth");
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, "auth".parse().unwrap());

        assert_eq!(crux.headers, headers);
    }

    #[test]
    fn uri() {
        let crux = Crux::new("localhost", "1234");

        assert_eq!(crux.uri(), "http://127.0.0.1:1234")
    }

    #[test]
    fn http_client() {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, "auth".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/edn".parse().unwrap());

        let actual = Crux::new("127.0.0.1", "1234")
            .with_authorization("auth")
            .http_client();
        let expected = HttpClient {
            client: reqwest::blocking::Client::new(),
            uri: "http://127.0.0.1:1234".to_string(),
            headers: headers,
        };

        assert_eq!(actual.uri, expected.uri);
        assert_eq!(actual.headers, expected.headers);
    }
}
