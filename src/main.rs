use reqwest;
use std::io::Read;

fn main() {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:92.0) Gecko/20100101 Firefox/92.0")
        .build().unwrap();
    let mut res = client.get("http://httpbin.org/user-agent").send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);
}
