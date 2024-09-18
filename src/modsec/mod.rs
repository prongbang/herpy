pub mod config;

use std::env;
use glob::glob;
use modsecurity::{ModSecurity, Rules};
use crate::config::WAF;
use crate::modsec::config::Config;

// ## ModSecurity
//
// ```
// Document:
// - https://github.com/owasp-modsecurity/ModSecurity/wiki/Reference-Manual-(v3.x)
// - https://github.com/owasp-modsecurity/ModSecurity/wiki/Compilation-recipes-for-v3.x#mac-osx-1013
//
// Requirement:
// - brew install flex bison zlib curl pcre libffi autoconf automake yajl pkg-config libtool ssdeep luarocks
// - brew install geoip
// - brew install doxygen
// - brew install libmaxminddb
// - brew install LMDB
// - brew install SSDEEP
// - brew install automake
// - brew install lua
// - brew install pcre
// - brew install libtool
//
// # Arbitrarily, create a directory to put things in
// - sudo mkdir -p /usr/local/modsecurity
// - sudo chown -R $(whoami) /usr/local/modsecurity
//
// Clone:
// - cd /usr/local/opt
// - mkdir ModSecurity
// - git clone https://github.com/SpiderLabs/ModSecurity && cd ModSecurity
// - git checkout -b v3/master origin/v3/master
//
// Build:
// - sh build.sh
// - git submodule init && git submodule update
// - ./configure
//
// Install:
// - make
// - sudo make install
// Output:
// - installed at /usr/local/modsecurity/lib/pkgconfig/modsecurity.pc
// - export PKG_CONFIG_PATH=/usr/local/modsecurity/lib/pkgconfig
// ```
pub fn initial(waf: &Option<WAF>) -> Config {
    let ms = ModSecurity::builder().with_log_callbacks().build();
    let mut rules = Rules::new();

    let mut enabled: bool = false;
    if let Some(v) = waf {
        enabled = v.mod_security;

        // # The order of file inclusion in your webserver configuration should always be:
        if enabled {
            // # 1. modsecurity.conf
            let mod_sec_conf = env::var("MOD_SECURITY_CONF").unwrap_or(String::from("etc/herpy/modsecurity/modsecurity.conf"));
            rules.add_file(mod_sec_conf).expect("Failed to add modsecurity.conf");

            // # 2. crs-setup.conf
            let crs_conf = env::var("MOD_SECURITY_CRS_CONF").unwrap_or(String::from("etc/herpy/modsecurity/crs-setup.conf"));
            rules.add_file(crs_conf).expect("Failed to add crs-setup.conf");

            // # 3. rules/*.conf (the CRS rule files)
            let rules_conf = env::var("MOD_SECURITY_RULES_CONF").unwrap_or(String::from("etc/herpy/modsecurity/rules/*.conf"));
            for entry in glob(rules_conf.as_str()).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        if path.is_file() {
                            let filepath = path.display().to_string();
                            rules.add_file(filepath.clone()).expect(format!("Failed to add {}", &filepath).as_str());
                        }
                    }
                    Err(e) => println!("Error reading file: {}", e),
                }
            }
        }
    }

    Config { enabled, rules, mod_security: ms }
}