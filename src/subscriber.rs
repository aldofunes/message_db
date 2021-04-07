use crate::message::Message;
use crate::reader::Reader;
use crate::writer::Writer;
use serde_json::json;
use std::{collections::HashMap, thread::sleep, time::Duration};
use uuid::Uuid;

pub type Callback = Box<(dyn Fn(&Message) -> () + 'static)>;
pub type Handlers = HashMap<String, Callback>;

pub struct Subscription<'a> {
  reader: Reader<'a>,
  writer: Writer<'a>,

  current_position: i64,
  messages_since_last_position_write: i64,
  keep_going: bool,

  subscriber_stream_name: String,
  stream_name: String,
  handlers: Handlers,

  messages_per_tick: i64,
  position_update_interval: i64,
  origin_stream_name: Option<String>,
  tick_interval_ms: i64,
  consumer_group_member: Option<i64>,
  consumer_group_size: Option<i64>,
}

impl<'a> Subscription<'a> {
  pub fn new(
    reader: Reader<'a>,
    writer: Writer<'a>,
    stream_name: String,
    handlers: Handlers,
    subscriber_id: String,

    messages_per_tick: Option<i64>,
    tick_interval_ms: Option<i64>,

    position_update_interval: Option<i64>,
    origin_stream_name: Option<String>,
    consumer_group_member: Option<i64>,
    consumer_group_size: Option<i64>,
  ) -> Self {
    let current_position = 0;
    let messages_since_last_position_write = 0;
    let keep_going = true;
    let subscriber_stream_name = format!("subscriber-{}", subscriber_id);

    Self {
      reader,
      writer,
      stream_name,
      handlers,

      origin_stream_name,
      messages_per_tick: messages_per_tick.unwrap_or(10),
      position_update_interval: position_update_interval.unwrap_or(100),
      tick_interval_ms: tick_interval_ms.unwrap_or(100),

      current_position,
      messages_since_last_position_write,
      keep_going,
      subscriber_stream_name,
      consumer_group_member,
      consumer_group_size,
    }
  }

  pub async fn start(&mut self) {
    log::info!("starting subscriber");
    self.load_position().await;

    while self.keep_going {
      let messages_processed = self.tick().await;

      if messages_processed == 0 {
        sleep(Duration::from_millis(self.tick_interval_ms as u64));
      }
    }
  }

  pub fn stop(&mut self) {
    log::info!("stopping subscriber");
    self.keep_going = false;
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

  pub async fn tick(&mut self) -> i64 {
    let messages = self.get_next_batch_of_messages().await;
    log::info!("tick");
    self.process_batch(messages).await
  }

  pub async fn get_next_batch_of_messages(&self) -> Vec<Message> {
    let messages = self
      .reader
      .get_category_messages(
        &self.stream_name,
        Some(&(self.current_position + 1)),
        Some(&self.messages_per_tick),
        self.origin_stream_name.as_ref(),
        self.consumer_group_member.as_ref(),
        self.consumer_group_size.as_ref(),
        None,
      )
      .await
      .unwrap();
    messages
  }

  pub async fn process_batch(&mut self, messages: Vec<Message>) -> i64 {
    let messages_count = messages.len() as i64;

    for message in messages {
      let position = message.global_position;
      self.handle_message(message).await;
      self.update_read_position(position).await;
    }

    messages_count
  }

  pub async fn handle_message(&self, message: Message) {
    match self.handlers.get(&message.r#type) {
      Some(handler) => handler(&message),
      None => (),
    }
  }

  pub async fn update_read_position(&mut self, position: i64) {
    self.current_position = position;
    self.messages_since_last_position_write += 1;

    if self.messages_since_last_position_write % self.position_update_interval == 0 {
      self.write_position(position).await;
    }
  }

  pub async fn write_position(&mut self, position: i64) {
    let data = json!({ "position": position });

    self
      .writer
      .write_message(
        &Uuid::new_v4(),
        &self.subscriber_stream_name,
        &"Read",
        &data,
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
  pub fn new(reader: Reader<'a>, writer: Writer<'a>) -> Self {
    Self { reader, writer }
  }

  pub fn subscribe(
    &self,
    stream_name: String,
    handlers: Handlers,
    subscriber_id: String,

    messages_per_tick: Option<i64>,
    tick_interval_ms: Option<i64>,

    position_update_interval: Option<i64>,
    origin_stream_name: Option<String>,

    consumer_group_member: Option<i64>,
    consumer_group_size: Option<i64>,
  ) -> Subscription {
    Subscription::new(
      self.reader,
      self.writer,
      stream_name,
      handlers,
      subscriber_id,
      messages_per_tick,
      tick_interval_ms,
      position_update_interval,
      origin_stream_name,
      consumer_group_member,
      consumer_group_size,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_run_the_closure() {
    let closure: Callback = Box::new(|message| {
      println!("{:?}", message);
    });

    let mut handlers = HashMap::new();

    handlers.insert("TestEvent", closure);
  }
}
