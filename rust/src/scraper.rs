use crate::api;

pub struct Scraper {
    api_client: api::ApiClient,
}

impl Scraper {
    pub fn new() -> Scraper {
        Scraper {
            api_client: api::ApiClient::new("https://httpbin.org/"),
        }
    }

    pub fn scrape_ip(&self) -> String {
        let res = self.api_client.get_ip().expect("Ip scraping failed");

        res.origin
    }
}
