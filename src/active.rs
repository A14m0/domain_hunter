// Contains all active gathering techniques
use reqwest;
use url::Url;
use std::io::Read;

use crate::stats::Stats;
use crate::log::{
    log,
    LogType
};

/// defines the maximum recursion depth for looking for domains
const MAX_DEPTH: u16 = 5;

/// finds unique subdomains given a base host and list of domains
fn find_subdomains(host: Url, domains: Vec<Url>) -> Vec<Url> {
    let mut ret: Vec<&str> = Vec::new();
    // loop through each domain
    for domain in domains {
        // filter off the domains that aren't
        let domain_host_str = match domain.host_str() {
            Some(a) => a.clone(),
            None => {
                log(
                    LogType::LogWarn, 
                    format!("Invalid URL detected. Skipping")
                );
                return Vec::new();
            }
        };
        let domain_host_str = domain_host_str.clone();
        let host_str = match host.host_str() {
            Some(a) => a,
            None => {
                log(
                    LogType::LogWarn, 
                    format!("Invalid URL detected. Skipping")
                );
                return Vec::new();
            }
        };

        if let Some(_) = domain_host_str.find(host_str) {
            // now make sure we don't have that domain already
            if !ret.contains(&domain_host_str) {
                ret.push(domain_host_str);
            }
            
        }
    }

    // convert unique subdomains to URLs
    let mut uniq: Vec<Url> = Vec::new();
    for u in ret {
        uniq.push(Url::parse(u).unwrap());
    }
    
    uniq
}

/// returns all urls in the body of the request
fn search_body(body: String) -> Vec<Url> {
    let body = body.split('>');
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
            //let line = &line[idx..];
        }
    }
    ret
}

/// Searches the domain for subdomains
pub fn subdomains(
    url: Url, 
    depth: u16, 
    client: Option<reqwest::blocking::Client>,
    stats: Option<Stats>
) -> Vec<Url> {
    //println!("URL={}, Depth={}", url, depth);
    // see if we are at the max depth to stop looking for crap
    if depth > MAX_DEPTH {
        return Vec::new();
    }
    let mut ret = Vec::new();

    // set up our client, and make a base request to the domain
    let r: reqwest::blocking::Client;
    if let Some(x) = client {
        r = x;
    } else {
        r = reqwest::blocking::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:92.0) Gecko/20100101 Firefox/92.0")
            .build().unwrap();
    }
    let s: Stats;
    if let Some(x) = stats {
        s = x;
    } else {
        s = Stats {urls: 0};
    }


    let mut res = match r.get(url.clone()).send() {
        Ok(a) => a,
        Err(e) => {
            log(LogType::LogCrit, format!("Failed to find domain: {}", e));
            std::process::exit(1);
        }
    };
    let mut body = String::new();
    match res.read_to_string(&mut body) {
        Ok(_) => (),
        Err(e) => {
            log(
                LogType::LogErr, 
                format!("Failed to parse body: {}", e)
            );
            log(
                LogType::LogWarn, 
                format!("Skipping URL...")
            );
            return Vec::new();
        }
    };
    
    //println!("Status: {}", res.status());
    //println!("Headers:\n{:#?}", res.headers());
    //println!("Body:\n{}", body);

    // search the contents for URLS and get them
    let t = search_body(body);
    //for s in t.clone() {
        //println!("{}",s);
    //}
    log(
        LogType::LogInfo,
        format!(
            "Processed {} URLs (Depth {}) {}",
            s.urls,
            depth,
            url
        )
    );
    /*log(
        LogType::LogInfo,
        format!(
            "DEPTH {}\tFound {} links in body. Pruning...", 
            depth, t.len()
        )
    );*/
    for sub in find_subdomains(url, t) {
        ret.append(&mut subdomains(sub, 
            depth + 1, 
            Some(r.clone()),
            Some(s)
        ));
    }
    //ret.append(&mut find_subdomains(url, t));
    ret
}