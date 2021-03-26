use crate::Value;

#[derive(Clone)]
pub struct LanguageTexts {
    language: String,
    texts: Value,
}

impl LanguageTexts {
    /// Create a new language texts.
    ///
    /// # Example
    /// ```rust
    /// use std::collections::HashMap;
    ///
    /// use languages_rs::{LanguageTexts, Value};
    ///
    /// let mut data: HashMap<String, Value> = HashMap::new();
    /// data.insert(String::from("message"), Value::String(String::from("Hi")));
    ///
    /// let texts = LanguageTexts::new(String::from("en"), Value::Object(data));
    /// assert!(texts.is_ok());
    /// assert_eq!(texts.unwrap().try_get_text("message"), Some(Value::String(String::from("Hi"))));
    /// ```
    pub fn new(language: String, texts: Value) -> anyhow::Result<Self> {
        if !texts.is_object() {
            return Err(anyhow::Error::msg(format!("`{}` is not an object.", texts)));
        }

        Ok(Self { language, texts })
    }

    /// Get the language of the texts.
    ///
    /// # Example
    /// ```rust
    /// use std::collections::HashMap;
    ///
    /// use languages_rs::{LanguageTexts, Value};
    ///
    /// let texts = LanguageTexts::new(String::from("en"), Value::Object(HashMap::new()));
    /// assert!(texts.is_ok());
    /// assert_eq!(texts.unwrap().get_language(), String::from("en"));
    /// ```
    pub fn get_language(&self) -> String {
        self.language.clone()
    }

    /// Get a text value of a language.
    ///
    /// # Example
    /// ```rust
    /// use std::collections::HashMap;
    ///
    /// use languages_rs::{LanguageTexts, Value};
    ///
    /// let mut data: HashMap<String, Value> = HashMap::new();
    /// data.insert(String::from("message"), Value::String(String::from("Hi")));
    /// data.insert(String::from("message2"), Value::String(String::from("Hi 2")));
    ///
    /// let texts = LanguageTexts::new(String::from("en"), Value::Object(data));
    /// assert!(texts.is_ok());
    ///
    /// let texts = texts.unwrap();
    /// assert_eq!(texts.try_get_text("message"), Some(Value::String(String::from("Hi"))));
    /// assert_eq!(texts.try_get_text("message2"), Some(Value::String(String::from("Hi 2"))));
    /// ```
    pub fn try_get_text(&self, text: &str) -> Option<Value> {
        if self.texts.is_object() {
            if let Some(value) = self.texts.get_object().unwrap().get(&String::from(text)) {
                return Some(value.clone());
            }
        }

        None
    }
}
