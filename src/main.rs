// first thing todo is to parse the file into a dictionary mapping everything to the corresponding
// latin letter
// then I can make the asciify function

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
        println!("{}\nescaped: {}", line, line.escape_unicode());
        let components: Vec<&str> = line.split_whitespace().collect(); // map(|s| s.trim()).collect();
        println!("{:?}", components);
        if components.len() < 3 {
            continue;
        }

        for component in components[2..].iter() {
            println!("component: {}", component);
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
            println!("Success: {:?}", dict);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
