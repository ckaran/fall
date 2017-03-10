extern crate fall;

pub fn foo() {}

pub mod sexp;
pub mod weird;
pub mod rust;

pub fn match_ast(actual: &str, expected: &str) {
    let actual = actual.trim();
    let expected = expected.trim();
    if actual != expected {
        panic!("Actual:\n{}\nExpected:\n{}\n", actual, expected)
    }
}