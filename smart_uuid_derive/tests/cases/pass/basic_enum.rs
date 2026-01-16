//! Basic enum test - standard usage with 2-3 variants

use smart_uuid::{TypedUuid, UuidType, UserFriendlyUuid};

#[derive(Debug, Clone, Copy, PartialEq, Eq, UuidType)]
enum UserType {
    Retail,
    Business,
    Enterprise,
}

fn main() {
    // Test discriminant values
    assert_eq!(UserType::Retail.discriminant(), 0);
    assert_eq!(UserType::Business.discriminant(), 1);
    assert_eq!(UserType::Enterprise.discriminant(), 2);

    // Test from_discriminant round-trip
    assert_eq!(UserType::from_discriminant(0), Some(UserType::Retail));
    assert_eq!(UserType::from_discriminant(1), Some(UserType::Business));
    assert_eq!(UserType::from_discriminant(2), Some(UserType::Enterprise));
    assert_eq!(UserType::from_discriminant(3), None);
    assert_eq!(UserType::from_discriminant(255), None);

    // Test auto-generated prefixes
    assert_eq!(UserType::Retail.prefix(), "retail");
    assert_eq!(UserType::Business.prefix(), "business");
    assert_eq!(UserType::Enterprise.prefix(), "enterprise");

    // Test TypedUuid creation and round-trip
    let typed = TypedUuid::new(UserType::Retail);
    assert_eq!(typed.variant_type(), UserType::Retail);

    // Test UserFriendlyUuid conversion
    let friendly: UserFriendlyUuid<UserType> = typed.into();
    let friendly_str = friendly.to_string();
    assert!(friendly_str.starts_with("retail_"));

    // Test parsing back
    let parsed: UserFriendlyUuid<UserType> = friendly_str.parse().unwrap();
    let back: TypedUuid<UserType> = parsed.into();
    assert_eq!(back.variant_type(), UserType::Retail);

    println!("All basic_enum tests passed!");
}
