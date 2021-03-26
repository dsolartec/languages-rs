# languages-rs

An internationalization library for Rust.

## Installation

```toml
[dependencies]
languages-rs = "0.1.0"
```

## Basic Usage
`languages/en.json`
```json
{
    "hello_world": "Hello world!"
}
```

`src/main.rs`
```rust
use languages_rs::{Config, Languages, load, Value};

fn main() -> Result<()> {
    let mut configuration: Config = Config::default().unwrap();
    configuration.add_language("en").unwrap();

    // Load all default languages.
    let texts: Languages = load(configuration).unwrap();

    // Get the English texts from `/languages/en.json`.
    let texts_en: LanguagesTexts = texts.try_get_language("en").unwrap();

    // Get the `hello_world` text from English texts.
    let en_hello_world: Value = texts_en.try_get_text("hello_world").unwrap();
    assert!(en_hello_world.is_string());

    // Another alternative to get the `hello_world` text from English texts is:
    let en_hello_world_2: Value = texts.try_get_text_from_language("en", "hello_world").unwrap();
    assert!(en_hello_world_2.is_string());

    assert_eq!(en_hello_world, en_hello_world_2);
    assert_eq!(en_hello_world.get_string(), en_hello_world_2.get_string());
}
```

## Testing

```console
$ cargo test
```

## Authors

- [@danielsolartech](https://github.com/danielsolartech) - Initial project

## Changelog

View the lastest repository changes in the [CHANGELOG.md](./CHANGELOG.md) file.

## Copyright

License: MIT

Read file [LICENSE](./LICENSE) for more information.
