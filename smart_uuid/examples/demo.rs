//! Demo showing both ways to implement UuidType:
//! 1. Using the derive macro (recommended)
//! 2. Manual implementation
//!
//! Run with: cargo run -p smart_uuid --example demo

use smart_uuid::{TypedUuid, UuidType, UserFriendlyUuid};

// =============================================================================
// APPROACH 1: Using the derive macro (recommended)
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, UuidType)]
enum UserType {
    Retail,
    Business,
    #[uuid_type(prefix = "org")]
    Organization,
}

// =============================================================================
// APPROACH 2: Manual implementation
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DocumentType {
    Invoice,
    Receipt,
    Contract,
}

impl UuidType for DocumentType {
    fn discriminant(&self) -> u8 {
        match self {
            Self::Invoice => 0,
            Self::Receipt => 1,
            Self::Contract => 2,
        }
    }

    fn from_discriminant(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Invoice),
            1 => Some(Self::Receipt),
            2 => Some(Self::Contract),
            _ => None,
        }
    }

    fn prefix(&self) -> &'static str {
        match self {
            Self::Invoice => "inv",
            Self::Receipt => "rcpt",
            Self::Contract => "contract",
        }
    }
}

// =============================================================================
// Demo
// =============================================================================

fn main() {
    println!("=== smart_uuid Demo ===\n");

    // --- Macro-derived type ---
    println!("1. Using derive macro (UserType):");
    println!("   ---------------------------------");

    let user_id = TypedUuid::new(UserType::Retail);
    println!("   Created TypedUuid: {}", user_id.as_uuid());
    println!("   Variant type: {:?}", user_id.variant_type());
    println!("   Discriminant: {}", user_id.variant_type().discriminant());

    let friendly: UserFriendlyUuid<UserType> = user_id.into();
    println!("   User-friendly format: {}", friendly);

    // Round-trip
    let parsed: UserFriendlyUuid<UserType> = friendly.to_string().parse().unwrap();
    let back_to_typed: TypedUuid<UserType> = parsed.into();
    println!("   Round-trip successful: {}", back_to_typed.as_uuid() == user_id.as_uuid());

    println!();

    // --- Manually implemented type ---
    println!("2. Using manual implementation (DocumentType):");
    println!("   ---------------------------------------------");

    let doc_id = TypedUuid::new(DocumentType::Invoice);
    println!("   Created TypedUuid: {}", doc_id.as_uuid());
    println!("   Variant type: {:?}", doc_id.variant_type());
    println!("   Discriminant: {}", doc_id.variant_type().discriminant());

    let friendly_doc: UserFriendlyUuid<DocumentType> = doc_id.into();
    println!("   User-friendly format: {}", friendly_doc);

    println!();

    // --- Show all variants ---
    println!("3. All UserType variants (macro-derived):");
    for variant in [UserType::Retail, UserType::Business, UserType::Organization] {
        println!("   {:?} -> discriminant={}, prefix=\"{}\"",
            variant, variant.discriminant(), variant.prefix());
    }

    println!();

    println!("4. All DocumentType variants (manual impl):");
    for variant in [DocumentType::Invoice, DocumentType::Receipt, DocumentType::Contract] {
        println!("   {:?} -> discriminant={}, prefix=\"{}\"",
            variant, variant.discriminant(), variant.prefix());
    }
}
