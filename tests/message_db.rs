mod test_setup;
mod utils;

use message_db::{async_callback, Handlers, Message, MessageDb};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde_json::json;
use std::collections::HashMap;
use test_context::{futures, test_context};
use test_setup::TestSetup;
use utils::publish_test_message;
use uuid::Uuid;

#[test_context(TestSetup)]
#[tokio::test]
async fn it_should_publish_and_read_a_message(ctx: &mut TestSetup) {
  let message_db = MessageDb::new(&ctx.client);

  let id = Uuid::new_v4();
  let stream_name = String::from(format!("test-{}", Uuid::new_v4()));
  let message_type = String::from("TestEvent");
  let data = json!({ "foo": "bar" });
  let metadata = json!({ "baz": "qux" });

  match message_db
    .writer
    .write_message(
      &id,
      &stream_name,
      &message_type,
      &data,
      Some(&metadata),
      None,
    )
    .await
  {
    Ok(_) => log::info!("message written"),
    Err(e) => log::error!("message failed to be written {}", e),
  };

  let message = match message_db
    .reader
    .get_last_stream_message(&stream_name)
    .await
  {
    Ok(message) => message,
    Err(e) => {
      log::error!("failed to get last stream message: {}", e);
      panic!("failed to get last stream message");
    }
  };

  assert_eq!(message.id, id);
}

#[test_context(TestSetup)]
#[tokio::test]
async fn it_should_subscribe(ctx: &mut TestSetup) {
  log::info!("building MessageDb");
  let message_db = MessageDb::new(&ctx.client);

  log::info!("generating category name");
  let category: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(7)
    .map(char::from)
    .collect();

  let stream_id = Uuid::new_v4();
  let stream_name = format!("{}-{}", category, stream_id);

  // publish 100 messages
  log::info!("publishing messages");
  for _ in 0..100 {
    publish_test_message(&message_db, &stream_name).await;
  }

  log::info!("building handlers");
  let mut handlers: Handlers = HashMap::new();

  async fn callback(message: Message) {
    println!("{:?}", message.id);
  }
  handlers.insert(String::from("TestEvent"), async_callback!(callback));

  log::info!("building subscription");
  let mut subscription = message_db.subscriber.subscribe(
    category,
    handlers,
    String::from("test_subscriber"),
    Some(1000),
    Some(1000),
    Some(1000),
    None,
    None,
    None,
  );

  log::info!("will tick");
  let messages_processed = subscription.tick().await;

  assert_eq!(messages_processed, 100);
}
