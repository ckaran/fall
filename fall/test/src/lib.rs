extern crate fall_parse;
extern crate fall_tree;

pub mod arith;
pub mod sexp;
pub mod weird;

pub fn match_ast(actual: &str, expected: &str) {
    let actual = actual.trim();
    let expected = expected.trim();
    if actual != expected {
        panic!("Actual:\n{}\nExpected:\n{}\n", actual, expected)
    }
}
