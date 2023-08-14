mod api;
mod scraper;

fn main() {
    let scraper = scraper::Scraper::new();

    println!("obtained IP {:?}", scraper.scrape_ip());
}
