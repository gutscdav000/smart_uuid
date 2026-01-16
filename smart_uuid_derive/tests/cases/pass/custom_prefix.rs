//! Test custom prefix attribute

use smart_uuid::{TypedUuid, UuidType, UserFriendlyUuid};

#[derive(Debug, Clone, Copy, PartialEq, Eq, UuidType)]
enum DocumentType {
    // Auto-generated prefix
    Invoice,

    // Custom prefix
    #[uuid_type(prefix = "rcpt")]
    Receipt,

    // Custom prefix with underscore
    #[uuid_type(prefix = "purchase_order")]
    PurchaseOrder,

    // Short custom prefix
    #[uuid_type(prefix = "q")]
    Quote,
}

fn main() {
    // Test auto-generated prefix
    assert_eq!(DocumentType::Invoice.prefix(), "invoice");

    // Test custom prefixes
    assert_eq!(DocumentType::Receipt.prefix(), "rcpt");
    assert_eq!(DocumentType::PurchaseOrder.prefix(), "purchase_order");
    assert_eq!(DocumentType::Quote.prefix(), "q");

    // Verify in UserFriendlyUuid output
    let typed = TypedUuid::new(DocumentType::Receipt);
    let friendly: UserFriendlyUuid<DocumentType> = typed.into();
    assert!(friendly.to_string().starts_with("rcpt_"));

    let typed2 = TypedUuid::new(DocumentType::Quote);
    let friendly2: UserFriendlyUuid<DocumentType> = typed2.into();
    assert!(friendly2.to_string().starts_with("q_"));

    println!("Custom prefix tests passed!");
}
