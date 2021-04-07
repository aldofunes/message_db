mod test_setup;
mod utils;

use message_db::{Handlers, MessageDb};
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

  message_db
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
    .unwrap();

  let message = message_db
    .reader
    .get_last_stream_message(&stream_name)
    .await
    .unwrap();

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
  handlers.insert(
    String::from("TestEvent"),
    Box::new(|message| println!("{:?}", message.id)),
  );

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
