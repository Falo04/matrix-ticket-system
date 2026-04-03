use uuid::Uuid;

/// A trait for types that wrap a `Uuid` to uniquely identify a specific business model.
///
/// This provides type safety by ensuring you don't accidentally pass an `AccountUuid`
/// where a `UserUuid` is expected.
pub trait BusinessModelUuid<T>
where
    T: galvyn::rorm::Model,
{
    /// Creates an instance of the identifier from a database foreign model wrapper.
    fn new_from_model(value: galvyn::rorm::prelude::ForeignModel<T>) -> Self;

    /// Returns the underlying raw `Uuid`.
    fn get_inner(&self) -> Uuid;
}
