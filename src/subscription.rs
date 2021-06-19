use crate::error::Error;
use crate::message::Message;
use crate::reader::Reader;
use crate::writer::Writer;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

/// The subscription object
pub struct Subscription<'a> {
  reader: Reader<'a>,
  writer: Writer<'a>,

  current_position: i64,
  messages_since_last_position_write: i64,

  subscriber_stream_name: String,
  category: String,

  position_update_interval: i64,
  origin_stream_name: Option<String>,
  consumer_group_member: Option<i64>,
  consumer_group_size: Option<i64>,
}

impl<'a> Subscription<'a> {
  pub fn new(
    pool: &'a PgPool,

    category: String,
    subscriber_id: String,

    position_update_interval: Option<i64>,
    origin_stream_name: Option<String>,
    consumer_group_member: Option<i64>,
    consumer_group_size: Option<i64>,
  ) -> Self {
    let reader = Reader::new(&pool);
    let writer = Writer::new(&pool);

    let current_position = 0;
    let messages_since_last_position_write = 0;
    let subscriber_stream_name = format!(
      "subscriber-{}-{}-{}",
      category,
      subscriber_id,
      consumer_group_member.unwrap_or(1)
    );

    Self {
      reader,
      writer,
      category,

      origin_stream_name,
      position_update_interval: position_update_interval.unwrap_or(100),

      current_position,
      messages_since_last_position_write,
      subscriber_stream_name,
      consumer_group_member,
      consumer_group_size,
    }
  }

  /// Fetch the current position
  pub async fn load_position(&mut self) -> Result<i64, Error> {
    match self
      .reader
      .get_last_stream_message(&self.subscriber_stream_name)
      .await?
    {
      Some(message) => {
        self.current_position = message.data["position"].as_i64().unwrap_or(0);
        Ok(self.current_position)
      }
      None => Ok(0),
    }
  }

  /// Get the next batch of messages for this subscription
  pub async fn poll(&mut self, messages_per_tick: Option<i64>) -> Result<Vec<Message>, Error> {
    log::trace!("polling");

    let messages = self
      .reader
      .get_category_messages(
        &self.category,
        Some(self.current_position + 1),
        messages_per_tick,
        self.origin_stream_name.as_deref(),
        self.consumer_group_member,
        self.consumer_group_size,
        None,
      )
      .await?;

    Ok(messages)
  }

  /// Update the current position of the subscriber
  pub async fn update_read_position(&mut self, position: i64) -> Result<(), Error> {
    self.current_position = position;
    self.messages_since_last_position_write += 1;

    if self.messages_since_last_position_write % self.position_update_interval == 0 {
      self.write_position(position).await?;
    }

    Ok(())
  }

  /// Persist the position of the subscriber
  pub async fn write_position(&self, position: i64) -> Result<(), Error> {
    let data = json!({ "position": position });
    let metadata = json!({ "category": self.category });

    self
      .writer
      .write_message(
        Uuid::new_v4(),
        &self.subscriber_stream_name,
        &"Read",
        data,
        Some(metadata),
        None,
      )
      .await?;

    Ok(())
  }
}
