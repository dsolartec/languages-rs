use std::{env, path::Path};

#[derive(Clone, Copy)]
pub enum Format {
    JSON,
    TOML,
}

impl Format {
    /// Check if the format is JSON.
    pub fn is_json(&self) -> bool {
        matches!(self, &Format::JSON)
    }

    /// Check if the format is TOML.
    pub fn is_toml(&self) -> bool {
        matches!(self, &Format::TOML)
    }

    /// Get file extension for the format (`.toml` or `.json`).
    ///
    /// # Example
    /// ```rust
    /// use languages_rs::Format;
    ///
    /// assert_eq!(Format::JSON.get_file_extension(), ".json");
    /// assert_eq!(Format::TOML.get_file_extension(), ".toml");
    /// ```
    pub fn get_file_extension(&self) -> &str {
        match self {
            Self::JSON => ".json",
            Self::TOML => ".toml",
        }
    }
}

#[derive(Clone)]
pub struct Config {
    directory: String,
    format: Format,
    languages: Vec<String>,
}

impl Config {
    /// Get the default configuration.
    ///
    /// # Default
    /// ```json
    /// {
    ///     "directory": "languages/",
    ///     "format": "JSON",
    ///     "languages": []
    /// ```
    ///
    /// # Example
    /// ```rust, ignore
    /// use languages_rs::Config;
    ///
    /// let config: Config = match Config::default() {
    ///     Ok(config) => config,
    ///     Err(e) => {
    ///         eprintln!("Error: {}", e);
    ///         return;
    ///     },
    /// };
    /// ```
    pub fn default() -> anyhow::Result<Self> {
        let path = Path::new(&env::current_dir()?).join("languages");
        if !path.exists() {
            std::fs::create_dir(&path)?;
        } else if !path.is_dir() {
            return Err(anyhow::Error::msg(format!(
                "The path `{}` is not a directory.",
                path.display()
            )));
        }

        Ok(Self {
            directory: path.display().to_string(),
            format: Format::JSON,
            languages: Vec::new(),
        })
    }

    /// Get the languages directory.
    ///
    /// # Example
    /// ```rust, ignore
    /// use std::{env, path};
    ///
    /// use languages_rs::Config;
    ///
    /// let config = Config::default().unwrap();
    /// assert_eq!(
    ///     config.get_directory(),
    ///     format!(
    ///         "{}{}languages",
    ///         env::current_dir().unwrap().display(),
    ///         path::MAIN_SEPARATOR,
    ///     ),
    /// );
    /// ```
    pub fn get_directory(&self) -> String {
        self.directory.clone()
    }

    /// Change the languages directory.
    ///
    /// # Example
    /// ```rust, ignore
    /// use std::env;
    ///
    /// use languages_rs::Config;
    ///
    /// let mut config = Config::default().unwrap();
    /// assert!(config.set_directory("languages").is_ok());
    /// ```
    pub fn set_directory(&mut self, new_directory: &str) -> anyhow::Result<()> {
        let path = Path::new(&env::current_dir()?).join(new_directory);
        if !path.exists() {
            return Err(anyhow::Error::msg(format!(
                "Cannot find `{}` directory.",
                path.display()
            )));
        } else if !path.is_dir() {
            return Err(anyhow::Error::msg(format!(
                "The path `{}` is not a directory.",
                path.display()
            )));
        }

        self.directory = path.display().to_string();
        Ok(())
    }

    /// Get the format files.
    ///
    /// # Example
    /// ```rust, ignore
    /// use languages_rs::{Config, Format};
    ///
    /// let config = Config::default().unwrap();
    /// assert_eq!(config.get_format(), Format::JSON);
    /// ```
    pub fn get_format(&self) -> Format {
        self.format
    }

    /// Change the format files.
    ///
    /// # Example
    /// ```rust, ignore
    /// use languages_rs::{Config, Format};
    ///
    /// let mut config = Config::default().unwrap();
    /// assert!(config.get_format().is_json());
    ///
    /// config.change_format(Format::TOML);
    /// assert!(config.get_format().is_toml());
    /// ```
    pub fn change_format(&mut self, new_format: Format) {
        self.format = new_format;
    }

    /// Get the availables languages.
    ///
    /// # Example
    /// ```rust, ignore
    /// use languages_rs::Config;
    ///
    /// let config = Config::default().unwrap();
    /// assert_eq!(config.get_languages(), Vec::<String>::new());
    /// ```
    pub fn get_languages(&self) -> Vec<String> {
        self.languages.clone()
    }

    /// Add a new language to the languages list if it does not exist.
    ///
    /// # Example
    /// ```rust, ignore
    /// use languages_rs::Config;
    ///
    /// let mut config = Config::default().unwrap();
    /// assert_eq!(config.get_languages(), Vec::<String>::new());
    /// assert!(config.add_language(String::from("en")).is_ok());
    /// assert_eq!(config.get_languages(), vec![String::from("en")]);
    /// ```
    pub fn add_language(&mut self, language: String) -> anyhow::Result<()> {
        if self.languages.contains(&language) {
            return Err(anyhow::Error::msg(format!(
                "The language `{}` already exists.",
                language
            )));
        }

        self.languages.push(language);
        Ok(())
    }
}
