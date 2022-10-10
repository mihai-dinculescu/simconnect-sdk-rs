#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-parse.rs");
    t.compile_fail("tests/02-struct-attr-errors.rs");
    t.compile_fail("tests/03-field-attr-errors.rs");
    t.compile_fail("tests/04-invalid-values.rs");
}
