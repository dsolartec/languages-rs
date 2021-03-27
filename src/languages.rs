mod language_texts;

pub use language_texts::LanguageTexts;

use std::{fs::read_to_string, path::Path};

use crate::{Config, Value};

pub struct Languages {
    config: Config,
    langs: Vec<LanguageTexts>,
}

impl Languages {
    /// Create a new Languages object using a configuration.
    ///
    /// # Example
    /// ```rust, ignore
    /// use languages_rs::{Config, Languages};
    ///
    /// let config = Config::default().unwrap();
    /// let texts = Languages::new(&config);
    /// ```
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
            langs: Vec::new(),
        }
    }

    /// Get all texts of a specific language.
    ///
    /// # Example
    /// ```rust, ignore
    /// use languages_rs::{Config, Languages};
    ///
    /// let mut config = Config::default().unwrap();
    /// assert!(config.add_language(String::from("en")).is_ok());
    ///
    /// let mut texts = Languages::new(&config);
    ///
    /// let texts_en = texts.try_get_language("en");
    /// assert!(texts_en.is_ok());
    ///
    /// assert_eq!(texts_en.try_get_text("message"), Some(Value::String(String::from("Hi"))));
    /// ```
    pub fn try_get_language(&mut self, lang: &str) -> anyhow::Result<LanguageTexts> {
        // Check if the configuration has the lang.
        if !self.config.get_languages().contains(&String::from(lang)) {
            return Err(anyhow::Error::msg(format!(
                "Cannot find the `{}` lang.",
                lang
            )));
        }

        // Check if the language is in the cache and return it if exists.
        for lang_texts in self.langs.iter() {
            if lang_texts.get_language() == *lang {
                return Ok(lang_texts.clone());
            }
        }

        // Get the language texts file location.
        let path = Path::new(&self.config.get_directory()).join(format!(
            "{}{}",
            lang,
            if cfg!(feature = "with-json") { ".json" } else if cfg!(feature = "with-toml") { ".toml" } else { "" }
        ));

        // Check if the file exists.
        if !path.exists() {
            return Err(anyhow::Error::msg(format!(
                "Cannot find `{}` file.",
                path.display()
            )));
        } else if !path.is_file() {
            return Err(anyhow::Error::msg(format!(
                "The path `{}` is not a file.",
                path.display()
            )));
        }

        // Generate the language texts object for the file.
        let lang_texts = LanguageTexts::new(
            String::from(lang),
            Value::from_string(read_to_string(path)?)?,
        )?;

        // Add the language texts to the cache.
        self.langs.push(lang_texts.clone());

        Ok(lang_texts)
    }

    /// Get a text of a specific language.
    ///
    /// # Example
    /// ```rust, ignore
    /// use languages_rs::{Config, Languages};
    ///
    /// let mut config = Config::default().unwrap();
    /// assert!(config.add_language(String::from("en")).is_ok());
    ///
    /// let mut texts = Languages::new(&config);
    ///
    /// let message = texts.try_get_text_from_language("en", "message");
    /// assert!(message.is_ok());
    /// assert_eq!(message.unwrap(), Some(Value::String(String::from("Hi"))));
    /// ```
    pub fn try_get_text_from_language(
        &mut self,
        lang: &str,
        text: &str,
    ) -> anyhow::Result<Option<Value>> {
        Ok(self.try_get_language(lang)?.try_get_text(text))
    }
}
