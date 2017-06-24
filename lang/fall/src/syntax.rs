use serde_json;
use fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl, FileStats, INode};
pub use fall_tree::{ERROR, WHITESPACE};

pub const EQ: NodeType = NodeType(100);
pub const PIPE: NodeType = NodeType(101);
pub const STAR: NodeType = NodeType(102);
pub const QUESTION: NodeType = NodeType(103);
pub const DOT: NodeType = NodeType(104);
pub const COMMA: NodeType = NodeType(105);
pub const HASH: NodeType = NodeType(106);
pub const LBRACE: NodeType = NodeType(107);
pub const RBRACE: NodeType = NodeType(108);
pub const LBRACK: NodeType = NodeType(109);
pub const RBRACK: NodeType = NodeType(110);
pub const LANGLE: NodeType = NodeType(111);
pub const RANGLE: NodeType = NodeType(112);
pub const LPAREN: NodeType = NodeType(113);
pub const RPAREN: NodeType = NodeType(114);
pub const KW_NODE: NodeType = NodeType(115);
pub const KW_CLASS: NodeType = NodeType(116);
pub const KW_TOKENIZER: NodeType = NodeType(117);
pub const KW_RULE: NodeType = NodeType(118);
pub const KW_VERBATIM: NodeType = NodeType(119);
pub const KW_AST: NodeType = NodeType(120);
pub const KW_PUB: NodeType = NodeType(121);
pub const KW_EXAMPLE: NodeType = NodeType(122);
pub const NUMBER: NodeType = NodeType(123);
pub const SIMPLE_STRING: NodeType = NodeType(124);
pub const HASH_STRING: NodeType = NodeType(125);
pub const IDENT: NodeType = NodeType(126);
pub const FALL_FILE: NodeType = NodeType(127);
pub const TOKENIZER_DEF: NodeType = NodeType(128);
pub const LEX_RULE: NodeType = NodeType(129);
pub const SYN_RULE: NodeType = NodeType(130);
pub const ATTRIBUTES: NodeType = NodeType(131);
pub const ATTRIBUTE: NodeType = NodeType(132);
pub const STRING: NodeType = NodeType(133);
pub const VERBATIM_DEF: NodeType = NodeType(134);
pub const EXAMPLE_DEF: NodeType = NodeType(135);
pub const AST_DEF: NodeType = NodeType(136);
pub const AST_NODE_DEF: NodeType = NodeType(137);
pub const AST_CLASS_DEF: NodeType = NodeType(138);
pub const METHOD_DEF: NodeType = NodeType(139);
pub const AST_SELECTOR: NodeType = NodeType(140);
pub const REF_EXPR: NodeType = NodeType(141);
pub const CALL_EXPR: NodeType = NodeType(142);
pub const SEQ_EXPR: NodeType = NodeType(143);
pub const BLOCK_EXPR: NodeType = NodeType(144);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR, WHITESPACE,
            EQ, PIPE, STAR, QUESTION, DOT, COMMA, HASH, LBRACE, RBRACE, LBRACK, RBRACK, LANGLE, RANGLE, LPAREN, RPAREN, KW_NODE, KW_CLASS, KW_TOKENIZER, KW_RULE, KW_VERBATIM, KW_AST, KW_PUB, KW_EXAMPLE, NUMBER, SIMPLE_STRING, HASH_STRING, IDENT, FALL_FILE, TOKENIZER_DEF, LEX_RULE, SYN_RULE, ATTRIBUTES, ATTRIBUTE, STRING, VERBATIM_DEF, EXAMPLE_DEF, AST_DEF, AST_NODE_DEF, AST_CLASS_DEF, METHOD_DEF, AST_SELECTOR, REF_EXPR, CALL_EXPR, SEQ_EXPR, BLOCK_EXPR,
        ];
        let parser_json = r##"[{"body":{"Pub":[29,{"Or":[{"And":[[{"Rep":{"WithSkip":[{"Rule":2},{"Rule":1}]}}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rule":3}],null]},{"And":[[{"Rule":5}],null]},{"And":[[{"Rule":9}],null]},{"And":[[{"Rule":11}],null]},{"And":[[{"Rule":10}],null]}]}},{"body":{"Or":[{"And":[[{"Token":19}],null]},{"And":[[{"Token":23}],null]},{"And":[[{"Token":20}],null]},{"And":[[{"Token":8}],null]},{"And":[[{"Token":21}],null]},{"And":[[{"Token":22}],null]},{"And":[[{"Token":24}],null]}]}},{"body":{"Pub":[30,{"Or":[{"And":[[{"Token":19},{"Token":9},{"Rep":{"Rule":4}},{"Token":10}],1]}]}]}},{"body":{"Pub":[31,{"Or":[{"And":[[{"Opt":{"Rule":6}},{"Token":28},{"Rule":8},{"Opt":{"Rule":8}}],2]}]}]}},{"body":{"Pub":[32,{"Or":[{"And":[[{"Opt":{"Rule":6}},{"Opt":{"Token":23}},{"Token":20},{"Token":28},{"Rule":21}],3]}]}]}},{"body":{"Pub":[33,{"Or":[{"And":[[{"Token":8},{"Token":11},{"Rep":{"Or":[{"And":[[{"Rule":7},{"Or":[{"And":[[{"Token":7}],null]},{"And":[[],null]}]}],null]}]}},{"Token":12}],null]}]}]}},{"body":{"Pub":[34,{"Or":[{"And":[[{"Token":28},{"Opt":{"Or":[{"And":[[{"Token":15},{"Token":25},{"Token":16}],null]}]}}],null]}]}]}},{"body":{"Pub":[35,{"Or":[{"And":[[{"Token":26}],null]},{"And":[[{"Token":27}],null]}]}]}},{"body":{"Pub":[36,{"Or":[{"And":[[{"Token":21},{"Token":27}],1]}]}]}},{"body":{"Pub":[37,{"Or":[{"And":[[{"Token":24},{"Token":27}],1]}]}]}},{"body":{"Pub":[38,{"Or":[{"And":[[{"Token":22},{"Token":9},{"Rep":{"WithSkip":[{"Or":[{"And":[[{"Token":17}],null]},{"And":[[{"Token":18}],null]}]},{"Or":[{"And":[[{"Rule":12}],null]},{"And":[[{"Rule":13}],null]}]}]}},{"Token":10}],1]}]}]}},{"body":{"Pub":[39,{"Or":[{"And":[[{"Token":17},{"Token":28},{"Token":9},{"Rep":{"Rule":14}},{"Token":10}],1]}]}]}},{"body":{"Pub":[40,{"Or":[{"And":[[{"Token":18},{"Token":28},{"Token":9},{"Layer":[{"Rule":22},{"Rep":{"Token":28}}]},{"Token":10}],1]}]}]}},{"body":{"Pub":[41,{"Or":[{"And":[[{"Token":28},{"Rule":15}],null]}]}]}},{"body":{"Pub":[42,{"Or":[{"And":[[{"Token":28},{"Opt":{"Rule":16}}],null]}]}]}},{"body":{"Or":[{"And":[[{"Token":5},{"Token":6},{"Token":28}],null]},{"And":[[{"Token":6},{"Token":28}],null]},{"And":[[{"Token":4}],null]},{"And":[[{"Token":5}],null]}]}},{"body":{"Or":[{"And":[[{"Rule":19}],null]},{"And":[[{"Rule":18}],null]},{"And":[[{"Rule":21}],null]}]}},{"body":{"Pub":[43,{"Or":[{"And":[[{"Token":28}],null]},{"And":[[{"Token":26}],null]}]}]}},{"body":{"Pub":[44,{"Or":[{"And":[[{"Token":13},{"Token":28},{"Rep":{"Rule":17}},{"Token":14}],null]}]}]}},{"body":{"Pub":[45,{"Or":[{"And":[[{"Rep":{"Rule":17}}],null]}]}]}},{"body":{"Pub":[46,{"Or":[{"And":[[{"Token":9},{"Layer":[{"Rule":22},{"Or":[{"And":[[{"Opt":{"Rule":20}},{"Rep":{"Or":[{"And":[[{"Token":3},{"Rule":20}],null]}]}}],null]}]}]},{"Token":10}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Rule":23}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":9},{"Rule":22},{"Token":10}],1]},{"And":[[{"Not":[10]}],null]}]}}]"##;
        let parser: Vec<SynRule> = serde_json::from_str(parser_json).unwrap();

        struct Impl { tokenizer: Vec<LexRule>, parser: Vec<SynRule> };
        impl LanguageImpl for Impl {
            fn parse(&self, text: &str) -> (FileStats, INode) {
                ::fall_parse::parse(text, &self.tokenizer, &|tokens, stats| {
                    Parser::new(ALL_NODE_TYPES, &self.parser).parse(tokens, stats)
                })
            }

            fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
                match ty {
                    ERROR => NodeTypeInfo { name: "ERROR" },
                    WHITESPACE => NodeTypeInfo { name: "WHITESPACE" },
                    EQ => NodeTypeInfo { name: "EQ" },
                    PIPE => NodeTypeInfo { name: "PIPE" },
                    STAR => NodeTypeInfo { name: "STAR" },
                    QUESTION => NodeTypeInfo { name: "QUESTION" },
                    DOT => NodeTypeInfo { name: "DOT" },
                    COMMA => NodeTypeInfo { name: "COMMA" },
                    HASH => NodeTypeInfo { name: "HASH" },
                    LBRACE => NodeTypeInfo { name: "LBRACE" },
                    RBRACE => NodeTypeInfo { name: "RBRACE" },
                    LBRACK => NodeTypeInfo { name: "LBRACK" },
                    RBRACK => NodeTypeInfo { name: "RBRACK" },
                    LANGLE => NodeTypeInfo { name: "LANGLE" },
                    RANGLE => NodeTypeInfo { name: "RANGLE" },
                    LPAREN => NodeTypeInfo { name: "LPAREN" },
                    RPAREN => NodeTypeInfo { name: "RPAREN" },
                    KW_NODE => NodeTypeInfo { name: "KW_NODE" },
                    KW_CLASS => NodeTypeInfo { name: "KW_CLASS" },
                    KW_TOKENIZER => NodeTypeInfo { name: "KW_TOKENIZER" },
                    KW_RULE => NodeTypeInfo { name: "KW_RULE" },
                    KW_VERBATIM => NodeTypeInfo { name: "KW_VERBATIM" },
                    KW_AST => NodeTypeInfo { name: "KW_AST" },
                    KW_PUB => NodeTypeInfo { name: "KW_PUB" },
                    KW_EXAMPLE => NodeTypeInfo { name: "KW_EXAMPLE" },
                    NUMBER => NodeTypeInfo { name: "NUMBER" },
                    SIMPLE_STRING => NodeTypeInfo { name: "SIMPLE_STRING" },
                    HASH_STRING => NodeTypeInfo { name: "HASH_STRING" },
                    IDENT => NodeTypeInfo { name: "IDENT" },
                    FALL_FILE => NodeTypeInfo { name: "FALL_FILE" },
                    TOKENIZER_DEF => NodeTypeInfo { name: "TOKENIZER_DEF" },
                    LEX_RULE => NodeTypeInfo { name: "LEX_RULE" },
                    SYN_RULE => NodeTypeInfo { name: "SYN_RULE" },
                    ATTRIBUTES => NodeTypeInfo { name: "ATTRIBUTES" },
                    ATTRIBUTE => NodeTypeInfo { name: "ATTRIBUTE" },
                    STRING => NodeTypeInfo { name: "STRING" },
                    VERBATIM_DEF => NodeTypeInfo { name: "VERBATIM_DEF" },
                    EXAMPLE_DEF => NodeTypeInfo { name: "EXAMPLE_DEF" },
                    AST_DEF => NodeTypeInfo { name: "AST_DEF" },
                    AST_NODE_DEF => NodeTypeInfo { name: "AST_NODE_DEF" },
                    AST_CLASS_DEF => NodeTypeInfo { name: "AST_CLASS_DEF" },
                    METHOD_DEF => NodeTypeInfo { name: "METHOD_DEF" },
                    AST_SELECTOR => NodeTypeInfo { name: "AST_SELECTOR" },
                    REF_EXPR => NodeTypeInfo { name: "REF_EXPR" },
                    CALL_EXPR => NodeTypeInfo { name: "CALL_EXPR" },
                    SEQ_EXPR => NodeTypeInfo { name: "SEQ_EXPR" },
                    BLOCK_EXPR => NodeTypeInfo { name: "BLOCK_EXPR" },
                    _ => panic!("Unknown NodeType: {:?}", ty)
                }
            }
        }

        Language::new(Impl {
            tokenizer: vec![
                LexRule::new(EQ, "=", None),
                LexRule::new(PIPE, "\\|", None),
                LexRule::new(STAR, "\\*", None),
                LexRule::new(QUESTION, "\\?", None),
                LexRule::new(DOT, "\\.", None),
                LexRule::new(COMMA, ",", None),
                LexRule::new(HASH, "\\#", None),
                LexRule::new(LBRACE, "\\{", None),
                LexRule::new(RBRACE, "\\}", None),
                LexRule::new(LBRACK, "\\[", None),
                LexRule::new(RBRACK, "\\]", None),
                LexRule::new(LANGLE, "<", None),
                LexRule::new(RANGLE, ">", None),
                LexRule::new(LPAREN, "\\(", None),
                LexRule::new(RPAREN, "\\)", None),
                LexRule::new(KW_NODE, "node", None),
                LexRule::new(KW_CLASS, "class", None),
                LexRule::new(KW_TOKENIZER, "tokenizer", None),
                LexRule::new(KW_RULE, "rule", None),
                LexRule::new(KW_VERBATIM, "verbatim", None),
                LexRule::new(KW_AST, "ast", None),
                LexRule::new(KW_PUB, "pub", None),
                LexRule::new(KW_EXAMPLE, "example", None),
                LexRule::new(WHITESPACE, "\\s+", None),
                LexRule::new(NUMBER, "\\d+", None),
                LexRule::new(SIMPLE_STRING, "\'([^\'\\\\]|\\\\.)*\'", None),
                LexRule::new(HASH_STRING, "r#*", Some(parse_raw_string)),
                LexRule::new(IDENT, "\\w+", None),
            ],
            parser: parser,
        })
    };
}
fn parse_raw_string(s: &str) -> Option<usize> {
    let quote_start = s.find('"').unwrap();
    let q_hashes = concat!('"', "######", "######", "######", "######", "######");
    let closing = &q_hashes[..quote_start];
    s[quote_start + 1..].find(closing).map(|i| i + quote_start + 1 + closing.len())
}

use fall_tree::{Text, AstNode, AstChildren, AstClass, AstClassChildren, Node};
use fall_tree::search::{child_of_type_exn, child_of_type};

#[derive(Clone, Copy)]
pub struct FallFile<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for FallFile<'f> {
    fn ty() -> NodeType { FALL_FILE }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        FallFile { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> FallFile<'f> {
    pub fn tokenizer_def(&self) -> Option<TokenizerDef<'f>> {
        AstChildren::new(self.node.children()).next()
    }
    pub fn syn_rules(&self) -> AstChildren<'f, SynRule<'f>> {
        AstChildren::new(self.node.children())
    }
    pub fn verbatim_def(&self) -> Option<VerbatimDef<'f>> {
        AstChildren::new(self.node.children()).next()
    }
    pub fn ast_def(&self) -> Option<AstDef<'f>> {
        AstChildren::new(self.node.children()).next()
    }
    pub fn examples(&self) -> AstChildren<'f, ExampleDef<'f>> {
        AstChildren::new(self.node.children())
    }
}
#[derive(Clone, Copy)]
pub struct TokenizerDef<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for TokenizerDef<'f> {
    fn ty() -> NodeType { TOKENIZER_DEF }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        TokenizerDef { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> TokenizerDef<'f> {
    pub fn lex_rules(&self) -> AstChildren<'f, LexRule<'f>> {
        AstChildren::new(self.node.children())
    }
}
#[derive(Clone, Copy)]
pub struct LexRule<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for LexRule<'f> {
    fn ty() -> NodeType { LEX_RULE }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        LexRule { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> LexRule<'f> {
    pub fn attributes(&self) -> Option<Attributes<'f>> {
        AstChildren::new(self.node.children()).next()
    }
    pub fn node_type(&self) -> Text<'f> {
        child_of_type_exn(self.node, IDENT).text()
    }
}
#[derive(Clone, Copy)]
pub struct SynRule<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for SynRule<'f> {
    fn ty() -> NodeType { SYN_RULE }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        SynRule { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> SynRule<'f> {
    pub fn attributes(&self) -> Option<Attributes<'f>> {
        AstChildren::new(self.node.children()).next()
    }
    pub fn name(&self) -> Option<Text<'f>> {
        child_of_type(self.node, IDENT).map(|n| n.text())
    }
    pub fn body(&self) -> Expr<'f> {
        AstClassChildren::new(self.node.children()).next().unwrap()
    }
}
#[derive(Clone, Copy)]
pub struct Attributes<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for Attributes<'f> {
    fn ty() -> NodeType { ATTRIBUTES }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        Attributes { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> Attributes<'f> {
    pub fn attributes(&self) -> AstChildren<'f, Attribute<'f>> {
        AstChildren::new(self.node.children())
    }
}
#[derive(Clone, Copy)]
pub struct Attribute<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for Attribute<'f> {
    fn ty() -> NodeType { ATTRIBUTE }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        Attribute { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> Attribute<'f> {
    pub fn name(&self) -> Text<'f> {
        child_of_type_exn(self.node, IDENT).text()
    }
    pub fn value(&self) -> Option<Text<'f>> {
        child_of_type(self.node, NUMBER).map(|n| n.text())
    }
}
#[derive(Clone, Copy)]
pub struct VerbatimDef<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for VerbatimDef<'f> {
    fn ty() -> NodeType { VERBATIM_DEF }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        VerbatimDef { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> VerbatimDef<'f> {
    
}
#[derive(Clone, Copy)]
pub struct AstDef<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for AstDef<'f> {
    fn ty() -> NodeType { AST_DEF }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        AstDef { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> AstDef<'f> {
    pub fn ast_nodes(&self) -> AstChildren<'f, AstNodeDef<'f>> {
        AstChildren::new(self.node.children())
    }
    pub fn ast_classes(&self) -> AstChildren<'f, AstClassDef<'f>> {
        AstChildren::new(self.node.children())
    }
}
#[derive(Clone, Copy)]
pub struct AstNodeDef<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for AstNodeDef<'f> {
    fn ty() -> NodeType { AST_NODE_DEF }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        AstNodeDef { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> AstNodeDef<'f> {
    pub fn name(&self) -> Text<'f> {
        child_of_type_exn(self.node, IDENT).text()
    }
    pub fn methods(&self) -> AstChildren<'f, MethodDef<'f>> {
        AstChildren::new(self.node.children())
    }
}
#[derive(Clone, Copy)]
pub struct AstClassDef<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for AstClassDef<'f> {
    fn ty() -> NodeType { AST_CLASS_DEF }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        AstClassDef { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> AstClassDef<'f> {
    
}
#[derive(Clone, Copy)]
pub struct MethodDef<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for MethodDef<'f> {
    fn ty() -> NodeType { METHOD_DEF }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        MethodDef { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> MethodDef<'f> {
    pub fn name(&self) -> Text<'f> {
        child_of_type_exn(self.node, IDENT).text()
    }
}
#[derive(Clone, Copy)]
pub struct RefExpr<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for RefExpr<'f> {
    fn ty() -> NodeType { REF_EXPR }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        RefExpr { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> RefExpr<'f> {
    
}
#[derive(Clone, Copy)]
pub struct CallExpr<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for CallExpr<'f> {
    fn ty() -> NodeType { CALL_EXPR }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        CallExpr { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> CallExpr<'f> {
    pub fn fn_name(&self) -> Text<'f> {
        child_of_type_exn(self.node, IDENT).text()
    }
    pub fn args(&self) -> AstClassChildren<'f, Expr<'f>> {
        AstClassChildren::new(self.node.children())
    }
}
#[derive(Clone, Copy)]
pub struct SeqExpr<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for SeqExpr<'f> {
    fn ty() -> NodeType { SEQ_EXPR }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        SeqExpr { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> SeqExpr<'f> {
    pub fn parts(&self) -> AstClassChildren<'f, Expr<'f>> {
        AstClassChildren::new(self.node.children())
    }
}
#[derive(Clone, Copy)]
pub struct BlockExpr<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for BlockExpr<'f> {
    fn ty() -> NodeType { BLOCK_EXPR }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        BlockExpr { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> BlockExpr<'f> {
    pub fn alts(&self) -> AstClassChildren<'f, Expr<'f>> {
        AstClassChildren::new(self.node.children())
    }
}
#[derive(Clone, Copy)]
pub struct ExampleDef<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for ExampleDef<'f> {
    fn ty() -> NodeType { EXAMPLE_DEF }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        ExampleDef { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> ExampleDef<'f> {
    
}

#[derive(Clone, Copy)]
pub enum Expr<'f> {
    RefExpr(RefExpr<'f>),
    CallExpr(CallExpr<'f>),
    SeqExpr(SeqExpr<'f>),
    BlockExpr(BlockExpr<'f>),
}

impl<'f> AstClass<'f> for Expr<'f> {
    fn tys() -> &'static [NodeType] {
        const TYS: &[NodeType] = &[
            REF_EXPR,
            CALL_EXPR,
            SEQ_EXPR,
            BLOCK_EXPR,
        ];
        TYS
    }

    fn new(node: Node<'f>) -> Self {
        match node.ty() {
            REF_EXPR => Expr::RefExpr(RefExpr::new(node)),
            CALL_EXPR => Expr::CallExpr(CallExpr::new(node)),
            SEQ_EXPR => Expr::SeqExpr(SeqExpr::new(node)),
            BLOCK_EXPR => Expr::BlockExpr(BlockExpr::new(node)),
            _ => panic!("Bad ast class")
        }
    }

    fn node(&self) -> Node<'f> {
        match *self {
            Expr::RefExpr(n) => n.node(),
            Expr::CallExpr(n) => n.node(),
            Expr::SeqExpr(n) => n.node(),
            Expr::BlockExpr(n) => n.node(),
        }
    }
}