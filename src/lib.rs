mod message;
mod reader;
mod subscriber;
mod writer;

pub use message::Message;
use reader::Reader;
use sqlx::PgPool;
use subscriber::Subscriber;
use writer::Writer;

pub struct MessageDb<'a> {
  pub reader: Reader<'a>,
  pub subscriber: Subscriber<'a>,
  pub writer: Writer<'a>,
}

impl<'a> MessageDb<'a> {
  pub fn new(pool: &'a PgPool) -> Self {
    Self {
      reader: Reader::new(&pool),
      writer: Writer::new(&pool),
      subscriber: Subscriber::new(&pool),
    }
  }
}
