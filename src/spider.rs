use reqwest;
use url::Url;
use regex::Regex;

use crate::common::{
    clear_dup_url,
    MAX_DEPTH
};
use crate::log::{
    log,
    LogType
};

/// extracts a link from the line
fn extract_link(line: &str, domain: Url) -> Result<Url, String> {
    if let Some(idx) = line.find("\"/") {
        // if we find an HTTP link, try to parse it and save it to the list
        if let Some(end_idx) = line[idx+1..].find("\"") {
            // make sure we actually have a URL
            match domain.join(&line[idx+1..idx+end_idx+1]) {
                Ok(a) => return Ok(a),
                Err(e) => {
                    log(LogType::LogWarn, 
                        format!(
                            "Illegal URL: {} + {} -> {}",
                            domain, 
                            &line[idx+1..idx+end_idx+1],
                            e
                        )
                    );
                    return Err(e.to_string())
                }
            }
        }
        //let line = &line[idx..];
    }
    Err("No URL Found".to_string())
}

/// checks body for links and returns them
fn check_for_links(domain: Url, body: String) -> Vec<Url> {
    // define our regex
    let re = Regex::new("\"/").unwrap();
    let mut ret: Vec<Url> = Vec::new();
    let body = body.split("<");
    // loop over each entry in the body
    for line in body {
        // check if the line contains a URL-looking thing
        if re.is_match(line) {
            // extract and append
            match extract_link(line, domain.clone()) {
                Ok(a) => ret.push(a),
                Err(_) => ()
            }
        }
    }
    
    // remove all dupes before moving on
    clear_dup_url(ret)
}

/// spiders a site for possible other links
pub async fn spider(start: Url) -> Vec<Url> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:92.0) Gecko/20100101 Firefox/92.0")
        .build().unwrap();
    let mut ret: Vec<Url> = Vec::new();
    let mut tmp: Vec<Url> = Vec::new();

    // get our initial url list
    let res = match client.get(start.clone()).send().await {
        Ok(a) => a,
        Err(e) => {
            log(LogType::LogCrit, format!("Failed to find domain: {}", e));
            std::process::exit(1);
        }
    };
    let mut current = check_for_links(start.clone(), res.text().await.unwrap());
    ret.append(&mut current.clone());

    log(LogType::LogInfo, format!("Running spider with MAX_DEPTH={}...", MAX_DEPTH));
    // go to MAX_DEPTH
    for _ in 0..MAX_DEPTH {
        for link in current.iter() {
            // get each link, appending the ones we found
            let res = match client.get(link.clone()).send().await {
                Ok(a) => a,
                Err(e) => {
                    log(LogType::LogCrit, format!("Failed to find domain: {}", e));
                    std::process::exit(1);
                }
            };
            tmp.append(&mut check_for_links(link.clone(), res.text().await.unwrap()));
        }

        log(LogType::LogInfo, format!("Found {} URLs", tmp.len()));
        
        // add the new ones, swap vectors, and continue
        ret.append(&mut clear_dup_url(tmp.clone()));
        current = clear_dup_url(tmp.clone());
        tmp.clear();

    }

    // Done. print all the URLs we found
    log(
        LogType::LogInfo,
        format!("Complete -- Found {} URLs --", ret.len())
    );
    
    for v in ret.iter() {
        println!("{}", v);
    }

    ret
}