use std::io::Error;

pub fn http_request(url: &str) -> Result<String, Error> {
    let response = ureq::get(url).call();

    //println!("info: status: {}", res.status());

    let body = response.into_string();

    body
}