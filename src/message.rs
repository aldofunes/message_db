use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
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
