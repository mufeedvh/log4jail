use warp::{
    hyper::body::Bytes,
    Filter,
    Rejection,
    http::{Response, HeaderMap}
};

use warp_reverse_proxy::{errors, extract_request_data_filter, proxy_to_and_forward_response};

use regex::Regex;
use once_cell::sync::Lazy;

use std::collections::HashMap;

static LOG4J_PAYLOAD_REGEXP: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)(\$|%24)(\{|%7b).*j.*n.*d.*i.*(:|%3a)").unwrap()
});

mod config;
use config::Config;

static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::get_config()
});

async fn reject_request() -> Result<http::Response<Bytes>, Rejection> {
    let response = Response::builder()
        .status(CONFIG.reject_response_status)
        .body(Bytes::from(CONFIG.reject_response_body.as_str()));

    response
        .map_err(errors::Error::HTTP)
        .map_err(warp::reject::custom)
}

#[tokio::main]
async fn main() {
    eprintln!("\n\t- ${{jndi:log4jail}} -\n");

    Config::generate_default();

    static PROXY_MAPPING: Lazy<HashMap<u16, std::string::String>> = Lazy::new(|| {
        Config::proxy_map()
    });

    // get host address octets
    let host_string: Vec<&str> = CONFIG.host.split(".").collect();

    let mut host: Vec<u8> = Vec::with_capacity(4);
    for octet in host_string {
        host.push(octet.parse::<u8>().unwrap())
    }

    let host = [host[0], host[1], host[2], host[3]];    

    for (port, target) in PROXY_MAPPING.iter() {        
        let request_filter = extract_request_data_filter();

        let app = warp::any()
            .and(request_filter)
            .and_then(move |path, query, method, headers: HeaderMap, body: Bytes| async move {
                for (k, v) in headers.iter() {
                    if LOG4J_PAYLOAD_REGEXP.is_match(k.as_str()) || LOG4J_PAYLOAD_REGEXP.is_match(v.to_str().unwrap()) {
                        return reject_request().await
                    }
                }

                if LOG4J_PAYLOAD_REGEXP.is_match(
                    std::str::from_utf8(&body).expect("error converting bytes to &str")
                ) {
                    return reject_request().await
                }     

                proxy_to_and_forward_response(
                    target.to_string(),
                    "".to_string(),
                    path,
                    query,
                    method,
                    headers,
                    body,
                ).await
            });

        tokio::spawn(async move {
            if CONFIG.enable_https {
                warp::serve(app)
                    .tls()
                    .cert_path(&CONFIG.tls_cert_path)
                    .key_path(&CONFIG.tls_key_path)
                    .run((host, *port)).await
            } else {
                warp::serve(app)
                    .run((host, *port)).await
            }
        });

        eprintln!("[-] Mapped port {} -> {}", port, target);
    }

    // keep running all proxy instances
    loop {}
}