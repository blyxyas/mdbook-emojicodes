use anyhow::Result;
use emojis::get_by_shortcode;
use lazy_static::lazy_static;
use mdbook::book::Book;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use regex::Regex;

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
                lazy_static! {
                    static ref RE: Regex = Regex::new(r":(.*?):").unwrap();
                };
				let mut known_emojis: Vec<&str> = Vec::new();

                let buf = ch.content.clone();
				
                for capt in RE.find_iter(&buf) {
					if known_emojis.contains(&capt.as_str()) {
						continue;
					}
					// ch.content = ch.content.replace(capt.as_str(), );
                    if let Some(emoji) = get_by_shortcode(&buf[capt.start() + 1..capt.end() - 1]) {
                        ch.content = ch.content
                            .replace(capt.as_str(), emoji.as_str());
							known_emojis.push(capt.as_str());
                    } else if let Some(emoji) = get_custom_emoji(&buf[capt.start() + 1..capt.end() - 1], &custom_emojis) {
						ch.content = ch.content
						.replace(capt.as_str(), emoji.as_str());
						known_emojis.push(capt.as_str());
					}
                }
            };
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

	for file in cd.read_dir().expect("Couldn't read directory `custom_emojis`").filter_map(|x| x.ok()) {
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