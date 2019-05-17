use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
  pub host: String,
  pub database: String,
  pub user: String,
  pub password: String,
}

impl Settings {
  pub fn load_settings() -> Result<Settings, Box<Error>> {
    let file = std::fs::File::open("database.yml")?;
    let settings: Settings = serde_yaml::from_reader(file)?;

    Ok(settings)
  }

  pub fn connection_string(&self) -> String {
    return format!("postgres://{}:{}@{}/{}", self.user, self.password, self.host, self.database)
  }
}

