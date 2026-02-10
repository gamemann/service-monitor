use std::collections::HashMap;

pub fn format_contents(contents: &str, fmt: HashMap<String, String>) -> String {
    let mut formatted = String::from(contents);
    for (key, value) in fmt {
        formatted = formatted.replace(&key, &value);
    }
    formatted
}
