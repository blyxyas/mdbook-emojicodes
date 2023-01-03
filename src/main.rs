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
        book.for_each_mut(|section: &mut BookItem| {
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
                    };
                }
            };
        });
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}
