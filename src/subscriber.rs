use crate::message::Message;
use crate::reader::Reader;
use crate::writer::Writer;
use serde_json::json;
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct Subscription<'a> {
  reader: Reader<'a>,
  writer: Writer<'a>,

  current_position: i64,
  messages_since_last_position_write: i64,

  subscriber_stream_name: String,
  stream_name: String,

  position_update_interval: i64,
  origin_stream_name: Option<String>,
  consumer_group_member: Option<i64>,
  consumer_group_size: Option<i64>,
}

impl<'a> Subscription<'a> {
  pub fn new(
    reader: Reader<'a>,
    writer: Writer<'a>,

    stream_name: String,
    subscriber_id: String,

    position_update_interval: Option<i64>,
    origin_stream_name: Option<String>,
    consumer_group_member: Option<i64>,
    consumer_group_size: Option<i64>,
  ) -> Self {
    let current_position = 0;
    let messages_since_last_position_write = 0;
    let subscriber_stream_name = format!("subscriber-{}", subscriber_id);

    Self {
      reader,
      writer,
      stream_name,

      origin_stream_name,
      position_update_interval: position_update_interval.unwrap_or(100),

      current_position,
      messages_since_last_position_write,
      subscriber_stream_name,
      consumer_group_member,
      consumer_group_size,
    }
  }

  pub async fn load_position(&mut self) {
    match self
      .reader
      .get_last_stream_message(&self.subscriber_stream_name)
      .await
    {
      Ok(message) => {
        self.current_position = message.data["position"].as_i64().unwrap_or(0);
      }
      Err(_) => {}
    }
  }

  pub async fn poll(&mut self, messages_per_tick: Option<i64>) -> Result<Vec<Message>, Error> {
    log::trace!("polling");

    self
      .reader
      .get_category_messages(
        &self.stream_name,
        Some(self.current_position + 1),
        messages_per_tick,
        self.origin_stream_name.as_deref(),
        self.consumer_group_member,
        self.consumer_group_size,
        None,
      )
      .await
  }

  pub async fn update_read_position(&mut self, position: i64) {
    self.current_position = position;
    self.messages_since_last_position_write += 1;

    if self.messages_since_last_position_write % self.position_update_interval == 0 {
      self.write_position(position).await;
    }
  }

  pub async fn write_position(&self, position: i64) {
    let data = json!({ "position": position });

    self
      .writer
      .write_message(
        Uuid::new_v4(),
        &self.subscriber_stream_name,
        &"Read",
        data,
        None,
        None,
      )
      .await
      .unwrap();
  }
}

pub struct Subscriber<'a> {
  reader: Reader<'a>,
  writer: Writer<'a>,
}

impl<'a> Subscriber<'a> {
  pub fn new(pool: &'a PgPool) -> Self {
    let reader = Reader::new(&pool);
    let writer = Writer::new(&pool);

    Self { reader, writer }
  }

  pub fn subscribe(
    &self,
    stream_name: String,
    subscriber_id: String,

    position_update_interval: Option<i64>,
    origin_stream_name: Option<String>,
    consumer_group_member: Option<i64>,
    consumer_group_size: Option<i64>,
  ) -> Subscription {
    Subscription::new(
      self.reader,
      self.writer,
      stream_name,
      subscriber_id,
      position_update_interval,
      origin_stream_name,
      consumer_group_member,
      consumer_group_size,
    )
  }
}
