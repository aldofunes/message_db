mod test_setup;
mod utils;

use message_db::{Reader, Subscription, Writer};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde_json::json;
use test_context::{futures, test_context};
use test_setup::TestSetup;
use utils::publish_test_message;
use uuid::Uuid;

#[test_context(TestSetup)]
#[tokio::test]
async fn it_should_publish_and_read_a_message(ctx: &mut TestSetup) {
  let reader = Reader::new(&ctx.pool);
  let writer = Writer::new(&ctx.pool);

  let id = Uuid::new_v4();
  let stream_name = format!("test-{}", Uuid::new_v4());
  let message_type = "TestEvent";
  let data = json!({ "foo": "bar" });
  let metadata = json!({ "baz": "qux" });

  match writer
    .write_message(id, &stream_name, &message_type, data, Some(metadata), None)
    .await
  {
    Ok(_) => log::info!("message written"),
    Err(e) => log::error!("message failed to be written {}", e),
  };

  let message = reader
    .get_last_stream_message(&stream_name)
    .await
    .expect("failed to get last stream message")
    .unwrap();

  assert_eq!(message.id, id);
}

#[test_context(TestSetup)]
#[tokio::test]
async fn it_should_subscribe(ctx: &mut TestSetup) {
  log::info!("building MessageDb");
  let writer = Writer::new(&ctx.pool);

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
    publish_test_message(&writer, &stream_name).await;
  }

  log::info!("building subscription");
  let mut subscription = Subscription::new(
    &ctx.pool,
    category,
    String::from("test_subscriber"),
    Some(1000),
    None,
    None,
    None,
  );

  let mut messages_processed: i64 = 0;

  subscription.load_position().await;
  for message in subscription.poll(Some(1000)).await.unwrap() {
    let position = message.global_position;
    println!("{:?}", message.id);
    messages_processed += 1;
    subscription.update_read_position(position).await;
  }
  subscription.write_position(24).await;

  assert_eq!(messages_processed, 100);
}
