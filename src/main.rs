mod requests;

fn main() {
    let api_client = requests::ApiClient::new("https://httpbin.org/");

    let res = api_client.get_ip();
    match res {
        Ok(v) => println!("obtained IP {:?}", v["origin"]),
        Err(e) => println!("failed request: {e:?}"),
    }
}
