use crate::message::Message;
use tokio_postgres::{Client, Error, Row};

#[derive(Clone, Copy)]
pub struct Reader<'a> {
  client: &'a Client,
}

impl<'a> Reader<'a> {
  pub fn new(client: &'a Client) -> Self {
    Self { client }
  }

  pub async fn get_category_messages(
    &self,
    category_name: &str,
    position: Option<&i64>,
    batch_size: Option<&i64>,
    correlation: Option<&String>,
    consumer_group_member: Option<&i64>,
    consumer_group_size: Option<&i64>,
    condition: Option<&str>,
  ) -> Result<Vec<Message>, Error> {
    let rows = self
      .client
      .query(
        "select * from get_category_messages(
          $1::varchar,
          $2::bigint,
          $3::bigint,
          $4::varchar,
          $5::bigint,
          $6::bigint,
          $7::varchar
        )",
        &[
          &category_name,
          &position,
          &batch_size,
          &correlation,
          &consumer_group_member,
          &consumer_group_size,
          &condition,
        ],
      )
      .await?;

    let messages = rows.iter().map(|x| x.into()).collect();
    Ok(messages)
  }

  pub async fn get_stream_messages(
    &self,
    stream_name: &str,
    position: Option<&i64>,
    batch_size: Option<&i64>,
    condition: Option<&str>,
  ) -> Result<Vec<Message>, Error> {
    let rows = self
      .client
      .query(
        "select * from get_stream_messages(
          $1::varchar,
          $2::bigint,
          $3::bigint,
          $4::varchar
        )",
        &[&stream_name, &position, &batch_size, &condition],
      )
      .await?;

    let messages = rows.iter().map(|x| x.into()).collect();
    Ok(messages)
  }

  pub async fn get_last_stream_message(&self, stream_name: &str) -> Result<Message, Error> {
    let row: Row = self
      .client
      .query_one(
        "select * from get_last_stream_message($1::varchar)",
        &[&stream_name],
      )
      .await?;

    Ok(row.into())
  }
}
