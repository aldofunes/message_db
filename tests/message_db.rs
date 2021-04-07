mod test_setup;

use message_db::{Handlers, MessageDb};
use std::collections::HashMap;
use test_context::{futures, test_context};
use test_setup::TestSetup;
use uuid::Uuid;

#[test_context(TestSetup)]
#[tokio::test]
async fn it_should_publish_and_read_a_message(ctx: &mut TestSetup) {
  let message_db = MessageDb::new(&ctx.client);

  let id = Uuid::new_v4();
  let stream_name = String::from("test-79fe36d2-072c-451b-8d1a-6950a5658477");
  let message_type = String::from("TestEvent");
  let data = serde_json::from_str(r#"{"foo":"bar"}"#).unwrap();
  let metadata = serde_json::from_str(r#"{"baz":"qux"}"#).unwrap();

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
  let message_db = MessageDb::new(&ctx.client);

  let id = Uuid::new_v4();
  let stream_name = String::from("test-79fe36d2-072c-451b-8d1a-6950a5658477");
  let message_type = String::from("TestEvent");
  let data = serde_json::from_str(r#"{"foo":"bar"}"#).unwrap();
  let metadata = serde_json::from_str(r#"{"baz":"qux"}"#).unwrap();

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

  let mut handlers: Handlers = HashMap::new();
  handlers.insert(
    String::from("TestEvent"),
    Box::new(|message| println!("{:?}", message.id)),
  );

  let mut subscription = message_db.subscriber.subscribe(
    String::from("test"),
    handlers,
    String::from("test_subscriber"),
    Some(5),
    Some(1000),
    Some(10),
    None,
    None,
    None,
  );

  subscription.start().await;
}
