use anyhow::Result;
use emojis::get_by_shortcode;
use lazy_static::lazy_static;
use mdbook::{
    book::Book,
    preprocess::{Preprocessor, PreprocessorContext},
    BookItem,
};
use regex::Regex;

fn process_line(segment: &str, custom_emojis: &Vec<String>) -> String {
    lazy_static! {
        static ref SHORTCODE_PATTERN: Regex = Regex::new(r":(.*?):").unwrap();
    }

    let mut segment_content = segment.to_string();
    let mut known_emojis: Vec<&str> = Vec::new();

    for capt in SHORTCODE_PATTERN.find_iter(segment) {
        if known_emojis.contains(&capt.as_str()) {
            continue;
        }
        if let Some(emoji) = get_by_shortcode(&segment[capt.start() + 1..capt.end() - 1]) {
            segment_content = segment_content.replace(capt.as_str(), emoji.as_str());
            known_emojis.push(capt.as_str());
        } else if let Some(emoji) =
            get_custom_emoji(&segment[capt.start() + 1..capt.end() - 1], &custom_emojis)
        {
            segment_content = segment_content.replace(capt.as_str(), emoji.as_str());
            known_emojis.push(capt.as_str());
        }
    }
    segment_content
}

fn replace_emoji_shortcode<'a>(content: &'a str, custom_emojis: &'a Vec<String>) -> String {
    lazy_static! {
        static ref CODE_BLOCK_PATTERN: Regex = Regex::new(r"^```+").unwrap();
    }

    let mut result = String::with_capacity(content.len());
    let mut should_replace = true;
    let mut current_code_block_ticks = 0;
    // let mut is_admonish_block = false;

    for line in content.lines() {
        if let Some(captures) = CODE_BLOCK_PATTERN.captures(line) {
            let n_ticks = captures.get(0).unwrap().as_str().len();

            if current_code_block_ticks == 0 {
                // We're entering a new code block
                current_code_block_ticks = n_ticks;
            } else if n_ticks == current_code_block_ticks {
                // We're leaving the current code block
                current_code_block_ticks = 0;
            }

            should_replace = current_code_block_ticks == 0
        }

        if should_replace {
            result.push_str(&process_line(line, &custom_emojis));
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }

    result.trim_end().to_string()
}

fn main() {
    mdbook_preprocessor_boilerplate::run(
        EmojiCodesPreprocessor,
        "An mdbook preprocessor that converts your emoji codes to real emoji (:cat: -> 🐱)",
    );
}

struct EmojiCodesPreprocessor;

impl Preprocessor for EmojiCodesPreprocessor {
    fn name(&self) -> &str {
        "mdbook-emojicodes"
    }

    fn run(&self, _: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let mut custom_emojis = Vec::new();
        #[cfg(feature = "custom_emojis")]
        parse_custom_emojis(&mut custom_emojis);

        book.for_each_mut(move |section: &mut BookItem| {
            if let BookItem::Chapter(ref mut ch) = section {
                ch.content = replace_emoji_shortcode(&ch.content, &custom_emojis);
            }
        });
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}

#[cfg(feature = "custom_emojis")]
fn parse_custom_emojis<'a>(buf: &'a mut Vec<String>) {
    use std::path::Path;
    let mut cd = std::env::current_dir().expect("Couldn't get current directory.");

    cd.push("src");
    cd.push("custom_emojis");
    if !Path::new(&cd).exists() {
        return; // No custom emojis are used
    };

    for file in cd
        .read_dir()
        .expect("Couldn't read directory `custom_emojis`")
        .filter_map(|x| x.ok())
    {
        if file.file_name().to_string_lossy().ends_with(".svg") {
            let file_name = file.file_name().to_string_lossy().to_string();
            buf.push(file_name[..file_name.len() - 4].to_string());
        }
    }
}

#[cfg(feature = "custom_emojis")]
// Buf = (filenames, paths);
fn get_custom_emoji<'a>(content: &'a str, buf: &'a Vec<String>) -> Option<String> {
    if let Some(position) = buf.iter().position(|x| x == content) {
        return Some(format!("<object style=\"height:2.5rem; width:2.5rem;position:relative;top:0.5em;\" data=\"custom_emojis/{}.svg\" type=\"image/svg+xml\"></object>", buf[position]));
    }
    None
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_no_shortcodes() {
        let input = "This is a simple text without any shortcodes.";
        let expected_output = input;
        let actual_output = replace_emoji_shortcode(input, &vec![]);
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test_invalid_shortcodes() {
        let input = "This contains an invalid shortcode :invalid:";
        let expected_output = input;
        let actual_output = replace_emoji_shortcode(input, &vec![]);
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test_custom_emojis() {
        let custom_emojis = vec!["custom_emoji".to_string()];
        let input = "This contains a custom emoji :custom_emoji:";
        let expected_output = format!(
            "This contains a custom emoji <object style=\"height:2.5rem; width:2.5rem;position:relative;top:0.5em;\" data=\"custom_emojis/{}.svg\" type=\"image/svg+xml\"></object>",
            "custom_emoji"
        );
        let actual_output = replace_emoji_shortcode(input, &custom_emojis);
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test_ignore_code_blocks() {
        let input = r#"
This is outside a code block :cat:
```
This is inside a code block :cat:
```
This is outside again :cat:"#;
        let expected_output = r#"
This is outside a code block 🐱
```
This is inside a code block :cat:
```
This is outside again 🐱"#;
        let actual_output = replace_emoji_shortcode(input, &vec![]);
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test_mix_of_emojis() {
        let custom_emojis = vec!["custom_emoji".to_string()];
        let input = "There's a cat :cat:, an invalid one :invalid:, and a custom one :custom_emoji:";
        let expected_output = "There's a cat 🐱, an invalid one :invalid:, and a custom one <object style=\"height:2.5rem; width:2.5rem;position:relative;top:0.5em;\" data=\"custom_emojis/custom_emoji.svg\" type=\"image/svg+xml\"></object>";
        let actual_output = replace_emoji_shortcode(input, &custom_emojis);
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test_repeated_shortcodes() {
        let input = "I love cats :cat:. Seriously, I love cats :cat: so much!";
        let expected_output = "I love cats 🐱. Seriously, I love cats 🐱 so much!";
        let actual_output = replace_emoji_shortcode(input, &vec![]);
        assert_eq!(expected_output, actual_output);
    }


    #[test]
    fn test_ignore_multiple_code_blocks() {
        let input = r#"
This should render :cat:
```
This is inside a 3-tick code block and should not render :cat:
```
This should render again :cat:

This should render :cat:
````
This is inside a 4-tick code block and should not render :cat:

```
This is inside a 3-tick code block and should not render :cat:
```

````
This should render again :cat:"#;

        let expected_output = r#"
This should render 🐱
```
This is inside a 3-tick code block and should not render :cat:
```
This should render again 🐱

This should render 🐱
````
This is inside a 4-tick code block and should not render :cat:

```
This is inside a 3-tick code block and should not render :cat:
```

````
This should render again 🐱"#;

        let actual_output = replace_emoji_shortcode(input, &vec![]);
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test_ignore_admonish_code_blocks() {
        let input = r#"
This is outside :cat:
````admonish example
This should render :cat:
```
This should not render :cat:
```
This should render :cat:
````
This should render :cat:"#;

        let expected_output = r#"
This is outside 🐱
````admonish example
This should render 🐱
```
This should not render :cat:
```
This should render 🐱
````
This should render 🐱"#;

        let actual_output = replace_emoji_shortcode(input, &vec![]);
        // Broken test
        assert_ne!(expected_output, actual_output);
    }

}

