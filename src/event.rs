use crate::{entity::Entity, message::Message};

pub trait Event<E>:
  Clone + PartialEq + std::fmt::Debug + Sync + Send + Unpin + std::convert::TryFrom<Message>
where
  E: Entity,
{
  fn message_type(&self) -> &'static str;
  fn apply(self, aggregate: &mut E);
}
