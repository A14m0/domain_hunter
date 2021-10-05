// Contains all passive gathering techniques
use std::net::*;
use trust_dns_resolver::AsyncResolver;
use trust_dns_resolver::config::*;
use trust_dns_proto::rr::{
    record_type::RecordType,
    RData
};
use trust_dns_proto::xfer::dns_request::DnsRequestOptions;
use url::Url;

use crate::active::subdomains_from_request;
use crate::stats::Stats;
use crate::log::{
    log,
    LogType
};

/// fetches all of the records from a specific domain 
async fn lookup_all(domain: String) -> Vec<RData>{
    let resolver = AsyncResolver::tokio(
        //ResolverConfig::cloudflare_tls(), 
        ResolverConfig::default(),
        ResolverOpts::default()
    ).unwrap();
    let opts = DnsRequestOptions{expects_multiple_responses: false, use_edns: true};


    // ANY -> Any cached records
    // fetch the any records
    let records =  resolver.lookup(domain.clone(), RecordType::ANY, opts).await.unwrap();
    println!("{} Record(s) found", records.iter().count());
    let mut ret: Vec<trust_dns_proto::rr::RData> = Vec::new();

    // print each record and its type
    for record in records {
        ret.push(record.clone());
        println!("{} ({}): {}", domain, record.to_record_type(),record);
    }

    ret
}

pub async fn passive_test(domain: Url) {
    // Construct a new Resolver with default configuration options
    let resolver = AsyncResolver::tokio(
                            //ResolverConfig::cloudflare_tls(), 
                            ResolverConfig::default(),
                            ResolverOpts::default()
                        ).unwrap();

    // On Unix/Posix systems, this will read the /etc/resolv.conf
    // let mut resolver = Resolver::from_system_conf().unwrap();

    // Lookup the IP addresses associated with a name.
    println!("{}", domain);
    let response = resolver.lookup_ip(domain.host_str().unwrap()).await.unwrap();

    // There can be many addresses associated with the name,
    //  this can return IPv4 and/or IPv6 addresses
    let address = response.iter().next().expect("no addresses returned!");
    println!("{}", address);
    
    let records = lookup_all(domain.host_str().unwrap().to_string()).await;
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