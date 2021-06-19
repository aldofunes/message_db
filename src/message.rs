use chrono::NaiveDateTime;
use sqlx::{postgres::PgRow, Error, FromRow, Row};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug)]
pub struct Message {
  pub id: Uuid,
  pub stream_name: String,
  pub r#type: String,
  pub position: i64,
  pub global_position: i64,
  pub data: serde_json::Value,
  pub metadata: Option<serde_json::Value>,
  pub time: NaiveDateTime,
}

impl<'r> FromRow<'r, PgRow> for Message {
  fn from_row(row: &'r PgRow) -> Result<Self, Error> {
    let data: String = row.try_get("data")?;
    let metadata: Option<String> = row.try_get("metadata")?;

    Ok(Message {
      id: Uuid::from_str(row.try_get("id")?).expect("id is not valid UUID"),
      stream_name: row.try_get("stream_name")?,
      r#type: row.try_get("type")?,
      position: row.try_get("position")?,
      global_position: row.try_get("global_position")?,
      data: serde_json::from_str(&data).expect("data is not valid JSON"),
      metadata: match metadata {
        Some(meta) => Some(serde_json::from_str(&meta).expect("metadata is not valid JSON")),
        None => None,
      },
      time: row.try_get("time")?,
    })
  }
}
