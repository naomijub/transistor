use reqwest::{
    header::{HeaderMap,AUTHORIZATION},
    blocking::{Client}, 
    Result
};

pub struct Crux {
    host: String,
    port: String,
    headers: HeaderMap
}

impl Crux{
    pub fn new(host: &str, port: &str) -> Self {
        Self{host: host.to_string(), port: port.to_string(), headers: HeaderMap::new()}
    }

    pub fn with_authorization(mut self, authorization: &str) -> Self {
        self.headers.insert(AUTHORIZATION, authorization.parse().unwrap());
        self
    }

    fn uri(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }

    pub fn client(&self) -> CruxClient {
        CruxClient {
            client: reqwest::blocking::Client::new(),
            uri: self.uri().clone(),
            headers: self.headers.clone()
        }
    }
}

pub struct CruxClient {
    client: Client,
    uri: String, 
    headers: HeaderMap,
}

impl CruxClient {
    pub fn state(&self) -> Result<String> {
        self.client.get(&self.uri)
            .headers(self.headers.clone())
            .send()?
            .text()
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
        let crux = Crux::new("localhost", "3000");

        assert_eq!(crux.uri(), "http://localhost:3000")
    }

    #[test]
    fn client() {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, "auth".parse().unwrap());

        let actual = Crux::new("localhost", "3000").with_authorization("auth").client();
        let expected = CruxClient {
            client: reqwest::blocking::Client::new(),
            uri: "http://localhost:3000".to_string(),
            headers: headers,
        };

        assert_eq!(actual.uri, expected.uri);
        assert_eq!(actual.headers, expected.headers);
    }
}