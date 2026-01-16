//! Demonstrates all features of the #[derive(UuidType)] macro
//!
//! Run with: cargo run -p smart_uuid --example macro_features

use smart_uuid::{TypedUuid, UuidType, UserFriendlyUuid};

// =============================================================================
// Feature 1: Basic derive with auto-generated prefixes
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, UuidType)]
enum BasicType {
    First,   // prefix = "first"
    Second,  // prefix = "second"
    Third,   // prefix = "third"
}

// =============================================================================
// Feature 2: Custom prefixes with #[uuid_type(prefix = "...")]
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, UuidType)]
enum ApiResource {
    #[uuid_type(prefix = "usr")]
    User,
    #[uuid_type(prefix = "org")]
    Organization,
    #[uuid_type(prefix = "proj")]
    Project,
    // Mix: this one uses auto-generated prefix
    Settings,  // prefix = "settings"
}

// =============================================================================
// Feature 3: Acronym handling (HTTPServer -> http_server, not h_t_t_p_server)
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, UuidType)]
enum ServiceType {
    HTTPServer,   // prefix = "http_server"
    HTTPSProxy,   // prefix = "https_proxy"
    TCPSocket,    // prefix = "tcp_socket"
    JSONParser,   // prefix = "json_parser"
    XMLEncoder,   // prefix = "xml_encoder"
}

// =============================================================================
// Feature 4: Single variant enum (edge case)
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, UuidType)]
enum Singleton {
    Instance,
}

// =============================================================================
// Demo
// =============================================================================

fn main() {
    println!("=== UuidType Derive Macro Features ===\n");

    // --- Feature 1: Basic derive ---
    println!("1. Basic derive (auto-generated prefixes):");
    println!("   ----------------------------------------");
    for variant in [BasicType::First, BasicType::Second, BasicType::Third] {
        let uuid = TypedUuid::new(variant);
        let friendly: UserFriendlyUuid<BasicType> = uuid.into();
        println!("   {:?} -> {}", variant, friendly);
    }
    println!();

    // --- Feature 2: Custom prefixes ---
    println!("2. Custom prefixes (#[uuid_type(prefix = \"...\")]):");
    println!("   -------------------------------------------------");
    for (variant, expected) in [
        (ApiResource::User, "usr"),
        (ApiResource::Organization, "org"),
        (ApiResource::Project, "proj"),
        (ApiResource::Settings, "settings"),
    ] {
        assert_eq!(variant.prefix(), expected);
        let uuid = TypedUuid::new(variant);
        let friendly: UserFriendlyUuid<ApiResource> = uuid.into();
        println!("   {:?} -> {} (prefix: \"{}\")", variant, friendly, expected);
    }
    println!();

    // --- Feature 3: Acronym handling ---
    println!("3. Acronym handling (smart snake_case conversion):");
    println!("   ------------------------------------------------");
    for (variant, expected) in [
        (ServiceType::HTTPServer, "http_server"),
        (ServiceType::HTTPSProxy, "https_proxy"),
        (ServiceType::TCPSocket, "tcp_socket"),
        (ServiceType::JSONParser, "json_parser"),
        (ServiceType::XMLEncoder, "xml_encoder"),
    ] {
        assert_eq!(variant.prefix(), expected);
        println!("   {:?} -> \"{}\"", variant, expected);
    }
    println!();

    // --- Feature 4: Discriminant round-trip ---
    println!("4. Discriminant round-trip verification:");
    println!("   --------------------------------------");
    for variant in [ApiResource::User, ApiResource::Organization, ApiResource::Project, ApiResource::Settings] {
        let disc = variant.discriminant();
        let recovered = ApiResource::from_discriminant(disc).unwrap();
        assert_eq!(variant, recovered);
        println!("   {:?} -> discriminant {} -> {:?} (round-trip OK)", variant, disc, recovered);
    }
    println!();

    // --- Feature 5: Full workflow ---
    println!("5. Full workflow (create -> stringify -> parse -> recover):");
    println!("   ---------------------------------------------------------");
    let original = TypedUuid::new(ServiceType::HTTPServer);
    println!("   Original UUID: {}", original.as_uuid());

    let friendly: UserFriendlyUuid<ServiceType> = original.into();
    let friendly_str = friendly.to_string();
    println!("   Friendly format: {}", friendly_str);

    let parsed: UserFriendlyUuid<ServiceType> = friendly_str.parse().unwrap();
    let recovered: TypedUuid<ServiceType> = parsed.into();
    println!("   Recovered UUID: {}", recovered.as_uuid());
    println!("   Match: {}", original.as_uuid() == recovered.as_uuid());

    println!("\n=== All macro features working correctly! ===");
}
