use env_logger;
use log::{debug, error, warn};
use serde_json;
use std::fs;
use {once_cell::sync::Lazy, regex::Regex};

fn read_file_to_dict() -> Result<std::collections::HashMap<char, char>, std::io::Error> {
    let string = fs::read_to_string("confusablesSummary.txt")?;
    let lines = string.lines();

    // match # followed by optional whitespace then a letter then anything
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"#\s*[a-zA-Z]+").unwrap());
    let mut dict = std::collections::HashMap::new();
    for line in lines.skip(9) {
        if !RE.is_match(line) {
            continue;
        }
        debug!("line: {}", line);
        debug!("{}\nescaped: {}", line, line.escape_unicode());
        let components: Vec<&str> = line.split_whitespace().collect();
        debug!("{:?}", components);
        if components.len() < 3 {
            continue;
        }

        for component in components[2..].iter() {
            debug!(
                "component: {}, unicode: {}, is_ascii: {}",
                component,
                component.escape_unicode(),
                component.is_ascii(),
            );
            let key = component.chars().next().unwrap();
            let value = components[1].chars().next().unwrap();
            debug!("key: {}, value: {}", key, value);
            if key.is_ascii() {
                // Never replace an ascii character with a non-ascii character
                // this is triggered by lines such as the confusable: "rn" -> "m"
                dict.insert(key, key);
            } else {
                dict.insert(key, value);
            }
        }
    }

    for (key, value) in dict.iter() {
        if !value.is_ascii() {
            warn!(
                "found non-ascii mapping:\n{} -> {}\n{} -> {}",
                key,
                value,
                key.escape_unicode(),
                value.escape_unicode()
            );
        }
    }
    Ok(dict)
}

fn main() {
    env_logger::init();
    let result = read_file_to_dict();
    match result {
        Ok(dict) => {
            let file = fs::File::create("confusables.json").unwrap();
            serde_json::to_writer(file, &dict).unwrap();
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    }
}
