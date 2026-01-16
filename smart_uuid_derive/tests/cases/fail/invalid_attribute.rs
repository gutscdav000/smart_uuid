//! Fail case: Invalid uuid_type attribute key

use smart_uuid::UuidType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, UuidType)]
enum EntityType {
    // Typo: "prfx" instead of "prefix"
    #[uuid_type(prfx = "usr")]
    User,

    Admin,
}

fn main() {}
