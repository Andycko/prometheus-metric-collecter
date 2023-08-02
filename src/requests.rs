use reqwest::blocking::Client;
use reqwest::Url;
use std::collections::HashMap;

pub struct ApiClient {
    client: Client,
    base_url: Url,
}

impl ApiClient {
    pub fn new(base_url: &str) -> ApiClient {
        let client = reqwest::blocking::Client::new();

        let parsed_url = match Url::parse(base_url) {
            Ok(v) => v,
            Err(e) => panic!("Error reading base URL - {e:?}"),
        };

        ApiClient {
            base_url: parsed_url,
            client,
        }
    }

    pub fn get_ip(&self) -> Result<HashMap<String, String>, reqwest::Error> {
        println!("sending request");
        let url = match Url::join(&self.base_url, "ip") {
            Ok(t) => t,
            Err(_) => panic!("Error joining strings"),
        };

        match self.client.get(url.as_str()).send() {
            Ok(t) => Ok(t.json::<HashMap<String, String>>()?),
            Err(e) => Err(e),
        }
    }
}
