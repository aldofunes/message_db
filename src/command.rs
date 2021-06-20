use crate::message::Message;

pub trait Command:
  Clone + PartialEq + std::fmt::Debug + Sync + Send + Unpin + std::convert::TryFrom<Message>
{
  fn message_type(&self) -> &'static str;
}
