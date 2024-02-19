use lazy_static::lazy_static;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

lazy_static! {
    static ref confusables: HashMap<std::string::String, std::string::String> =
        serde_json::from_str::<HashMap<String, String>>(include_str!("../confusables.json"))
            .expect("failed to parse confusables.json");
}

fn asciify(text: &str) -> String {
    // TODO: convert to return and optional String
    // that way, when we don't convert anything, we can return None
    // this makes it cheaper
    let mut converted = false;
    let mut arr = Vec::new();
    for char in UnicodeSegmentation::graphemes(text, true) {
        if let Some(replacement) = confusables.get(char) {
            arr.push(replacement.as_str());
            converted = true;
        } else {
            arr.push(char);
        }
    }
    if converted {
        arr.join("")
    } else {
        text.to_string()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::asciify("𝓗𝓮𝓵𝓵𝓸, 𝓦𝓸𝓻𝓵𝓭!"), "Hello, World!");
    }
}

/*
const conversions = ImmutableMap<string, string>(confusables);

export const asciify = (text: string): string => {
  const arr = [];
  let converted = false;
  for (const char of text) {
    if (conversions.has(char)) {
      arr.push(conversions.get(char));
      converted = true;
    } else {
      arr.push(char);
    }
  }
  if (converted) {
    return arr.join("");
  }

  return text;
};

*/
