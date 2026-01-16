//! Fail case: Enum with struct variant (not unit variant)

use smart_uuid::UuidType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, UuidType)]
enum HasStructVariant {
    Unit,
    Struct { value: i32 },
}

fn main() {}
