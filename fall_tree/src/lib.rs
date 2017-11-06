extern crate difference;
extern crate elapsed;
extern crate serde;
extern crate fall_text;
#[macro_use]
extern crate serde_derive;
extern crate file;

mod text_edit;

mod node_type;
mod node;
mod edit;

mod ast;
mod util;
mod lang;

pub mod visitor;
pub mod search;
pub mod test_util;

pub use fall_text::{Text, TextRange, TextUnit, tu};
pub use text_edit::TextEdit;
pub use node_type::{NodeType, NodeTypeInfo, ERROR};
pub use node::{File, Node, FileStats, INode};
pub use edit::FileEdit;
pub use lang::{Language, LanguageImpl};
pub use ast::{AstNode, AstChildren, AstClass, AstClassChildren};
pub use util::{dump_file, dump_file_ws, walk_tree};
