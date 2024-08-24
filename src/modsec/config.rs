use modsecurity::msc::ModSecurity;
use modsecurity::rules::Rules;

pub struct Config {
    pub enabled: bool,
    pub rules: Rules,
    pub mod_security: ModSecurity,
}