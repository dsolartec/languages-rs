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

use serde_json::Value as JSONValue;
use toml::Value as TOMLValue;

use crate::Format;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Value {
    fn from_json_value(value: JSONValue) -> anyhow::Result<Self> {
        if value.is_string() {
            return Ok(Self::String(String::from(value.as_str().unwrap())));
        } else if value.is_array() {
            return Ok(Self::Array(
                value
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|e| Self::from_json_value(e.clone()).expect("Invalid format."))
                    .collect(),
            ));
        } else if value.is_object() {
            let mut new_data: HashMap<String, Value> = HashMap::new();
            for (key, value) in value.as_object().unwrap().iter() {
                new_data.insert(key.clone(), Self::from_json_value(value.clone())?);
            }

            return Ok(Self::Object(new_data));
        }

        Err(anyhow::Error::msg(format!(
            "Cannot parse `{}` as a language text value.",
            value
        )))
    }

    pub fn from_toml_value(value: TOMLValue) -> anyhow::Result<Self> {
        if value.is_str() {
            return Ok(Self::String(String::from(value.as_str().unwrap())));
        } else if value.is_array() {
            return Ok(Self::Array(
                value
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|e| Self::from_toml_value(e.clone()).expect("Invalid format."))
                    .collect(),
            ));
        } else if value.is_table() {
            let mut new_data: HashMap<String, Value> = HashMap::new();
            for (key, value) in value.as_table().unwrap().iter() {
                new_data.insert(key.clone(), Self::from_toml_value(value.clone())?);
            }

            return Ok(Self::Object(new_data));
        }

        Err(anyhow::Error::msg(format!(
            "Cannot parse `{}` as a language text value.",
            value
        )))
    }

    /// Get the texts from a JSON string or TOML string.
    ///
    /// # Example JSON
    /// ```rust
    /// use languages_rs::{Format, Value};
    ///
    /// let value = Value::from_string(String::from("\"Hi\""), Format::Json);
    /// assert!(value.is_ok());
    /// assert_eq!(value.unwrap(), Value::String(String::from("Hi")));
    /// ```
    ///
    /// # Example TOML
    /// ```rust
    /// use std::collections::HashMap;
    ///
    /// use languages_rs::{Format, Value};
    ///
    /// let value = Value::from_string(String::from("hi = \"Hi\""), Format::Toml);
    /// assert!(value.is_ok());
    ///
    /// let mut data: HashMap<String, Value> = HashMap::new();
    /// data.insert(String::from("hi"), Value::String(String::from("Hi")));
    ///
    /// assert_eq!(value.unwrap(), Value::Object(data));
    /// ```
    pub fn from_string(text: String, format: Format) -> anyhow::Result<Self> {
        if format.is_json() {
            Self::from_json_value(serde_json::from_str(&text)?)
        } else {
            Self::from_toml_value(text.parse()?)
        }
    }

    /// Check if the current value is a string.
    ///
    /// # Example
    /// ```rust
    /// use languages_rs::{Format, Value};
    ///
    /// let value = Value::from_string(String::from("\"Hi\""), Format::Json);
    /// assert!(value.is_ok());
    /// assert!(value.unwrap().is_string());
    /// ```
    ///
    /// # Example TOML
    /// ```rust
    /// use languages_rs::{Format, Value};
    ///
    /// let value = Value::from_string(String::from("hi = \"Hi\""), Format::Toml);
    /// assert!(value.is_ok());
    ///
    /// let table = value.unwrap().get_object();
    /// assert!(table.is_some());
    ///
    /// let table = table.unwrap();
    /// let text = table.get("hi");
    /// assert!(text.is_some());
    /// assert!(text.unwrap().is_string());
    /// ```
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    /// Get the string value.
    ///
    /// # Example JSON
    /// ```rust
    /// use languages_rs::{Format, Value};
    ///
    /// let value = Value::from_string(String::from("\"Hi\""), Format::Json);
    /// assert!(value.is_ok());
    /// assert_eq!(value.unwrap().get_string(), Some(String::from("Hi")));
    /// ```
    ///
    /// # Example TOML
    /// ```rust
    /// use languages_rs::{Format, Value};
    ///
    /// let value = Value::from_string(String::from("hi = \"Hi\""), Format::Toml);
    /// assert!(value.is_ok());
    ///
    /// let table = value.unwrap().get_object();
    /// assert!(table.is_some());
    ///
    /// let table = table.unwrap();
    /// let text = table.get("hi");
    /// assert!(text.is_some());
    /// assert_eq!(text.unwrap().get_string(), Some(String::from("Hi")));
    /// ```
    pub fn get_string(&self) -> Option<String> {
        match self {
            Self::String(value) => Some(value.clone()),
            _ => None,
        }
    }

    /// Check if the current value is an array.
    ///
    /// # Example JSON
    /// ```rust
    /// use languages_rs::{Format, Value};
    ///
    /// let value = Value::from_string(String::from("[\"1\", \"2\"]"), Format::Json);
    /// assert!(value.is_ok());
    /// assert!(value.unwrap().is_array());
    /// ```
    ///
    /// # Example TOML
    /// ```rust
    /// use languages_rs::{Format, Value};
    ///
    /// let value = Value::from_string(String::from("numbers = [\"1\", \"2\"]"), Format::Toml);
    /// assert!(value.is_ok());
    ///
    /// let table = value.unwrap().get_object();
    /// assert!(table.is_some());
    ///
    /// let table = table.unwrap();
    /// let values = table.get("numbers");
    /// assert!(values.is_some());
    /// assert!(values.unwrap().is_array());
    /// ```
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    /// Get the array value.
    ///
    /// # Example JSON
    /// ```rust
    /// use languages_rs::{Format, Value};
    ///
    /// let value = Value::from_string(String::from("[\"1\", \"2\"]"), Format::Json);
    /// assert!(value.is_ok());
    /// assert_eq!(value.unwrap().get_array(), Some(vec![Value::String(String::from("1")), Value::String(String::from("2"))]));
    /// ```
    ///
    /// # Example TOML
    /// ```rust
    /// use languages_rs::{Format, Value};
    ///
    /// let value = Value::from_string(String::from("numbers = [\"1\", \"2\"]"), Format::Toml);
    /// assert!(value.is_ok());
    ///
    /// let table = value.unwrap().get_object();
    /// assert!(table.is_some());
    ///
    /// let table = table.unwrap();
    /// let values = table.get("numbers");
    /// assert!(values.is_some());
    /// assert_eq!(values.unwrap().get_array(), Some(vec![Value::String(String::from("1")), Value::String(String::from("2"))]));
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
    /// use languages_rs::{Format, Value};
    ///
    /// let value = Value::from_string(String::from("{\"home\":{\"title\":\"Home page\"}}"), Format::Json);
    /// assert!(value.is_ok());
    /// assert!(value.unwrap().is_object());
    /// ```
    ///
    /// # Example TOML
    /// ```rust
    /// use languages_rs::{Format, Value};
    ///
    /// let value = Value::from_string(String::from("[home]\r\ntitle = \"Home page\""), Format::Toml);
    /// assert!(value.is_ok());
    /// assert!(value.unwrap().is_object());
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
    /// use languages_rs::{Format, Value};
    ///
    /// let value = Value::from_string(String::from("{ \"title\": \"Home page\" }"), Format::Json);
    /// assert!(value.is_ok());
    ///
    /// let mut data: HashMap<String, Value> = HashMap::new();
    /// data.insert(String::from("title"), Value::String(String::from("Home page")));
    ///
    /// assert_eq!(value.unwrap().get_object(), Some(data));
    /// ```
    ///
    /// # Example TOML
    /// ```rust
    /// use std::collections::HashMap;
    ///
    /// use languages_rs::{Format, Value};
    ///
    /// let value = Value::from_string(String::from("title = \"Home page\""), Format::Toml);
    /// assert!(value.is_ok());
    ///
    /// let mut data: HashMap<String, Value> = HashMap::new();
    /// data.insert(String::from("title"), Value::String(String::from("Home page")));
    ///
    /// assert_eq!(value.unwrap().get_object(), Some(data));
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
