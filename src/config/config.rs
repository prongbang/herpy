use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayConfig {
    pub port: Option<u16>,
    pub authorization: HashMap<String, Authorization>,
    pub services: Vec<Service>,
    pub services_map: Option<HashMap<String, Service>>,
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
    pub authorization: Option<String>,
}

pub fn load_config(path: &str) -> GatewayConfig {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut conf: GatewayConfig = serde_yaml::from_str(&contents).unwrap();
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
        let path = "config.test.yaml";
        let config = load_config(path);

        // assert_eq!(config.authorization_api_url, "https://auth-service.local/authorization");
        // assert_eq!(config.services.len(), 2);
        //
        // let service1 = &config.services[0];
        // assert_eq!(service1.path, "/users");
        // assert_eq!(service1.target_service, "https://user-service.local/users");
        // assert_eq!(service1.target_port, "8080");
        //
        // let service2 = &config.services[1];
        // assert_eq!(service2.path, "/orders");
        // assert_eq!(service2.target_service, "https://order-service.local/orders");
        // assert_eq!(service2.target_port, "8080");
    }
}