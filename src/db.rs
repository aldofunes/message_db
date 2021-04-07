use crate::config::Config;
use refinery::embed_migrations;
use tokio_postgres::{connect, Client, Error, NoTls};

pub async fn run_migrations(client: &mut Client) {
  embed_migrations!("./migrations");
  // run migrations
  match migrations::runner().run_async(client).await {
    Ok(_) => log::info!("database migrations ran successfully"),
    Err(_) => log::error!("database migrations failed"),
  };
}

pub async fn db_client() -> Result<Client, Error> {
  let config = Config::new();

  // Connect to the database.
  let (client, connection) = connect(&config.connection_uri, NoTls).await?;

  // The connection object performs the actual communication with the database,
  // so spawn it off to run on its own.
  tokio::spawn(async move {
    if let Err(e) = connection.await {
      eprintln!("connection error: {}", e);
    }
  });

  Ok(client)
}
