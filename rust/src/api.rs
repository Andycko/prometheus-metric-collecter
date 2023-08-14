use reqwest::blocking::Client;
use reqwest::Url;
use serde::Deserialize;

pub struct ApiClient {
    client: Client,
    base_url: Url,
}

#[derive(Deserialize)]
pub struct Ip {
    pub origin: String,
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

    pub fn get_ip(&self) -> Result<Ip, reqwest::Error> {
        println!("sending request");
        let url = match Url::join(&self.base_url, "ip") {
            Ok(t) => t,
            Err(_) => panic!("Error joining strings"),
        };

        match self.client.get(url.as_str()).send() {
            Ok(t) => Ok(t.json::<Ip>()?),
            Err(e) => Err(e),
        }
    }
}
