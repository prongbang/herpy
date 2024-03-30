use std::collections::HashMap;
use url::form_urlencoded;

pub mod handler;

fn parse_query_string(query_string: &str) -> HashMap<String, String> {
    let mut query_params = HashMap::new();

    if query_string.is_empty() {
        return query_params;
    }

    let query_pairs = form_urlencoded::parse(query_string.as_bytes());
    for (key, value) in query_pairs {
        query_params.insert(key.into_owned(), value.into_owned());
    }

    query_params
}