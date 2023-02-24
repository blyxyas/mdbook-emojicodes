<h1 align="center">mdbook-emojicodes</h1>

---

`mdbook-emojicodes` is a MDBook preprocessor to replace your emojicodes (e.g. `:cat:`) to emojis. No more copy-pasting!

## 📦 Installation

### Using crates.io

```
$	cargo install mdbook-emojicodes
```

### Manual installation

#### Clone the repo

```
$	git clone https://github.com/blyxyas/mdbook-emojicodes
```

#### Build & Install the preprocessor

```bash
$	cd mdbook-emojicodes;
	cargo install --path .
```

## ❓ Usage

Write this in your `book.toml`:

```toml
[preprocessor.emojicodes]
```

Now, **✨ It's ready to use! ✨**.

You can use emojis by writing an emojicode in your files.

### Example

```md
<!-- my_chapter.md -->

# My :cat: cat journey

I love cats :cat: and dogs :dog:, I have two, one's gray, like a raccoon :raccoon:, and the other one is black, like the night :night_with_stars:.
```

This will render to:

```md
<!-- my_chapter.md -->

# My 🐱 cat journey

I love cats 🐱 and dogs 🐶, I have two, one's gray, like a raccoon 🦝 and the other one is black, like the night 🌃
```

---

Now, when you run `mdbook build`, all your emojis will be converted.

## 🎡 Custom Emojis

You can use your own emojis as `.svg` files. Just create a directory called `custom_emojis` in `src`; there you can include your custom emojis, like this:

```
.
├── book.toml
└── src
    ├── chapter_1.md
    ├── custom_emojis
    │   └── myemoji.svg
    └── SUMMARY.md
```

```md
# Chapter 1
:myemoji:
```

These emojis will be automatically sized to the correct size.

---

#### Stargazers

[![Stargazers repo roster for @blyxyas/mdbook-emojicodes](https://reporoster.com/stars/blyxyas/mdbook-emojicodes)](https://github.com/blyxyas/mdbook-emojicodes/stargazers)

#### License

This software uses the **MIT License**. Check the file [LICENSE](https://github.com/blyxyas/mdbook-emojicodes/blob/master/LICENSE) for more details

