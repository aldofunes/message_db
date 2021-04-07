mod test_setup;

use message_db::MessageDb;
use test_context::{futures, test_context};
use test_setup::TestSetup;
use uuid::Uuid;

async fn publish_test_message(message_db: &MessageDb<'_>, stream_id: &Uuid) {
  let id = Uuid::new_v4();
  let stream_name = String::from(format!("testReader-{}", stream_id));
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
}

#[test_context(TestSetup)]
#[tokio::test]
async fn it_works(ctx: &mut TestSetup) {
  let message_db = MessageDb::new(&ctx.client);

  for _ in 0..500 {
    let stream_id = Uuid::new_v4();
    publish_test_message(&message_db, &stream_id).await;
    publish_test_message(&message_db, &stream_id).await;
    publish_test_message(&message_db, &stream_id).await;
  }

  let messages = message_db
    .reader
    .get_category_messages(&"testReader", None, None, None, None, None, None)
    .await
    .unwrap();

  assert_eq!(messages.len(), 100);
}
