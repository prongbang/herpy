use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};
use serde_yaml;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServiceConfig {
    pub path: String,
    pub target_service: String,
    pub target_port: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GatewayConfig {
    pub authorization_api_url: String,
    pub services: Vec<ServiceConfig>,
}

pub fn load_config(path: &str) -> GatewayConfig {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return serde_yaml::from_str(&contents).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let path = "config.test.yaml";
        let config = load_config(path);

        assert_eq!(config.authorization_api_url, "https://auth-service.local/authorization");
        assert_eq!(config.services.len(), 2);

        let service1 = &config.services[0];
        assert_eq!(service1.path, "/users");
        assert_eq!(service1.target_service, "https://user-service.local/users");
        assert_eq!(service1.target_port, "8080");

        let service2 = &config.services[1];
        assert_eq!(service2.path, "/orders");
        assert_eq!(service2.target_service, "https://order-service.local/orders");
        assert_eq!(service2.target_port, "8080");
    }
}