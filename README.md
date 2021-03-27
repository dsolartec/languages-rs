# languages-rs

An internationalization library for Rust.

## Installation

Use with JSON language files:
```toml
[dependencies]
languages-rs = { version = "0.2.0", features = ["with-json"] }
```

Use with TOML language files:
```toml
[dependencies]
languages-rs = { version = "0.2.0", features = ["with-toml"] }
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

## Examples

-   [json_files](./examples/json_files.rs) - Languages files with JSON.
    ```console
    $ cargo run --example json_files --features "with-json"
    ```

-   [toml_files](./examples/toml_files.rs) - Languages files with TOML.
    ```console
    $ cargo run --example toml_files --features "with-toml"
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
