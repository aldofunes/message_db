use message_db::MessageDb;
use serde_json::json;
use uuid::Uuid;

pub async fn publish_test_message(message_db: &MessageDb<'_>, stream_name: &str) {
  let id = Uuid::new_v4();
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
    Ok(_) => log::info!("published message"),
    Err(_) => log::error!("failed to publish message"),
  };
}
