//! Fail case: Applying UuidType to a struct instead of enum

use smart_uuid::UuidType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, UuidType)]
struct NotAnEnum {
    id: u32,
}

fn main() {}
