use crate::error::Error;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(FromRow)]
struct WriteMessage {
  write_message: i64,
}

pub struct Writer<'a> {
  pool: &'a PgPool,
}

impl<'a> Writer<'a> {
  pub fn new(pool: &'a PgPool) -> Self {
    Self { pool }
  }

  pub async fn write_message(
    &self,
    id: Uuid,
    stream_name: &str,
    message_type: &str,
    data: serde_json::Value,
    metadata: Option<serde_json::Value>,
    expected_version: Option<i64>,
  ) -> Result<i64, Error> {
    let row: WriteMessage = sqlx::query_as(
      "select message_store.write_message(
        $1::varchar,
        $2::varchar,
        $3::varchar,
        $4::jsonb,
        $5::jsonb,
        $6::bigint
      )",
    )
    .bind(id)
    .bind(stream_name)
    .bind(message_type)
    .bind(data)
    .bind(metadata)
    .bind(expected_version)
    .fetch_one(self.pool)
    .await?;

    Ok(row.write_message)
  }
}
