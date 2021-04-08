use async_trait::async_trait;
use message_db::{db_client, run_migrations};
use test_context::AsyncTestContext;
use tokio_postgres::Client;

pub struct TestSetup {
  pub client: Client,
}

#[async_trait]
impl AsyncTestContext for TestSetup {
  async fn setup() -> Self {
    match pretty_env_logger::try_init() {
      Ok(_) => log::info!("logger initialized"),
      Err(error) => log::error!("error: {}", error),
    };
    let mut client = db_client().await.unwrap();

    match run_migrations(&mut client).await {
      Ok(_) => log::info!("database migrations were successful"),
      Err(error) => log::error!("database migrations failed: {}", error),
    };
    Self { client }
  }

  async fn teardown(self) {
    // Perform any teardown you wish.
  }
}
