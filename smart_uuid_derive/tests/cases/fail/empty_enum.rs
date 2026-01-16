//! BUG TEST: Empty enum
//!
//! Current behavior: Fails with confusing "non-exhaustive patterns" error
//! Expected behavior: Should produce a clear macro error like:
//!   "UuidType cannot be derived for empty enums"
//!
//! After Phase D fixes, the .stderr should show our helpful error message.

use smart_uuid::UuidType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, UuidType)]
enum EmptyEnum {}

fn main() {}
