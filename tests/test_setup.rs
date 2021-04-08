use async_trait::async_trait;
use message_db::db_client;
use test_context::AsyncTestContext;
use tokio_postgres::Client;

pub struct TestSetup {
  pub client: Client,
}

#[async_trait]
impl AsyncTestContext for TestSetup {
  async fn setup() -> Self {
    let client = db_client().await.unwrap();

    Self { client }
  }

  async fn teardown(self) {
    // Perform any teardown you wish.
  }
}
