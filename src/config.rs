use std::env;

pub struct Config {
  pub connection_uri: String,
}

impl Config {
  pub fn new() -> Self {
    let default_connection = "postgresql://postgres:postgres@localhost:5432/postgres";
    let connection_uri =
      env::var("MESSAGE_DB_CONNECTION_URI").unwrap_or(String::from(default_connection));

    Self { connection_uri }
  }
}
