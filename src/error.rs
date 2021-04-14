use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
  InvalidValue(String),
  NoneError(String),
  Sql(sqlx::Error),
  SqlMigration(sqlx::migrate::MigrateError),
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Self::InvalidValue(msg) => write!(f, "invalid value found: ({})", msg),
      Self::NoneError(msg) => write!(f, "none value found: ({})", msg),
      Self::Sql(err) => write!(f, "sql command failed: ({})", &err.to_string()),
      Self::SqlMigration(err) => write!(f, "failed to run migrations: ({})", &err.to_string()),
    }
  }
}

impl std::error::Error for Error {}

impl From<sqlx::Error> for Error {
  fn from(error: sqlx::Error) -> Self {
    Error::Sql(error)
  }
}

impl From<sqlx::migrate::MigrateError> for Error {
  fn from(error: sqlx::migrate::MigrateError) -> Self {
    Error::SqlMigration(error)
  }
}
