//! The Value enum, a loosely typed way of representing any valid language text value.
//!
//! # Valid language texts
//! Language texts only can be in JSON or TOML format.
//!
//! ## JSON
//! ```json
//! {
//!     "hello_world": "Hello, world!",
//!     "pages": {
//!         "home": {
//!             "title": "Home page",
//!             "description": "This is the home page."
//!         },
//!     },
//!     "data": [
//!         "Message 1",
//!         "Message 2"
//!     ]
//! }
//! ```
//!
//! ## TOML
//! ```toml
//! hello_world = "Hello, world!"
//!
//! [pages]
//!     [pages.home]
//!     title = "Home page"
//!     description = "This is the home page."
//!
//! data = [
//!     "Message 1",
//!     "Message 2"
//! ]
//! ```

use std::{collections::HashMap, fmt};

#[cfg(feature = "with-json")]
use serde_json::Value as JsonValue;

#[cfg(feature = "with-toml")]
use toml::Value as TomlValue;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Value {
    #[cfg(feature = "with-json")]
    fn from_value(value: JsonValue) -> anyhow::Result<Self> {
        if value.is_string() {
            return Ok(Self::String(String::from(value.as_str().unwrap())));
        } else if value.is_array() {
            return Ok(Self::Array(
                value
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|e| Self::from_value(e.clone()).expect("Invalid format."))
                    .collect(),
            ));
        } else if value.is_object() {
            let mut new_data: HashMap<String, Value> = HashMap::new();
            for (key, value) in value.as_object().unwrap().iter() {
                new_data.insert(key.clone(), Self::from_value(value.clone())?);
            }

            return Ok(Self::Object(new_data));
        }

        Err(anyhow::Error::msg(format!(
            "Cannot parse `{}` as a language text value.",
            value
        )))
    }

    #[cfg(feature = "with-toml")]
    pub fn from_value(value: TomlValue) -> anyhow::Result<Self> {
        if value.is_str() {
            return Ok(Self::String(String::from(value.as_str().unwrap())));
        } else if value.is_array() {
            return Ok(Self::Array(
                value
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|e| Self::from_value(e.clone()).expect("Invalid format."))
                    .collect(),
            ));
        } else if value.is_table() {
            let mut new_data: HashMap<String, Value> = HashMap::new();
            for (key, value) in value.as_table().unwrap().iter() {
                new_data.insert(key.clone(), Self::from_value(value.clone())?);
            }

            return Ok(Self::Object(new_data));
        }

        Err(anyhow::Error::msg(format!(
            "Cannot parse `{}` as a language text value.",
            value
        )))
    }

    /// Get the texts from a JSON string.
    ///
    /// # Example
    /// ```rust
    /// use languages_rs::Value;
    ///
    /// fn main() {
    ///     let value = Value::from_string(String::from("\"Hi\""));
    ///     assert!(value.is_ok());
    ///     assert_eq!(value.unwrap(), Value::String(String::from("Hi")));
    /// }
    /// ```
    #[cfg(feature = "with-json")]
    pub fn from_string(text: String) -> anyhow::Result<Self> {
        Self::from_value(serde_json::from_str(&text)?)
    }

    /// Get the texts from a JSON string or TOML string.
    ///
    /// # Example
    /// ```rust
    /// use languages_rs::Value;
    ///
    /// fn main() {
    ///     use std::collections::HashMap;
    ///
    ///     let value = Value::from_string(String::from("hi = \"Hi\""));
    ///     assert!(value.is_ok());
    ///
    ///     let mut data: HashMap<String, Value> = HashMap::new();
    ///     data.insert(String::from("hi"), Value::String(String::from("Hi")));
    ///
    ///     assert_eq!(value.unwrap(), Value::Object(data));
    /// }
    /// ```
    #[cfg(feature = "with-toml")]
    pub fn from_string(text: String) -> anyhow::Result<Self> {
        Self::from_value(text.parse()?)
    }

    #[cfg(all(not(feature = "with-json"), not(feature = "with-toml")))]
    pub fn from_string(_text: String) -> anyhow::Result<Self> {
        Err(anyhow::Error::msg("You must define the parse feature."))
    }

    /// Check if the current value is a string.
    ///
    /// # Example
    /// ```rust
    /// use languages_rs::Value;
    ///
    /// #[cfg(feature = "with-json")]
    /// fn main() {
    ///     let value = Value::from_string(String::from("\"Hi\""));
    ///     assert!(value.is_ok());
    ///     assert!(value.unwrap().is_string());
    /// }
    ///
    /// #[cfg(feature = "with-toml")]
    /// fn main() {
    ///     let value = Value::from_string(String::from("hi = \"Hi\""));
    ///     assert!(value.is_ok());
    ///
    ///     let table = value.unwrap().get_object();
    ///     assert!(table.is_some());
    ///
    ///     let table = table.unwrap();
    ///     let text = table.get("hi");
    ///     assert!(text.is_some());
    ///     assert!(text.unwrap().is_string());
    /// }
    ///
    /// #[cfg(all(not(feature = "with-json"), not(feature = "with-toml")))]
    /// fn main() {}
    /// ```
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    /// Get the string value.
    ///
    /// # Example
    /// ```rust
    /// use languages_rs::Value;
    ///
    /// #[cfg(feature = "with-json")]
    /// fn main() {
    ///     let value = Value::from_string(String::from("\"Hi\""));
    ///     assert!(value.is_ok());
    ///     assert_eq!(value.unwrap().get_string(), Some(String::from("Hi")));
    /// }
    ///
    /// #[cfg(feature = "with-toml")]
    /// fn main() {
    ///     let value = Value::from_string(String::from("hi = \"Hi\""));
    ///     assert!(value.is_ok());
    ///
    ///     let table = value.unwrap().get_object();
    ///     assert!(table.is_some());
    ///
    ///     let table = table.unwrap();
    ///     let text = table.get("hi");
    ///     assert!(text.is_some());
    ///     assert_eq!(text.unwrap().get_string(), Some(String::from("Hi")));
    /// }
    ///
    /// #[cfg(all(not(feature = "with-json"), not(feature = "with-toml")))]
    /// fn main() {}
    /// ```
    pub fn get_string(&self) -> Option<String> {
        match self {
            Self::String(value) => Some(value.clone()),
            _ => None,
        }
    }

    /// Check if the current value is an array.
    ///
    /// # Example
    /// ```rust
    /// use languages_rs::Value;
    ///
    /// #[cfg(feature = "with-json")]
    /// fn main() {
    ///     let value = Value::from_string(String::from("[\"1\", \"2\"]"));
    ///     assert!(value.is_ok());
    ///     assert!(value.unwrap().is_array());
    /// }
    ///
    /// #[cfg(feature = "with-toml")]
    /// fn main() {
    ///     let value = Value::from_string(String::from("numbers = [\"1\", \"2\"]"));
    ///     assert!(value.is_ok());
    ///
    ///     let table = value.unwrap().get_object();
    ///     assert!(table.is_some());
    ///
    ///     let table = table.unwrap();
    ///     let values = table.get("numbers");
    ///     assert!(values.is_some());
    ///     assert!(values.unwrap().is_array());
    /// }
    ///
    /// #[cfg(all(not(feature = "with-json"), not(feature = "with-toml")))]
    /// fn main() {}
    /// ```
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    /// Get the array value.
    ///
    /// # Example
    /// ```rust
    /// use languages_rs::Value;
    ///
    /// #[cfg(feature = "with-json")]
    /// fn main() {
    ///     let value = Value::from_string(String::from("[\"1\", \"2\"]"));
    ///     assert!(value.is_ok());
    ///     assert_eq!(
    ///         value.unwrap().get_array(),
    ///         Some(vec![Value::String(String::from("1")), Value::String(String::from("2"))]),
    ///     );
    /// }
    ///
    /// #[cfg(feature = "with-toml")]
    /// fn main() {
    ///     let value = Value::from_string(String::from("numbers = [\"1\", \"2\"]"));
    ///     assert!(value.is_ok());
    ///
    ///     let table = value.unwrap().get_object();
    ///     assert!(table.is_some());
    ///
    ///     let table = table.unwrap();
    ///     let values = table.get("numbers");
    ///     assert!(values.is_some());
    ///     assert_eq!(
    ///         values.unwrap().get_array(),
    ///         Some(vec![Value::String(String::from("1")), Value::String(String::from("2"))]),
    ///     );
    /// }
    ///
    /// #[cfg(all(not(feature = "with-json"), not(feature = "with-toml")))]
    /// fn main() {}
    /// ```
    pub fn get_array(&self) -> Option<Vec<Value>> {
        match self {
            Self::Array(data) => Some(data.clone()),
            _ => None,
        }
    }

    /// Check if the current value is an object.
    ///
    /// # Example JSON
    /// ```rust
    /// use languages_rs::Value;
    ///
    /// #[cfg(feature = "with-json")]
    /// fn main() {
    ///     let value = Value::from_string(String::from("{\"home\":{\"title\":\"Home page\"}}"));
    ///     assert!(value.is_ok());
    ///     assert!(value.unwrap().is_object());
    /// }
    ///
    /// #[cfg(feature = "with-toml")]
    /// fn main() {
    ///     let value = Value::from_string(String::from("[home]\r\ntitle = \"Home page\""));
    ///     assert!(value.is_ok());
    ///     assert!(value.unwrap().is_object());
    /// }
    ///
    /// #[cfg(all(not(feature = "with-json"), not(feature = "with-toml")))]
    /// fn main() {}
    /// ```
    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    /// Get the object value.
    ///
    /// # Example JSON
    /// ```rust
    /// use std::collections::HashMap;
    ///
    /// use languages_rs::Value;
    ///
    /// #[cfg(feature = "with-json")]
    /// fn main() {
    ///     let value = Value::from_string(String::from("{ \"title\": \"Home page\" }"));
    ///     assert!(value.is_ok());
    ///
    ///     let mut data: HashMap<String, Value> = HashMap::new();
    ///     data.insert(String::from("title"), Value::String(String::from("Home page")));
    ///
    ///     assert_eq!(value.unwrap().get_object(), Some(data));
    /// }
    ///
    /// #[cfg(feature = "with-toml")]
    /// fn main() {
    ///     let value = Value::from_string(String::from("title = \"Home page\""));
    ///     assert!(value.is_ok());
    ///
    ///     let mut data: HashMap<String, Value> = HashMap::new();
    ///     data.insert(String::from("title"), Value::String(String::from("Home page")));
    ///
    ///     assert_eq!(value.unwrap().get_object(), Some(data));
    /// }
    ///
    /// #[cfg(all(not(feature = "with-json"), not(feature = "with-toml")))]
    /// fn main() {}
    /// ```
    pub fn get_object(&self) -> Option<HashMap<String, Value>> {
        match self {
            Self::Object(data) => Some(data.clone()),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::Array(value) => write!(
                f,
                "[{}]",
                value
                    .iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Object(value) => {
                write!(
                    f,
                    "{{ {} }}",
                    value
                        .iter()
                        .map(|(key, value)| format!("{}: {}", key, value))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }
    }
}
