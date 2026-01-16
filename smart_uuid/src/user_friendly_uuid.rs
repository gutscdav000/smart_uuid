use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;
use serde::{Serialize, Deserialize, Serializer, Deserializer};

use crate::error::TypedUuidError;
use crate::traits::UuidType;
use crate::typed_uuid::TypedUuid;

/// A user-friendly representation of a TypedUuid with a human-readable prefix.
///
/// Format: `{prefix}_{uuid}` where:
/// - `prefix` is derived from `UuidType::prefix()` (e.g., "retail", "business")
/// - `uuid` is the hyphenated UUID string
///
/// # Example
/// ```text
/// retail_550e8400-e29b-41d4-a716-446655440000
/// business_6ba7b810-9dad-11d1-80b4-00c04fd430c8
/// org_f47ac10b-58cc-4372-a567-0e02b2c3d479
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserFriendlyUuid<T: UuidType> {
    typed_uuid: TypedUuid<T>,
    _marker: PhantomData<T>,
}

impl<T: UuidType> UserFriendlyUuid<T> {
    /// Creates a new UserFriendlyUuid with a random UUID and the given type variant.
    pub fn new(variant: T) -> Self {
        Self {
            typed_uuid: TypedUuid::new(variant),
            _marker: PhantomData,
        }
    }

    /// Creates a UserFriendlyUuid from an existing TypedUuid.
    pub fn from_typed_uuid(typed: TypedUuid<T>) -> Self {
        Self {
            typed_uuid: typed,
            _marker: PhantomData,
        }
    }

    /// Parses a user-friendly string like "retail_550e8400-e29b-41d4-a716-446655440000".
    pub fn parse_str(s: &str) -> Result<Self, TypedUuidError> {
        // Find the last underscore to split prefix from UUID.
        // We use rfind because prefixes may contain underscores (e.g., "http_server"),
        // but UUIDs never contain underscores (only hyphens).
        let underscore_pos = s.rfind('_').ok_or_else(|| {
            TypedUuidError::InvalidFormat(
                "expected format 'prefix_uuid', no underscore found".to_string(),
            )
        })?;

        let prefix = &s[..underscore_pos];
        let uuid_str = &s[underscore_pos + 1..];

        // Parse the UUID
        let uuid = uuid::Uuid::parse_str(uuid_str)
            .map_err(|e| TypedUuidError::ParseError(e.to_string()))?;

        // Create TypedUuid (this validates the discriminant)
        let typed_uuid: TypedUuid<T> = TypedUuid::from_uuid(uuid)?;

        // Verify the prefix matches the variant encoded in the UUID
        let expected_prefix = typed_uuid.variant_type().prefix();
        if prefix != expected_prefix {
            return Err(TypedUuidError::UnknownPrefix {
                prefix: prefix.to_string(),
                type_name: std::any::type_name::<T>(),
            });
        }

        Ok(Self {
            typed_uuid,
            _marker: PhantomData,
        })
    }

    /// Returns the enum variant encoded in this UUID.
    pub fn variant_type(&self) -> T {
        self.typed_uuid.variant_type()
    }

    /// Returns the prefix string for this UUID's variant.
    pub fn prefix(&self) -> &'static str {
        self.typed_uuid.variant_type().prefix()
    }

    /// Returns a reference to the underlying TypedUuid.
    pub fn as_typed_uuid(&self) -> &TypedUuid<T> {
        &self.typed_uuid
    }

    /// Consumes self and returns the underlying TypedUuid.
    pub fn into_typed_uuid(self) -> TypedUuid<T> {
        self.typed_uuid
    }
}

impl<T: UuidType> fmt::Debug for UserFriendlyUuid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserFriendlyUuid")
            .field("typed_uuid", &self.typed_uuid)
            .finish()
    }
}

impl<T: UuidType> fmt::Display for UserFriendlyUuid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}_{}", self.prefix(), self.typed_uuid.as_uuid())
    }
}

impl<T: UuidType> FromStr for UserFriendlyUuid<T> {
    type Err = TypedUuidError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_str(s)
    }
}

impl<T: UuidType> From<TypedUuid<T>> for UserFriendlyUuid<T> {
    fn from(typed: TypedUuid<T>) -> Self {
        Self::from_typed_uuid(typed)
    }
}

impl<T: UuidType> From<UserFriendlyUuid<T>> for TypedUuid<T> {
    fn from(friendly: UserFriendlyUuid<T>) -> Self {
        friendly.typed_uuid
    }
}

impl<T: UuidType> Serialize for UserFriendlyUuid<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, T: UuidType> Deserialize<'de> for UserFriendlyUuid<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::parse_str(&s).map_err(serde::de::Error::custom)
    }
}
