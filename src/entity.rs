pub trait Entity: Default + Send + Sync + Unpin {
  /// entity_type is a unique identifier for this entity
  fn stream_category() -> &'static str;
}
