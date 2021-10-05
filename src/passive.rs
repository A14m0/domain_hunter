// Contains all passive gathering techniques
use std::net::*;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;
use trust_dns_proto::rr::{
    record_type::RecordType,
    RData
};
use url::Url;

use crate::active::subdomains_from_request;
use crate::stats::Stats;
use crate::log::{
    log,
    LogType
};

/// fetches all of the records from a specific domain 
fn lookup_all(domain: String, resolver: trust_dns_resolver::Resolver) -> Vec<RData>{
    // ANY -> Any cached records
    // fetch the any records
    let records =  resolver.lookup(domain.clone(), RecordType::ANY).unwrap();
    println!("{} Record(s) found", records.iter().count());
    let mut ret: Vec<trust_dns_proto::rr::RData> = Vec::new();

    // print each record and its type
    for record in records {
        ret.push(record.clone());
        println!("{} ({}): {}", domain, record.to_record_type(),record);
    }

    ret
}

pub async fn passive_test() {
    // Construct a new Resolver with default configuration options
    let resolver = Resolver::new(
                            //ResolverConfig::cloudflare_tls(), 
                            ResolverConfig::default(),
                            ResolverOpts::default()
                        ).unwrap();

    // On Unix/Posix systems, this will read the /etc/resolv.conf
    // let mut resolver = Resolver::from_system_conf().unwrap();

    // Lookup the IP addresses associated with a name.
    let response = resolver.lookup_ip("www.example.com.").unwrap();

    // There can be many addresses associated with the name,
    //  this can return IPv4 and/or IPv6 addresses
    let address = response.iter().next().expect("no addresses returned!");
    println!("{}", address);
    if address.is_ipv4() {
        assert_eq!(address, IpAddr::V4(Ipv4Addr::new(93, 184, 216, 34)));
    } else {
        assert_eq!(address, IpAddr::V6(Ipv6Addr::new(0x2606, 0x2800, 0x220, 0x1, 0x248, 0x1893, 0x25c8, 0x1946)));
    }

    let records = lookup_all("www.example.com".to_string(), resolver);



}



/// tries to find domains through DuckDuck-dorking 
async fn dork_domains(domain: Url) -> Vec<Url> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:92.0) Gecko/20100101 Firefox/92.0")
        .build().unwrap();
    let mut stats = Stats{urls:0};
    
    // create our request call
    let call = format!(
        "https://duckduckgo.com/?q=site%3A*+{}",
        domain.host_str().unwrap()
    );

    // make the call
    let res = match client.get(call).send().await {
        Ok(a) => a,
        Err(e) => {
            log(LogType::LogCrit, format!("Failed to find domain: {}", e));
            std::process::exit(1);
        }
    };
   

    subdomains_from_request(domain, res,&mut stats).await
}