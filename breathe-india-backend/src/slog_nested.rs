#[derive(Clone, Serialize)]
pub struct WrapSerde<T>(pub T);

impl<T> slog::SerdeValue for WrapSerde<T>
where
    T: serde::Serialize + Clone + Send + 'static,
{
    fn as_serde(&self) -> &dyn erased_serde::Serialize {
        self
    }

    fn to_sendable(&self) -> Box<dyn slog::SerdeValue + Send + 'static> {
        Box::new(self.clone())
    }
}

impl<T> slog::Value for WrapSerde<T>
where
    T: serde::Serialize + Clone + Send + 'static,
{
    fn serialize(
        &self,
        _: &slog::Record<'_>,
        key: slog::Key,
        serializer: &mut dyn slog::Serializer,
    ) -> slog::Result {
        serializer.emit_serde(key, self)
    }
}
