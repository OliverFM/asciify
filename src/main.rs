// first thing todo is to parse the file into a dictionary mapping everything to the corresponding
// latin letter
// then I can make the asciify function

use serde_json::json;
use std::fs;
use {once_cell::sync::Lazy, regex::Regex};

fn read_file_to_dict() -> Result<std::collections::HashMap<char, char>, std::io::Error> {
    let string = fs::read_to_string("confusablesSummary.txt")?;
    // println!("string[0..100]: {:?}", &string[0..100]);
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
        if line.contains("𝘄") {
            should_print = true;
            println!("line: {}", line);
        }
        if should_print {
            println!("{}\nescaped: {}", line, line.escape_unicode());
        }
        let components: Vec<&str> = line.split_whitespace().collect(); // map(|s| s.trim()).collect();
        if should_print {
            println!("{:?}", components);
        }
        if components.len() < 3 {
            continue;
        }

        for component in components[2..].iter() {
            if should_print {
                println!(
                    "component: {}, unicode: {}, is_ascii: {}",
                    component,
                    component.escape_unicode(),
                    component.is_ascii(),
                );
            }
            dict.insert(
                component.chars().next().unwrap(),
                components[1].chars().next().unwrap(),
            );
        }
    }
    Ok(dict)
}

fn main() {
    let result = read_file_to_dict();
    match result {
        Ok(dict) => {
            // let json = json!(dict).to_string();
            let file = fs::File::create("confusables.json").unwrap();
            serde_json::to_writer(file, &dict).unwrap();
            // let json = serde_json::to_string_pretty(&dict).unwrap();
            //
            // println!("Success: {:?}", json);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
