use crate::{entity::Entity, error::Error, event::Event, message::Message};
use sqlx::PgPool;
use std::convert::TryInto;

pub struct Reader<'a> {
  pool: &'a PgPool,
}

impl<'a> Reader<'a> {
  pub fn new(pool: &'a PgPool) -> Self {
    Self { pool }
  }

  pub async fn get_category_messages(
    &self,
    category_name: &str,
    position: Option<i64>,
    batch_size: Option<i64>,
    correlation: Option<&str>,
    consumer_group_member: Option<i64>,
    consumer_group_size: Option<i64>,
    condition: Option<&str>,
  ) -> Result<Vec<Message>, Error> {
    let messages = sqlx::query_as(
      "select
        id::uuid,
        stream_name::varchar,
        type::varchar,
        position::bigint,
        global_position::bigint,
        data::jsonb,
        metadata::jsonb,
        time::timestamp
      from message_store.get_category_messages(
        $1::varchar,
        $2::bigint,
        $3::bigint,
        $4::varchar,
        $5::bigint,
        $6::bigint,
        $7::varchar
      )",
    )
    .bind(category_name)
    .bind(position)
    .bind(batch_size)
    .bind(correlation)
    .bind(consumer_group_member)
    .bind(consumer_group_size)
    .bind(condition)
    .fetch_all(self.pool)
    .await?;

    Ok(messages)
  }

  pub async fn get_stream_messages(
    &self,
    stream_name: &str,
    position: Option<i64>,
    batch_size: Option<i64>,
    condition: Option<&str>,
  ) -> Result<Vec<Message>, Error> {
    let messages = sqlx::query_as(
      "select
        id::uuid,
        stream_name::varchar,
        type::varchar,
        position::bigint,
        global_position::bigint,
        data::jsonb,
        metadata::jsonb,
        time::timestamp
      from message_store.get_stream_messages(
        $1::varchar,
        $2::bigint,
        $3::bigint,
        $4::varchar
      )",
    )
    .bind(stream_name)
    .bind(position)
    .bind(batch_size)
    .bind(condition)
    .fetch_all(self.pool)
    .await?;

    Ok(messages)
  }

  pub async fn get_last_stream_message(&self, stream_name: &str) -> Result<Option<Message>, Error> {
    let message = sqlx::query_as(
      "select
        id::uuid,
        stream_name::varchar,
        type::varchar,
        position::bigint,
        global_position::bigint,
        data::jsonb,
        metadata::jsonb,
        time::timestamp
     from message_store.get_last_stream_message(
       $1::varchar
      )",
    )
    .bind(stream_name)
    .fetch_optional(self.pool)
    .await?;

    Ok(message)
  }

  pub async fn get_current_entity<E: Entity, V: Event<E>>(
    &self,
    entity_id: &str,
  ) -> Result<E, Error> {
    let stream_name = format!("{}-{}", E::stream_category(), entity_id);

    let messages = self
      .get_stream_messages(&stream_name, None, None, None)
      .await?;

    let mut entity = E::default();

    for message in messages {
      let event: V = match message.try_into() {
        Ok(value) => Ok(value),
        Err(_) => Err(Error::InvalidValue(String::from(
          "unable to convert message into event",
        ))),
      }?;
      event.apply(&mut entity);
    }

    Ok(entity)
  }
}
