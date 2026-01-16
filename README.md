# smart_uuid

Type-safe UUIDs with embedded type information using UUID v8.

## Overview

This crate provides UUIDs that encode an enum variant directly in the UUID bytes, allowing you to:
- Distinguish different entity types by their UUID alone
- Display human-readable prefixed formats (e.g., `retail_550e8400-e29b-...`)
- Parse and validate UUIDs with type safety

## Usage

### Implementing UuidType

There are two ways to implement the `UuidType` trait:

**1. Using the derive macro (recommended):**

```rust
use smart_uuid::{TypedUuid, UuidType, UserFriendlyUuid};

#[derive(Debug, Clone, Copy, PartialEq, Eq, UuidType)]
enum UserType {
    Retail,                      // discriminant=0, prefix="retail"
    Business,                    // discriminant=1, prefix="business"
    #[uuid_type(prefix = "org")] // custom prefix
    Organization,                // discriminant=2, prefix="org"
}

let user_id = TypedUuid::new(UserType::Retail);
let friendly: UserFriendlyUuid<UserType> = user_id.into();
println!("{}", friendly); // retail_550e8400-e29b-...
```

**2. Manual implementation:**

```rust
use smart_uuid::UuidType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DocumentType {
    Invoice,
    Receipt,
}

impl UuidType for DocumentType {
    fn discriminant(&self) -> u8 {
        match self {
            Self::Invoice => 0,
            Self::Receipt => 1,
        }
    }

    fn from_discriminant(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Invoice),
            1 => Some(Self::Receipt),
            _ => None,
        }
    }

    fn prefix(&self) -> &'static str {
        match self {
            Self::Invoice => "inv",
            Self::Receipt => "rcpt",
        }
    }
}
```

## Running the Demo

To see both approaches in action:

```bash
cargo run -p smart_uuid --example demo
```

## Running Tests

```bash
cargo test -p smart_uuid
```

## Project Structure

This is a Cargo workspace with two crates:

- `smart_uuid/` - The main library
- `smart_uuid_derive/` - Procedural macro for `#[derive(UuidType)]`

The derive macro lives in a separate crate because Rust requires procedural macros to be in their own crate.
