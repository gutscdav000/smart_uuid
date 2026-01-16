//! Fail case: Enum with tuple variant (not unit variant)

use smart_uuid::UuidType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, UuidType)]
enum HasTupleVariant {
    Unit,
    Tuple(i32),
    AlsoUnit,
}

fn main() {}
