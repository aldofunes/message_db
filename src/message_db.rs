use crate::reader::Reader;
use crate::subscriber::Subscriber;
use crate::writer::Writer;
use tokio_postgres::Client;

pub struct MessageDb<'a> {
  pub reader: Reader<'a>,
  pub subscriber: Subscriber<'a>,
  pub writer: Writer<'a>,
}

impl<'a> MessageDb<'a> {
  pub fn new(client: &'a Client) -> Self {
    let reader = Reader::new(&client);
    let writer = Writer::new(&client);

    let subscriber = Subscriber::new(reader, writer);

    Self {
      reader: reader,
      writer: writer,
      subscriber: subscriber,
    }
  }
}