use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use serde_json::{Map, Value};

#[derive(Serialize, Deserialize)]
pub(crate) struct Config {
    pub host: String,
    pub enable_https: bool,
    pub tls_cert_path: String,
    pub tls_key_path: String,
    pub reject_response_status: u16,
    pub reject_response_body: String,
    pub proxy_mapping: Map<String, Value>
}

static CONFIG_FILE: &str = "log4jail.json";

impl Config {
    /// Default config file values
    pub fn default() -> Self {
        let mut default_proxy_mapping: Map<String, Value> = Map::default();
        
        default_proxy_mapping.insert("1337".into(), "http://127.0.0.1:8080/".into());
        default_proxy_mapping.insert("6969".into(), "http://127.0.0.1:9000/".into());

        Self {
            host: "127.0.0.1".into(),
            enable_https: false,
            tls_cert_path: "cert.pem".into(),
            tls_key_path: "key.rsa".into(),
            reject_response_status: 403,
            reject_response_body: "This request has been blocked.".into(),
            proxy_mapping: default_proxy_mapping
        }
    }

    /// Generate a boilerplate config file
    pub fn generate_default() {
        if !Path::new(CONFIG_FILE).exists() {
            let json_config = serde_json::to_string_pretty(&Self::default()).unwrap();
            let mut config_file = File::create(CONFIG_FILE).unwrap();

            config_file.write_all(json_config.as_bytes()).unwrap();

            eprintln!("[>] A default log4jail config has been generated, configure `log4jail.json` to start the proxy!\n");
            std::process::exit(0)
        }
    }

    /// Fetch config values from `log4jail.json`
    pub fn get_config() -> Self {
        let file = File::open(CONFIG_FILE).unwrap();

        let mut buf_reader = BufReader::new(file);
        let mut user_config = String::new();
        buf_reader.read_to_string(&mut user_config).unwrap();

        let config: Config = serde_json::from_str(&user_config).unwrap();
        config
    }


    /// Returns a standard HashMap of the proxy server mapping
    pub fn proxy_map() -> HashMap<u16, String> {
        let config = Self::get_config();

        let mut map: HashMap<u16, String> = HashMap::new();

        for (port, target) in config.proxy_mapping.iter() {
            map.insert(
                port.parse::<u16>().unwrap(),    
                target.as_str().unwrap().into(),
            );
        }

        map
    }
}