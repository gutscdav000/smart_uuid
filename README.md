# smart_uuid

Type-safe UUIDs with embedded type information using UUID v8.

## Overview

This crate provides UUIDs that encode an enum variant directly in the UUID bytes, allowing you to:
- Distinguish different entity types by their UUID alone
- Display human-readable prefixed formats (e.g., `retail_550e8400-e29b-...`)
- Parse and validate UUIDs with type safety

## Usage

### Using the Derive Macro (Recommended)

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

### Manual Implementation

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

## The UuidType Derive Macro

### What Kind of Macro Is This?

This is a **derive procedural macro** - one of three types of Rust macros:

| Type | Syntax | Use Case |
|------|--------|----------|
| Declarative | `macro_rules!` | Pattern-based text substitution |
| **Derive Procedural** | `#[derive(Foo)]` | Auto-implement traits (what we use) |
| Attribute Procedural | `#[foo]` | Transform entire items |
| Function-like Procedural | `foo!(...)` | Custom syntax |

Procedural macros are compiled Rust code that runs at compile time, manipulating the Abstract Syntax Tree (AST). They must live in a separate crate (hence `smart_uuid_derive`).

### Supported Features

| Feature | Example | Notes |
|---------|---------|-------|
| Unit variant enums | `enum Foo { A, B }` | Required |
| Custom prefixes | `#[uuid_type(prefix = "x")]` | Optional |
| Up to 256 variants | `enum Big { V0, V1, ... V255 }` | Discriminant is stored in 1 byte |
| Acronym handling | `HTTPServer` -> `http_server` | Automatic |

### Not Supported

| Feature | Error Message |
|---------|--------------|
| Structs | "UuidType can only be derived for enums" |
| Tuple variants | "UuidType can only be derived for enums with unit variants" |
| Struct variants | "UuidType can only be derived for enums with unit variants" |
| Empty enums | "UuidType cannot be derived for empty enums" |
| >256 variants | "UuidType can only be derived for enums with at most 256 variants" |
| Invalid attributes | "unknown uuid_type attribute `foo`. Expected `prefix = \"...\"`" |

### What Can Go Wrong With Macros

Procedural macros have several failure modes that must be tested:

#### 1. Input Validation
The macro must reject invalid inputs with clear error messages:
- Applied to wrong type (struct instead of enum)
- Enum with non-unit variants
- Too many variants (>256)
- Empty enums

#### 2. Code Generation Bugs
The generated code could be subtly wrong:
- **Discriminant mismatch**: `discriminant()` and `from_discriminant()` must be inverses
- **Prefix bugs**: Snake_case conversion must handle edge cases like acronyms
- **Path resolution**: Must use `smart_uuid::UuidType`, not just `UuidType`

#### 3. Attribute Parsing
Custom attributes can fail in confusing ways:
- Typos in attribute keys (e.g., `prfx` instead of `prefix`)
- Wrong value types
- Unknown keys

### How We Test the Macro

We use [`trybuild`](https://crates.io/crates/trybuild) - the standard crate for testing proc macros:

```
smart_uuid_derive/tests/
├── integration.rs          # Test runner
└── cases/
    ├── pass/               # Should compile and run
    │   ├── basic_enum.rs
    │   ├── custom_prefix.rs
    │   ├── many_variants.rs
    │   ├── single_variant.rs
    │   └── snake_case_acronyms.rs
    └── fail/               # Should fail with expected errors
        ├── empty_enum.rs + .stderr
        ├── invalid_attribute.rs + .stderr
        ├── struct_not_enum.rs + .stderr
        ├── struct_variant.rs + .stderr
        ├── too_many_variants.rs + .stderr
        └── tuple_variant.rs + .stderr
```

Run macro tests:
```bash
cargo test -p smart_uuid_derive
```

## Building, Testing, and Running

### Build

```bash
# Build everything
cargo build --workspace

# Build only the main library
cargo build -p smart_uuid

# Build only the derive macro
cargo build -p smart_uuid_derive
```

### Test

```bash
# Run all tests (library + macro)
cargo test --workspace

# Run only library tests (21 tests)
cargo test -p smart_uuid

# Run only macro tests (11 trybuild cases)
cargo test -p smart_uuid_derive
```

### Run

```bash
# Run the demo example
cargo run -p smart_uuid --example demo
```

## Project Structure

```
smart_uuid/
├── Cargo.toml              # Workspace manifest (defines members)
├── smart_uuid/             # Main library
│   ├── Cargo.toml          # Library package manifest
│   ├── src/
│   │   ├── lib.rs
│   │   ├── traits.rs       # UuidType trait
│   │   ├── typed_uuid.rs
│   │   ├── user_friendly_uuid.rs
│   │   └── error.rs
│   └── examples/
│       └── demo.rs
└── smart_uuid_derive/      # Procedural macro crate
    ├── Cargo.toml          # Macro package manifest
    ├── src/
    │   └── lib.rs          # Macro implementation
    └── tests/
        └── cases/          # trybuild test cases
```

### Why Multiple Cargo.toml Files?

This project uses a **Cargo workspace** - a common Rust pattern for multi-crate projects.

| File | Purpose |
|------|---------|
| `/Cargo.toml` | **Workspace manifest** - declares member crates, enables shared `Cargo.lock` and `target/` directory |
| `/smart_uuid/Cargo.toml` | **Library manifest** - defines the main library's dependencies and metadata |
| `/smart_uuid_derive/Cargo.toml` | **Macro manifest** - defines the proc-macro crate with `proc-macro = true` |

**Why is the macro in a separate crate?** Rust requires procedural macros to be compiled before the code that uses them. A proc-macro crate can only export procedural macros - it cannot contain regular library code. This is a language-level requirement, not a stylistic choice.

This pattern is used by many popular crates:
- `serde` + `serde_derive`
- `thiserror` + `thiserror-impl`
- `tokio` + `tokio-macros`
