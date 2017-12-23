extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate ordslice;

extern crate fall_tree;
extern crate fall_parse;
extern crate fall_editor;
extern crate indxr;

mod rust;

pub use self::rust::*;
pub use self::rust::language as lang_rust;
pub mod editor;

