use modsecurity::{ModSecurity, Rules, Transaction};
use crate::config::WAF;

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
pub fn initial(waf: &Option<WAF>) -> Option<Transaction> {
    if let Some(v) = waf {
        if !v.mod_security {
            return None;
        }
    } else {
        return None;
    }

    let ms = ModSecurity::default();

    let mut rules = Rules::new();
    rules.add_plain(r#"
    SecRuleEngine On

    SecRule REQUEST_URI "@rx admin" "id:1,phase:1,deny,status:401"
"#).expect("Failed to add rules");

    let mut transaction = ms
        .transaction_builder()
        .with_rules(&rules)
        .build()
        .expect("Error building transaction");

    Some(transaction)
}