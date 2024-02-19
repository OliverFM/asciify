// first thing todo is to parse the file into a dictionary mapping everything to the corresponding
// latin letter
// then I can make the asciify function

use env_logger;
use log::{debug, error};
use serde_json;
use std::fs;
use {once_cell::sync::Lazy, regex::Regex};

fn read_file_to_dict() -> Result<std::collections::HashMap<char, char>, std::io::Error> {
    let string = fs::read_to_string("confusablesSummary.txt")?;
    // debug!("string[0..100]: {:?}", &string[0..100]);
    let lines = string.lines();

    // match # followed by optional whitespace then a letter then anything
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"#\s*[a-zA-Z]+").unwrap());
    let mut dict = std::collections::HashMap::new();
    let mut count = 0;
    for line in lines {
        if !RE.is_match(line) {
            continue;
        }
        let mut should_print = false;
        debug!("line: {}", line);
        debug!("{}\nescaped: {}", line, line.escape_unicode());
        let components: Vec<&str> = line.split_whitespace().collect(); // map(|s| s.trim()).collect();
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
            dict.insert(
                component.chars().next().unwrap(),
                components[1].chars().next().unwrap(),
            );
        }
    }

    for (key, value) in dict.iter() {
        if !value.is_ascii() {
            debug!(
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
            // let json = json!(dict).to_string();
            let file = fs::File::create("confusables.json").unwrap();
            serde_json::to_writer(file, &dict).unwrap();
            // let json = serde_json::to_string_pretty(&dict).unwrap();
            //
            // debug!("Success: {:?}", json);
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    }
}
