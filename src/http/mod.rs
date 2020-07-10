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