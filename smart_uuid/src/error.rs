use thiserror::Error;

/// Errors that can occur when working with TypedUuid and UserFriendlyUuid.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TypedUuidError {
    /// The UUID does not contain a valid type discriminant.
    #[error("invalid discriminant {found} for type {type_name}")]
    InvalidDiscriminant {
        found: u8,
        type_name: &'static str,
    },

    /// Failed to parse a UUID string.
    #[error("failed to parse UUID: {0}")]
    ParseError(String),

    /// The prefix in a UserFriendlyUuid string is not recognized.
    #[error("unknown prefix '{prefix}' for type {type_name}")]
    UnknownPrefix {
        prefix: String,
        type_name: &'static str,
    },

    /// Invalid format for UserFriendlyUuid string.
    #[error("invalid format: {0}")]
    InvalidFormat(String),
}
