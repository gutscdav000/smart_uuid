use std::fmt::Debug;

/// Trait that must be implemented by enum types used with TypedUuid and UserFriendlyUuid.
///
/// This trait is typically derived using `#[derive(UuidType)]` rather than implemented manually.
///
/// The trait provides:
/// - Byte discriminant encoding for UUID storage
/// - String prefix for human-readable formatting
pub trait UuidType: Copy + Clone + Eq + PartialEq + Debug + Sized {
    /// Returns the byte discriminant for this variant.
    /// Used internally to encode the type in UUID byte 0.
    fn discriminant(&self) -> u8;

    /// Reconstructs a variant from a byte discriminant.
    /// Returns `None` if the discriminant is not recognized.
    fn from_discriminant(value: u8) -> Option<Self>;

    /// Returns the prefix string used in UserFriendlyUuid formatting.
    fn prefix(&self) -> &'static str;
}
