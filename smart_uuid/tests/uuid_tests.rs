//! Unit tests for smart_uuid
//!
//! These tests are written FIRST (TDD) before implementation.

use smart_uuid::{TypedUuid, UserFriendlyUuid, UuidType, TypedUuidError, Uuid};

// ============================================================================
// Test Enum - uses derive macro
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, UuidType)]
enum UserType {
    Retail,
    Business,
    #[uuid_type(prefix = "org")]
    Organization,
}

// ============================================================================
// Derive Macro Tests
// ============================================================================

#[test]
fn derive_macro_generates_correct_discriminants() {
    // Discriminants should be assigned in order: 0, 1, 2
    assert_eq!(UserType::Retail.discriminant(), 0);
    assert_eq!(UserType::Business.discriminant(), 1);
    assert_eq!(UserType::Organization.discriminant(), 2);
}

#[test]
fn derive_macro_from_discriminant_works() {
    assert_eq!(UserType::from_discriminant(0), Some(UserType::Retail));
    assert_eq!(UserType::from_discriminant(1), Some(UserType::Business));
    assert_eq!(UserType::from_discriminant(2), Some(UserType::Organization));
    assert_eq!(UserType::from_discriminant(255), None);
}

#[test]
fn derive_macro_generates_correct_prefixes() {
    // Default snake_case for Retail and Business
    assert_eq!(UserType::Retail.prefix(), "retail");
    assert_eq!(UserType::Business.prefix(), "business");
    // Custom prefix for Organization
    assert_eq!(UserType::Organization.prefix(), "org");
}

// ============================================================================
// TypedUuid Tests
// ============================================================================

#[test]
fn typed_uuid_new_preserves_variant_type() {
    let typed = TypedUuid::new(UserType::Retail);
    assert_eq!(typed.variant_type(), UserType::Retail);

    let typed = TypedUuid::new(UserType::Business);
    assert_eq!(typed.variant_type(), UserType::Business);

    let typed = TypedUuid::new(UserType::Organization);
    assert_eq!(typed.variant_type(), UserType::Organization);
}

#[test]
fn typed_uuid_discriminant_stored_in_byte_0() {
    let typed = TypedUuid::new(UserType::Retail);
    assert_eq!(typed.as_bytes()[0], 0);

    let typed = TypedUuid::new(UserType::Business);
    assert_eq!(typed.as_bytes()[0], 1);

    let typed = TypedUuid::new(UserType::Organization);
    assert_eq!(typed.as_bytes()[0], 2);
}

#[test]
fn typed_uuid_is_valid_v8_uuid() {
    let typed = TypedUuid::new(UserType::Retail);
    let uuid = typed.as_uuid();

    // UUID v8 should have version 8
    assert_eq!(uuid.get_version_num(), 8);
}

#[test]
fn typed_uuid_from_uuid_validates_discriminant() {
    // Create a valid TypedUuid
    let original = TypedUuid::new(UserType::Retail);
    let uuid = original.into_uuid();

    // Should be able to recreate from the UUID
    let recreated = TypedUuid::<UserType>::from_uuid(uuid).unwrap();
    assert_eq!(recreated.variant_type(), UserType::Retail);
}

#[test]
fn typed_uuid_from_uuid_rejects_invalid_discriminant() {
    // Create a v8 UUID with an invalid discriminant (255)
    let mut bytes = [0u8; 16];
    bytes[0] = 255; // Invalid discriminant
    let uuid = Uuid::new_v8(bytes);

    let result = TypedUuid::<UserType>::from_uuid(uuid);
    assert!(matches!(result, Err(TypedUuidError::InvalidDiscriminant { found: 255, .. })));
}

#[test]
fn typed_uuid_parse_str_works() {
    let original = TypedUuid::new(UserType::Business);
    let uuid_str = original.to_string();

    let parsed: TypedUuid<UserType> = uuid_str.parse().unwrap();
    assert_eq!(parsed.variant_type(), UserType::Business);
}

#[test]
fn typed_uuid_display_is_standard_uuid_format() {
    let typed = TypedUuid::new(UserType::Retail);
    let display = typed.to_string();

    // Should be standard UUID format (36 chars with hyphens)
    assert_eq!(display.len(), 36);
    assert!(display.chars().filter(|c| *c == '-').count() == 4);
}

// ============================================================================
// UserFriendlyUuid Tests
// ============================================================================

#[test]
fn user_friendly_uuid_formats_with_prefix() {
    let friendly = UserFriendlyUuid::new(UserType::Retail);
    let display = friendly.to_string();

    assert!(display.starts_with("retail_"));
    // After prefix, should have standard UUID
    let uuid_part = &display[7..]; // "retail_" is 7 chars
    assert_eq!(uuid_part.len(), 36);
}

#[test]
fn user_friendly_uuid_uses_custom_prefix() {
    let friendly = UserFriendlyUuid::new(UserType::Organization);
    let display = friendly.to_string();

    assert!(display.starts_with("org_"));
}

#[test]
fn user_friendly_uuid_parse_str_works() {
    let original = UserFriendlyUuid::new(UserType::Business);
    let friendly_str = original.to_string();

    let parsed = UserFriendlyUuid::<UserType>::parse_str(&friendly_str).unwrap();
    assert_eq!(parsed.variant_type(), UserType::Business);
}

#[test]
fn user_friendly_uuid_parse_rejects_unknown_prefix() {
    // Create a valid TypedUuid for Retail
    let typed = TypedUuid::new(UserType::Retail);
    let uuid_str = typed.to_string();

    // Try to parse with a wrong prefix - should fail with UnknownPrefix
    let wrong_prefix_str = format!("wrong_{}", uuid_str);
    let result = UserFriendlyUuid::<UserType>::parse_str(&wrong_prefix_str);
    assert!(matches!(result, Err(TypedUuidError::UnknownPrefix { .. })));
}

#[test]
fn user_friendly_uuid_parse_rejects_invalid_format() {
    // Missing underscore separator
    let result = UserFriendlyUuid::<UserType>::parse_str("retail550e8400-e29b-41d4-a716-446655440000");
    assert!(matches!(result, Err(TypedUuidError::InvalidFormat(_))));
}

// ============================================================================
// Conversion Tests
// ============================================================================

#[test]
fn typed_uuid_converts_to_user_friendly() {
    let typed = TypedUuid::new(UserType::Retail);
    let friendly: UserFriendlyUuid<UserType> = typed.into();

    assert_eq!(friendly.variant_type(), UserType::Retail);
    assert!(friendly.to_string().starts_with("retail_"));
}

#[test]
fn user_friendly_converts_to_typed_uuid() {
    let friendly = UserFriendlyUuid::new(UserType::Business);
    let typed: TypedUuid<UserType> = friendly.into();

    assert_eq!(typed.variant_type(), UserType::Business);
}

#[test]
fn roundtrip_typed_to_friendly_and_back() {
    let original = TypedUuid::new(UserType::Organization);
    let original_uuid = *original.as_uuid();

    let friendly: UserFriendlyUuid<UserType> = original.into();
    let back: TypedUuid<UserType> = friendly.into();

    assert_eq!(*back.as_uuid(), original_uuid);
    assert_eq!(back.variant_type(), UserType::Organization);
}

// ============================================================================
// Serde Tests
// ============================================================================

#[test]
fn typed_uuid_serde_roundtrip() {
    let original = TypedUuid::new(UserType::Retail);

    let json = serde_json::to_string(&original).unwrap();
    let deserialized: TypedUuid<UserType> = serde_json::from_str(&json).unwrap();

    assert_eq!(*original.as_uuid(), *deserialized.as_uuid());
    assert_eq!(original.variant_type(), deserialized.variant_type());
}

#[test]
fn user_friendly_uuid_serde_roundtrip() {
    let original = UserFriendlyUuid::new(UserType::Business);

    let json = serde_json::to_string(&original).unwrap();
    let deserialized: UserFriendlyUuid<UserType> = serde_json::from_str(&json).unwrap();

    assert_eq!(original.variant_type(), deserialized.variant_type());
    assert_eq!(original.to_string(), deserialized.to_string());
}

#[test]
fn user_friendly_uuid_serializes_as_prefixed_string() {
    let friendly = UserFriendlyUuid::new(UserType::Organization);

    let json = serde_json::to_string(&friendly).unwrap();

    // Should serialize as a quoted string with prefix
    assert!(json.starts_with("\"org_"));
    assert!(json.ends_with("\""));
}
