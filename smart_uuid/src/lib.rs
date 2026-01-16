//! # smart_uuid
//!
//! Type-safe UUIDs with embedded type information using UUID v8.
//!
//! This crate provides two main types:
//! - [`TypedUuid<T>`]: A UUID that encodes an enum variant in its bytes
//! - [`UserFriendlyUuid<T>`]: A human-readable format with a prefix
//!
//! ## Example
//!
//! ```rust
//! use smart_uuid::{TypedUuid, UserFriendlyUuid, UuidType};
//!
//! #[derive(Debug, Clone, Copy, PartialEq, Eq, UuidType)]
//! enum UserType {
//!     Retail,
//!     Business,
//!     #[uuid_type(prefix = "org")]
//!     Organization,
//! }
//!
//! // Create a typed UUID
//! let typed = TypedUuid::new(UserType::Retail);
//! assert_eq!(typed.variant_type(), UserType::Retail);
//!
//! // Convert to user-friendly format
//! let friendly: UserFriendlyUuid<UserType> = typed.into();
//! // friendly.to_string() -> "retail_550e8400-e29b-..."
//! ```

mod error;
mod traits;
mod typed_uuid;
mod user_friendly_uuid;

pub use error::TypedUuidError;
pub use traits::UuidType;
pub use typed_uuid::TypedUuid;
pub use user_friendly_uuid::UserFriendlyUuid;

// Re-export the derive macro
pub use smart_uuid_derive::UuidType;

// Re-export uuid::Uuid for convenience
pub use uuid::Uuid;
