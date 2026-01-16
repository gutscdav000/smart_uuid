//! Test with many variants (near the 256 limit)

use smart_uuid::UuidType;

// Generate an enum with 100 variants to test scalability
// (Not 256 to keep compile times reasonable)
#[derive(Debug, Clone, Copy, PartialEq, Eq, UuidType)]
enum ManyVariants {
    V000, V001, V002, V003, V004, V005, V006, V007, V008, V009,
    V010, V011, V012, V013, V014, V015, V016, V017, V018, V019,
    V020, V021, V022, V023, V024, V025, V026, V027, V028, V029,
    V030, V031, V032, V033, V034, V035, V036, V037, V038, V039,
    V040, V041, V042, V043, V044, V045, V046, V047, V048, V049,
    V050, V051, V052, V053, V054, V055, V056, V057, V058, V059,
    V060, V061, V062, V063, V064, V065, V066, V067, V068, V069,
    V070, V071, V072, V073, V074, V075, V076, V077, V078, V079,
    V080, V081, V082, V083, V084, V085, V086, V087, V088, V089,
    V090, V091, V092, V093, V094, V095, V096, V097, V098, V099,
}

fn main() {
    // Test first variant
    assert_eq!(ManyVariants::V000.discriminant(), 0);
    assert_eq!(ManyVariants::from_discriminant(0), Some(ManyVariants::V000));

    // Test last variant
    assert_eq!(ManyVariants::V099.discriminant(), 99);
    assert_eq!(ManyVariants::from_discriminant(99), Some(ManyVariants::V099));

    // Test out of bounds
    assert_eq!(ManyVariants::from_discriminant(100), None);
    assert_eq!(ManyVariants::from_discriminant(255), None);

    // Test middle variant
    assert_eq!(ManyVariants::V050.discriminant(), 50);
    assert_eq!(ManyVariants::from_discriminant(50), Some(ManyVariants::V050));

    // Test prefix generation for numbered variants
    assert_eq!(ManyVariants::V000.prefix(), "v000");
    assert_eq!(ManyVariants::V099.prefix(), "v099");

    println!("Many variants test passed!");
}
