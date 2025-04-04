#[test]
fn run_passing_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/it/enums.rs");
    t.pass("tests/it/structs.rs");
}

#[test]
fn struct_compile_failures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/it/struct_incorrect_fields.rs");
}

#[test]
fn enum_compile_failures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/it/tuple_enum_incorrect_fields.rs");
    t.compile_fail("tests/it/struct_variant_enum_incorrect_fields.rs");
}

#[test]
fn thiserror_integration() {
    let t = trybuild::TestCases::new();
    t.pass("tests/it/thiserror.rs");
}
