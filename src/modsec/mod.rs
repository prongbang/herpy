pub mod config;

use glob::glob;
use modsecurity::{ModSecurity, Rules};

use crate::config::WAF;
use crate::modsec::config::Config;

// Document: https://github.com/owasp-modsecurity/ModSecurity/wiki/Reference-Manual-(v3.x)
// Requirement:
// - brew install automake
// - brew install lua
// - brew install pcre
// - git clone https://github.com/owasp-modsecurity/ModSecurity
// - ./build.sh
// - ./configure
// - make
// - sudo make install
// - installed at /usr/local/modsecurity/lib/pkgconfig/modsecurity.pc
// - export PKG_CONFIG_PATH=/usr/local/modsecurity/lib/pkgconfig
pub fn initial(waf: &Option<WAF>) -> Config {
    let ms = ModSecurity::builder().with_log_callbacks().build();
    let mut rules = Rules::new();

    let mut enabled: bool = false;
    if let Some(v) = waf {
        enabled = v.mod_security;

        // # The order of file inclusion in your webserver configuration should always be:
        if v.mod_security {
            // # 1. modsecurity.conf
            rules.add_file("/etc/herpy/modsecurity/modsecurity.conf").expect("Failed to add modsecurity.conf");

            // # 2. crs-setup.conf
            rules.add_file("/etc/herpy/modsecurity/crs-setup.conf").expect("Failed to add crs-setup.conf");

            // # 3. rules/*.conf (the CRS rule files)
            let pattern = "/etc/herpy/modsecurity/rules/*.conf";

            for entry in glob(pattern).expect("Failed to read glob pattern") {
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