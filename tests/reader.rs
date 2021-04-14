mod test_setup;
mod utils;

use message_db::{Reader, Writer};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use test_context::{futures, test_context};
use test_setup::TestSetup;
use utils::publish_test_message;
use uuid::Uuid;

#[test_context(TestSetup)]
#[tokio::test]
async fn it_works(ctx: &mut TestSetup) {
  let reader = Reader::new(&ctx.pool);
  let writer = Writer::new(&ctx.pool);

  let category: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(7)
    .map(char::from)
    .collect();

  for _ in 0..100 {
    let stream_id = Uuid::new_v4();
    let stream_name = format!("{}-{}", category, stream_id);

    publish_test_message(&writer, &stream_name).await;
    publish_test_message(&writer, &stream_name).await;
    publish_test_message(&writer, &stream_name).await;
  }

  let messages = reader
    .get_category_messages(&category, None, None, None, None, None, None)
    .await
    .unwrap();

  assert_eq!(messages.len(), 300);
}
