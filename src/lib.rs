//! # languages-rs
//!
//! An internationalization library for your applications.
//!
//! # Features
//! - `JSON` or `TOML` languages files.
//! - Only can use Objects, Arrays and Strings.
//! - Customize the languages directory.
//!
//! # JSON Language File
//! ```json
//! {
//!     "hello_world": "Hello, world!",
//!     "home": {
//!         "title": "Home page",
//!         "description": "This is the home page."
//!     },
//!     "data": {
//!         "messages": [
//!             "Message 1",
//!             "Message 2"
//!         ]
//!     ]
//! }
//! ```
//!
//! # TOML Language File
//! ```toml
//! hello_world = "Hello, world!"
//!
//! [home]
//! title = "Home page"
//! description = "This is the home page."
//!
//! [data]
//! messages = [
//!     "Message 1",
//!     "Message 2"
//! ]
//! ```
//!
//! # Basic Usage
//! `languages/en.json`
//! ```json
//! {
//!     "hello_world": "Hello world!"
//! }
//! ```
//!
//! `src/main.rs`
//! ```rust, ignore
//! use languages_rs::{Config, Languages, load, Value};
//!
//! fn main() -> Result<()> {
//!     let mut configuration: Config = Config::default().unwrap();
//!     configuration.add_language("en").unwrap();
//!
//!     // Load all default languages.
//!     let texts: Languages = load(configuration).unwrap();
//!
//!     // Get the English texts from `/languages/es.json`.
//!     let texts_en: LanguagesTexts = texts.try_get_language("en").unwrap();
//!
//!     // Get the `hello_world` text from English texts.
//!     let en_hello_world: Value = texts_en.try_get_text("hello_world").unwrap();
//!     assert!(en_hello_world.is_string());
//!
//!     // Other alternative to get the `hello_world` text from English texts.
//!     let en_hello_world_2: Value = texts.try_get_text_from_language("en", "hello_world").unwrap();
//!     assert!(en_hello_world_2.is_string());
//!
//!     assert_eq!(en_hello_world, en_hello_world_2);
//!     assert_eq!(en_hello_world.get_string(), en_hello_world_2.get_string());
//! }
//! ```

mod config;
mod languages;
mod value;

pub use config::{Config, Format};
pub use languages::{LanguageTexts, Languages};
pub use value::Value;

/// Load the languages of a configuration and return the `Languages` struct.
///
/// # Example
/// ```rust, ignore
/// use languages_rs::{Config, load};
///
/// let mut config = Config::default();
///
/// // Add `en` language to the configuration.
/// config.add_language("en").unwrap();
///
/// // This loads `languages/en.json` to the cache.
/// let texts = load(config);
/// ```
pub fn load(configuration: Config) -> anyhow::Result<Languages> {
    let mut languages = Languages::new(&configuration);

    for lang in configuration.get_languages().iter() {
        languages.try_get_language(lang)?;
    }

    Ok(languages)
}
