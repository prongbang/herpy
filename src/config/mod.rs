use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "herpy.yaml")]
    config: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayConfig {
    pub metadata: Metadata,
    pub authorization: Option<HashMap<String, Authorization>>,
    pub services: Vec<Service>,
    pub services_map: Option<HashMap<String, Service>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authorization {
    pub host: String,
    pub path: String,
    pub method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub endpoint: String,
    pub method: String,
    pub backends: Vec<Backend>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Backend {
    pub host: String,
    pub path: String,
    pub method: String,
    pub timeout: Option<u64>,
    pub authorization: Option<String>,
}

pub fn load(args: Args) -> GatewayConfig {
    // Parse yml to struct
    let file = File::open(args.config).expect("Could not open file.");
    let mut conf: GatewayConfig = serde_yaml::from_reader(file).expect("Could not read values.");
    conf.initial();
    return conf;
}

impl GatewayConfig {
    pub fn initial(&mut self) {
        let mut service_map: HashMap<String, Service> = HashMap::new();
        for service in &self.services {
            service_map.insert(service.endpoint.clone(), service.clone());
        }
        self.services_map = Some(service_map);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let args = Args { config: "herpy.yaml".to_string() };
        let config = load(args);

        assert_eq!(config.services.len(), 3);
    }
}