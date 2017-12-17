extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate fall_tree;

use std::sync::Arc;
use fall_tree::{File, dump_file, TextEdit};

pub mod hl;

use self::hl::Highlights;

#[derive(Clone)]
pub struct EditorFile {
    imp: Arc<EditorFileImpl>
}

impl EditorFile {
    pub fn new<F: EditorFileImpl>(imp: F) -> EditorFile {
        EditorFile { imp: Arc::new(imp) }
    }

    pub fn edit(&self, edit: &TextEdit) -> EditorFile {
        EditorFile { imp: self.imp.edit(edit).into() }
    }

    pub fn file(&self) -> &File {
        self.imp.file()
    }

    pub fn syntax_tree(&self) -> String {
        self.imp.syntax_tree()
    }

    pub fn highlight(&self) -> Highlights {
        self.imp.highlight()
    }

    pub fn metrics(&self) -> String {
        self.file().metrics().to_string()
    }
}

pub trait EditorFileImpl: Sync + 'static {
    fn file(&self) -> &File;
    fn edit(&self, edit: &TextEdit) -> Box<EditorFileImpl>;
    fn syntax_tree(&self) -> String;
    fn highlight(&self) -> Highlights;
}

#[derive(Clone, Copy)]
pub struct EditorSupport {
    pub extension: &'static str,
    pub parse: fn(text: &str) -> EditorFile,
}

impl EditorSupport {
    pub fn parse(&self, text: &str) -> EditorFile {
        (self.parse)(text)
    }
}

pub fn gen_syntax_tree(file: &File) -> String {
    dump_file(&file)
}
