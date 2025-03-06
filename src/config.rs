use config::{File, FileFormat};
use eyre::Report;
use regex::Regex;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Config {
    /// Global configuration options
    #[serde(rename = "DEFAULT")]
    pub default: Option<DefaultSection>,

    /// Database configuration
    #[serde(default)]
    pub database: DatabaseSection,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct DefaultSection {}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct DatabaseSection {
    pub connection: String,
}

impl DatabaseSection {
    pub fn get_connection(&self) -> String {
        if self.connection.contains("+") {
            let re = Regex::new(r"(?<type>\w+)\+(\w+)://").unwrap();
            return re.replace(&self.connection, "${type}://").to_string();
        }
        self.connection.clone()
    }
}

impl Config {
    pub fn new(path: PathBuf) -> Result<Self, Report> {
        let mut builder = config::Config::builder();

        if std::path::Path::new(&path).is_file() {
            builder = builder.add_source(File::from(path).format(FileFormat::Ini));
        }

        Ok(builder.build()?.try_deserialize()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_connection() {
        let sot = DatabaseSection {
            connection: "mysql://u:p@h".into(),
        };
        assert_eq!("mysql://u:p@h", sot.get_connection());
        let sot = DatabaseSection {
            connection: "mysql+driver://u:p@h".into(),
        };
        assert_eq!("mysql://u:p@h", sot.get_connection());
    }
}
