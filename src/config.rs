use std::{env, path::Path};

#[derive(Clone)]
pub struct Config {
    directory: String,
    languages: Vec<String>,
}

impl Config {
    /// Create a new configuration.
    ///
    /// # Example
    /// ```rust, ignore
    /// use languages_rs::Config;
    ///
    /// let config: Config = match Config::new("languages", vec!["en"]) {
    ///     Ok(config) => config,
    ///     Err(e) => {
    ///         eprintln!("Error: {}", e);
    ///         return;
    ///     },
    /// };
    /// ```
    pub fn new(directory: &str, languages: Vec<&str>) -> anyhow::Result<Self> {
        let path = Path::new(&env::current_dir()?).join(directory);
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

        Ok(Self {
            directory: path.display().to_string(),
            languages: languages.iter().map(|e| String::from(*e)).collect(),
        })
    }

    /// Get the default configuration.
    ///
    /// # Default
    /// ```json
    /// {
    ///     "directory": "languages/",
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
