use std::io::{Read, Error};

pub fn http_request(url: &str) -> Result<String, Error> {
    let response = ureq::get(url).call();

    //println!("info: status: {}", res.status());

    let body = response.into_string();

    body
}

pub fn load_file(url: &str) -> Box<dyn Read> {
    let reader = ureq::get(&url)
        .timeout_connect(5_000)
        .timeout_read(1_000)
        .call()
        .into_reader();
    Box::new(reader)
}