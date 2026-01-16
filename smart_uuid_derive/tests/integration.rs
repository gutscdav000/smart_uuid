//! Integration tests for the UuidType derive macro.
//!
//! Uses `trybuild` to test:
//! - Compile-pass cases: valid usage that should compile
//! - Compile-fail cases: invalid usage with expected error messages

#[test]
fn compile_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/cases/pass/*.rs");
    t.compile_fail("tests/cases/fail/*.rs");
}
