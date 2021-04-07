use tokio_postgres::{Client, Error};
use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct Writer<'a> {
  client: &'a Client,
}

impl<'a> Writer<'a> {
  pub fn new(client: &'a Client) -> Self {
    Self { client }
  }

  pub async fn write_message(
    &self,
    id: &Uuid,
    stream_name: &str,
    message_type: &str,
    data: &serde_json::Value,
    metadata: Option<&serde_json::Value>,
    expected_version: Option<&i64>,
  ) -> Result<i64, Error> {
    let row = self
      .client
      .query_one(
        "select write_message(
          $1::uuid,
          $2::varchar,
          $3::varchar,
          $4::jsonb,
          $5::jsonb,
          $6::bigint
        )",
        &[
          &id,
          &stream_name,
          &message_type,
          &data,
          &metadata,
          &expected_version,
        ],
      )
      .await?;

    let position = row.get("write_message");
    Ok(position)
  }
}
