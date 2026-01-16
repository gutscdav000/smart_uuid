use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;
use uuid::Uuid;
use serde::{Serialize, Deserialize, Serializer, Deserializer};

use crate::error::TypedUuidError;
use crate::traits::UuidType;

/// A strongly-typed UUID that encodes an enum variant in its bytes.
///
/// Uses UUID v8 (custom) format, storing the type discriminant in byte 0.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypedUuid<T: UuidType> {
    inner: Uuid,
    _marker: PhantomData<T>,
}

impl<T: UuidType> TypedUuid<T> {
    /// Creates a new TypedUuid with a random UUID and the given type variant.
    pub fn new(variant: T) -> Self {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 16];

        // Fill with random bytes
        rng.fill(&mut bytes);

        // Set the discriminant in byte 0
        bytes[0] = variant.discriminant();

        // Create a v8 UUID (this will set version and variant bits)
        let uuid = Uuid::new_v8(bytes);

        Self {
            inner: uuid,
            _marker: PhantomData,
        }
    }

    /// Creates a TypedUuid from an existing UUID, validating the discriminant.
    pub fn from_uuid(uuid: Uuid) -> Result<Self, TypedUuidError> {
        let bytes = uuid.as_bytes();
        let discriminant = bytes[0];

        // Validate that the discriminant maps to a known variant
        T::from_discriminant(discriminant).ok_or(TypedUuidError::InvalidDiscriminant {
            found: discriminant,
            type_name: std::any::type_name::<T>(),
        })?;

        Ok(Self {
            inner: uuid,
            _marker: PhantomData,
        })
    }

    /// Returns the enum variant encoded in this UUID.
    pub fn variant_type(&self) -> T {
        let bytes = self.inner.as_bytes();
        let discriminant = bytes[0];

        // This should never fail if the TypedUuid was created correctly
        T::from_discriminant(discriminant)
            .expect("TypedUuid contains invalid discriminant - this is a bug")
    }

    /// Returns a reference to the underlying UUID.
    pub fn as_uuid(&self) -> &Uuid {
        &self.inner
    }

    /// Consumes self and returns the underlying UUID.
    pub fn into_uuid(self) -> Uuid {
        self.inner
    }

    /// Returns the raw bytes of the UUID.
    pub fn as_bytes(&self) -> &[u8; 16] {
        self.inner.as_bytes()
    }
}

impl<T: UuidType> fmt::Debug for TypedUuid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TypedUuid")
            .field("uuid", &self.inner)
            .field("variant", &self.variant_type())
            .finish()
    }
}

impl<T: UuidType> fmt::Display for TypedUuid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<T: UuidType> FromStr for TypedUuid<T> {
    type Err = TypedUuidError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::parse_str(s)
            .map_err(|e| TypedUuidError::ParseError(e.to_string()))?;
        Self::from_uuid(uuid)
    }
}

impl<T: UuidType> From<TypedUuid<T>> for Uuid {
    fn from(typed: TypedUuid<T>) -> Self {
        typed.inner
    }
}

impl<T: UuidType> Serialize for TypedUuid<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de, T: UuidType> Deserialize<'de> for TypedUuid<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let uuid = Uuid::deserialize(deserializer)?;
        Self::from_uuid(uuid).map_err(serde::de::Error::custom)
    }
}
