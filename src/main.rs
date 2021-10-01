use reqwest;
use url::Url;
use std::io::Read;

mod log;
use crate::log::{
    log,
    LogType
};

/// finds unique subdomains given a base host and list of domains
fn find_subdomains(host: Url, domains: Vec<Url>) -> Vec<Url> {
    let mut ret = Vec::new();
    // loop through each domain
    for domain in domains {
        // filter off the domains that aren't
        if let Some(_) = domain.host_str().unwrap().find(host.host_str().unwrap()) {
            ret.push(domain);
        }
    }
    
    ret
}

/// returns all urls in the body of the request
fn search_body(body: String) -> Vec<Url> {
    let body = body.split('\n');
    let mut ret = Vec::new();
    // loop over each line in the response, looking for any HTTP links
    for line in body{
        /* for later
            let this_document = Url::parse("http://servo.github.io/rust-url/url/index.html")?;
            let css_url = this_document.join("../main.css")?;
            assert_eq!(css_url.as_str(), "http://servo.github.io/rust-url/main.css");
        */
        if let Some(idx) = line.find("http") {
            // if we find an HTTP link, try to parse it and save it to the list
            if let Some(end_idx) = line[idx..].find("\"") {
                // make sure we actually have a URL
                match Url::parse(&line[idx..idx+end_idx]) {
                    Ok(a) => ret.push(a),
                    Err(_) => ()
                }
            }
        }
    }
    ret
}

fn main() {
    // define our base domain
    // TODO: Set this up to be CLI-defined
    let base_domain = Url::parse("https://junkgarbage.org").unwrap();
    
    // set up our client, and make a base request to the domain
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:92.0) Gecko/20100101 Firefox/92.0")
        .build().unwrap();
    let mut res = match client.get(base_domain).send() {
        Ok(a) => a,
        Err(e) => {
            log(LogType::LogCrit, format!("Failed to find domain: {}", e));
            std::process::exit(1);
        }
    };
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    // search the contents for URLS and get them
    let t = search_body(body);
    for s in t {
        println!("{}",s);
    }
}
