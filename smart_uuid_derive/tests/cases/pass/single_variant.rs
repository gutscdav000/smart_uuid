//! Edge case: Single variant enum

use smart_uuid::{TypedUuid, UuidType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, UuidType)]
enum SingletonType {
    TheOnlyOne,
}

fn main() {
    // Test discriminant
    assert_eq!(SingletonType::TheOnlyOne.discriminant(), 0);

    // Test from_discriminant
    assert_eq!(SingletonType::from_discriminant(0), Some(SingletonType::TheOnlyOne));
    assert_eq!(SingletonType::from_discriminant(1), None);

    // Test prefix
    assert_eq!(SingletonType::TheOnlyOne.prefix(), "the_only_one");

    // Test TypedUuid
    let typed = TypedUuid::new(SingletonType::TheOnlyOne);
    assert_eq!(typed.variant_type(), SingletonType::TheOnlyOne);

    println!("Single variant test passed!");
}
