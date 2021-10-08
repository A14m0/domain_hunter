use url::Url;

use crate::log::{
    log,
    LogType
};

/// defines the maximum recursion depth for looking for domains
pub const MAX_DEPTH: u16 = 2;

/// helper function that adds `https://` to the front of a URL
pub fn https(url: String) -> String {
    "https://".to_string() + &url
}

/// removes duplicate domains from a vector
pub fn clear_dupes(out: Vec<Url>) -> Vec<Url> {
    let mut ret: Vec<String> = Vec::new();

    // loop over each domain
    for v in out {
        let domain_host_str = match v.host_str() {
            Some(a) => a.to_string(),
            None => {
                log(
                    LogType::LogWarn, 
                    format!("Invalid URL detected. Skipping")
                );
                continue;
            }
        };
        // now make sure we don't have that domain already
        if !ret.contains(&domain_host_str) {
            ret.push(domain_host_str.clone());
        }   
    }

    // now convert them back to URLs
    let mut uniq: Vec<Url> = Vec::new();
    for u in ret {
        match Url::parse(&https(u.clone())[..]) {
            Ok(a) => uniq.push(a),
            Err(e) => {
                log(
                    LogType::LogErr, 
                    format!(
                        "Failed to parse URL {}: {}",
                        u,
                        e
                    )
                )
            }
        };
    }

    uniq
}


/// removes duplicate urls from a vector
pub fn clear_dup_url(out: Vec<Url>) -> Vec<Url> {
    let mut ret: Vec<Url> = Vec::new();

    // loop over each domain
    for v in out {
        // now make sure we don't have that domain already
        if !ret.contains(&v) {
            ret.push(v.clone());
        }   
    }

    ret
}
