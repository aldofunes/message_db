use chrono::NaiveDateTime;
use tokio_postgres::Row;
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

impl From<Row> for Message {
  fn from(row: Row) -> Self {
    Self {
      id: row.get("id"),
      stream_name: row.get("stream_name"),
      r#type: row.get("type"),
      position: row.get("position"),
      global_position: row.get("global_position"),
      data: row.get("data"),
      metadata: row.get("metadata"),
      time: row.get("time"),
    }
  }
}

impl From<&Row> for Message {
  fn from(row: &Row) -> Self {
    Self {
      id: row.get("id"),
      stream_name: row.get("stream_name"),
      r#type: row.get("type"),
      position: row.get("position"),
      global_position: row.get("global_position"),
      data: row.get("data"),
      metadata: row.get("metadata"),
      time: row.get("time"),
    }
  }
}
