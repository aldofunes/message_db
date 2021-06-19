use async_trait::async_trait;
use sqlx::PgPool;
use std::env;
use test_context::AsyncTestContext;

pub struct TestSetup {
  pub pool: PgPool,
}

#[async_trait]
impl AsyncTestContext for TestSetup {
  async fn setup() -> Self {
    let default_connection = "postgresql://message_store:message_store@localhost:5432/message_store";
    let connection_uri =
      env::var("MESSAGE_DB_CONNECTION_URI").unwrap_or(String::from(default_connection));

    let pool = PgPool::connect(&connection_uri).await.unwrap();

    Self { pool }
  }

  async fn teardown(self) {
    // Perform any teardown you wish.
  }
}
