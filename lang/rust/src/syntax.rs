use fall_parse::runtime::*;
use self::fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl, FileStats, INode};
pub use self::fall_tree::ERROR;

pub const WHITESPACE: NodeType = NodeType(100);
pub const UNION: NodeType = NodeType(101);
pub const AS: NodeType = NodeType(102);
pub const CRATE: NodeType = NodeType(103);
pub const EXTERN: NodeType = NodeType(104);
pub const FN: NodeType = NodeType(105);
pub const LET: NodeType = NodeType(106);
pub const PUB: NodeType = NodeType(107);
pub const STRUCT: NodeType = NodeType(108);
pub const USE: NodeType = NodeType(109);
pub const MOD: NodeType = NodeType(110);
pub const IF: NodeType = NodeType(111);
pub const ELSE: NodeType = NodeType(112);
pub const ENUM: NodeType = NodeType(113);
pub const IMPL: NodeType = NodeType(114);
pub const SELF: NodeType = NodeType(115);
pub const SUPER: NodeType = NodeType(116);
pub const TYPE: NodeType = NodeType(117);
pub const CONST: NodeType = NodeType(118);
pub const FOR: NodeType = NodeType(119);
pub const L_PAREN: NodeType = NodeType(120);
pub const R_PAREN: NodeType = NodeType(121);
pub const L_CURLY: NodeType = NodeType(122);
pub const R_CURLY: NodeType = NodeType(123);
pub const L_ANGLE: NodeType = NodeType(124);
pub const R_ANGLE: NodeType = NodeType(125);
pub const L_BRACK: NodeType = NodeType(126);
pub const R_BRACK: NodeType = NodeType(127);
pub const SHL: NodeType = NodeType(128);
pub const SHR: NodeType = NodeType(129);
pub const THIN_ARROW: NodeType = NodeType(130);
pub const EQ: NodeType = NodeType(131);
pub const SEMI: NodeType = NodeType(132);
pub const COLON: NodeType = NodeType(133);
pub const COLONCOLON: NodeType = NodeType(134);
pub const COMMA: NodeType = NodeType(135);
pub const DOT: NodeType = NodeType(136);
pub const HASH: NodeType = NodeType(137);
pub const STAR: NodeType = NodeType(138);
pub const SLASH: NodeType = NodeType(139);
pub const PERCENT: NodeType = NodeType(140);
pub const PLUS: NodeType = NodeType(141);
pub const MINUS: NodeType = NodeType(142);
pub const AMPERSAND: NodeType = NodeType(143);
pub const PIPE: NodeType = NodeType(144);
pub const UNDERSCORE: NodeType = NodeType(145);
pub const LIFETIME: NodeType = NodeType(146);
pub const IDENT: NodeType = NodeType(147);
pub const NUMBER: NodeType = NodeType(148);
pub const STRING: NodeType = NodeType(149);
pub const FILE: NodeType = NodeType(150);
pub const USE_DECL: NodeType = NodeType(151);
pub const USE_SPEC: NodeType = NodeType(152);
pub const USE_SPEC_ENTRY: NodeType = NodeType(153);
pub const EXTERN_CRATE_DECL: NodeType = NodeType(154);
pub const FN_DEF: NodeType = NodeType(155);
pub const VALUE_PARAM: NodeType = NodeType(156);
pub const LAMBDA_VALUE_PARAM: NodeType = NodeType(157);
pub const SELF_PARAMETER: NodeType = NodeType(158);
pub const STRUCT_DEF: NodeType = NodeType(159);
pub const STRUCT_FIELD: NodeType = NodeType(160);
pub const TUPLE_FIELD: NodeType = NodeType(161);
pub const ENUM_DEF: NodeType = NodeType(162);
pub const ENUM_VARIANT: NodeType = NodeType(163);
pub const MOD_DEF: NodeType = NodeType(164);
pub const IMPL_DEF: NodeType = NodeType(165);
pub const TYPE_DEF: NodeType = NodeType(166);
pub const CONST_DEF: NodeType = NodeType(167);
pub const TYPE_PARAMETERS: NodeType = NodeType(168);
pub const TYPE_PARAMETER: NodeType = NodeType(169);
pub const LIFETIME_PARAMETER: NodeType = NodeType(170);
pub const VISIBILITY: NodeType = NodeType(171);
pub const PATH: NodeType = NodeType(172);
pub const TYPE_ARGUMENTS: NodeType = NodeType(173);
pub const ALIAS: NodeType = NodeType(174);
pub const TYPE_REFERENCE: NodeType = NodeType(175);
pub const PATTERN: NodeType = NodeType(176);
pub const EXPR: NodeType = NodeType(177);
pub const LITERAL: NodeType = NodeType(178);
pub const STRUCT_LITERAL: NodeType = NodeType(179);
pub const STRUCT_LITERAL_FIELD: NodeType = NodeType(180);
pub const PATH_EXPR: NodeType = NodeType(181);
pub const PAREN_EXPR: NodeType = NodeType(182);
pub const LAMBDA_EXPR: NodeType = NodeType(183);
pub const BLOCK_EXPR: NodeType = NodeType(184);
pub const LET_STMT: NodeType = NodeType(185);
pub const EMPTY_STMT: NodeType = NodeType(186);
pub const EXPR_STMT: NodeType = NodeType(187);
pub const IF_EXPR: NodeType = NodeType(188);
pub const CALL_EXPR: NodeType = NodeType(189);
pub const FIELD_EXPR: NodeType = NodeType(190);
pub const VALUE_ARGUMENT: NodeType = NodeType(191);
pub const REFERENCE_EXPR: NodeType = NodeType(192);
pub const DEREFERENCE_EXPR: NodeType = NodeType(193);
pub const NEGATION_EXPR: NodeType = NodeType(194);
pub const PRODUCT_EXPR: NodeType = NodeType(195);
pub const SUM_EXPR: NodeType = NodeType(196);
pub const BIT_SHIFT: NodeType = NodeType(197);
pub const BIT_AND: NodeType = NodeType(198);
pub const BIT_OR: NodeType = NodeType(199);
pub const ATTRIBUTE: NodeType = NodeType(200);
pub const ATTR_VALUE: NodeType = NodeType(201);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR,
            WHITESPACE, UNION, AS, CRATE, EXTERN, FN, LET, PUB, STRUCT, USE, MOD, IF, ELSE, ENUM, IMPL, SELF, SUPER, TYPE, CONST, FOR, L_PAREN, R_PAREN, L_CURLY, R_CURLY, L_ANGLE, R_ANGLE, L_BRACK, R_BRACK, SHL, SHR, THIN_ARROW, EQ, SEMI, COLON, COLONCOLON, COMMA, DOT, HASH, STAR, SLASH, PERCENT, PLUS, MINUS, AMPERSAND, PIPE, UNDERSCORE, LIFETIME, IDENT, NUMBER, STRING, FILE, USE_DECL, USE_SPEC, USE_SPEC_ENTRY, EXTERN_CRATE_DECL, FN_DEF, VALUE_PARAM, LAMBDA_VALUE_PARAM, SELF_PARAMETER, STRUCT_DEF, STRUCT_FIELD, TUPLE_FIELD, ENUM_DEF, ENUM_VARIANT, MOD_DEF, IMPL_DEF, TYPE_DEF, CONST_DEF, TYPE_PARAMETERS, TYPE_PARAMETER, LIFETIME_PARAMETER, VISIBILITY, PATH, TYPE_ARGUMENTS, ALIAS, TYPE_REFERENCE, PATTERN, EXPR, LITERAL, STRUCT_LITERAL, STRUCT_LITERAL_FIELD, PATH_EXPR, PAREN_EXPR, LAMBDA_EXPR, BLOCK_EXPR, LET_STMT, EMPTY_STMT, EXPR_STMT, IF_EXPR, CALL_EXPR, FIELD_EXPR, VALUE_ARGUMENT, REFERENCE_EXPR, DEREFERENCE_EXPR, NEGATION_EXPR, PRODUCT_EXPR, SUM_EXPR, BIT_SHIFT, BIT_AND, BIT_OR, ATTRIBUTE, ATTR_VALUE,
        ];
        let parser_json = r##"[{"body":{"Pub":[51,{"Or":[{"And":[[{"Rule":1}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rep":{"WithSkip":[{"Rule":2},{"Rule":3}]}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":8}],null]},{"And":[[{"Token":5}],null]},{"And":[[{"Token":38}],null]},{"And":[[{"Token":6}],null]},{"And":[[{"Token":9}],null]},{"And":[[{"ContextualToken":[2,"union"]}],null]},{"And":[[{"Token":14}],null]},{"And":[[{"Token":10}],null]},{"And":[[{"Token":11}],null]},{"And":[[{"Token":15}],null]},{"And":[[{"Token":18}],null]},{"And":[[{"Token":19}],null]}]}},{"body":{"Or":[{"And":[[{"Rule":4}],null]},{"And":[[{"Rule":7}],null]},{"And":[[{"Rule":8}],null]},{"And":[[{"Rule":13}],null]},{"And":[[{"Rule":16}],null]},{"And":[[{"Rule":18}],null]},{"And":[[{"Rule":19}],null]},{"And":[[{"Rule":22}],null]},{"And":[[{"Rule":23}],null]}]}},{"body":{"Pub":[52,{"Or":[{"And":[[{"Rule":68},{"Opt":{"Rule":27}},{"Token":10},{"Or":[{"And":[[{"Rule":28},{"Or":[{"And":[[{"Rule":36}],null]},{"And":[[{"Opt":{"Or":[{"And":[[{"Token":35},{"Rule":5}],null]}]}}],null]}]}],null]},{"And":[[{"Opt":{"Token":35}},{"Rule":5}],null]}]},{"Token":33}],3]}]}]}},{"body":{"Pub":[53,{"Or":[{"And":[[{"Token":39}],null]},{"And":[[{"Call":[{"Rule":71},[[3,{"Call":[{"Rule":70},[[2,{"Rule":6}]]]}]]]}],null]}]}]}},{"body":{"Pub":[54,{"Or":[{"And":[[{"Token":48},{"Opt":{"Rule":36}}],1]}]}]}},{"body":{"Pub":[55,{"Or":[{"And":[[{"Rule":68},{"Token":5},{"Token":4},{"Token":48},{"Opt":{"Rule":36}},{"Token":33}],3]}]}]}},{"body":{"Pub":[56,{"Or":[{"And":[[{"Rule":68},{"Opt":{"Rule":27}},{"Token":6},{"Token":48},{"Opt":{"Rule":24}},{"Rule":9},{"Opt":{"Or":[{"And":[[{"Token":31},{"Rule":37}],null]}]}},{"Rule":49}],3]}]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":72},[[4,{"Or":[{"And":[[{"Opt":{"Rule":12}},{"Call":[{"Rule":70},[[2,{"Rule":10}]]]}],null]}]}]]]}],null]}]}},{"body":{"Pub":[57,{"Or":[{"And":[[{"Rule":38},{"Token":34},{"Rule":37}],1]}]}]}},{"body":{"Pub":[58,{"Or":[{"And":[[{"Rule":38},{"Opt":{"Or":[{"And":[[{"Token":34},{"Rule":37}],null]}]}}],null]}]}]}},{"body":{"Pub":[59,{"Or":[{"And":[[{"Opt":{"Token":44}},{"Token":16},{"Or":[{"And":[[{"Token":36}],null]},{"And":[["Eof"],null]}]}],2]}]}]}},{"body":{"Pub":[60,{"Or":[{"And":[[{"Rule":68},{"Opt":{"Rule":27}},{"Or":[{"And":[[{"Token":9}],null]},{"And":[[{"ContextualToken":[2,"union"]}],null]}]},{"Token":48},{"Opt":{"Rule":24}},{"Or":[{"And":[[{"Call":[{"Rule":71},[[3,{"Call":[{"Rule":70},[[2,{"Rule":14}]]]}]]]}],null]},{"And":[[{"Token":33}],null]},{"And":[[{"Call":[{"Rule":72},[[4,{"Call":[{"Rule":70},[[2,{"Rule":15}]]]}]]]},{"Token":33}],null]}]}],3]}]}]}},{"body":{"Pub":[61,{"Or":[{"And":[[{"Opt":{"Rule":27}},{"Token":48},{"Token":34},{"Rule":37}],2]}]}]}},{"body":{"Pub":[62,{"Or":[{"And":[[{"Opt":{"Rule":27}},{"Rule":37}],null]}]}]}},{"body":{"Pub":[63,{"Or":[{"And":[[{"Rule":68},{"Opt":{"Rule":27}},{"Token":14},{"Token":48},{"Call":[{"Rule":71},[[3,{"Call":[{"Rule":70},[[2,{"Rule":17}]]]}]]]}],3]}]}]}},{"body":{"Pub":[64,{"Or":[{"And":[[{"Token":48},{"Opt":{"Or":[{"And":[[{"Token":32},{"Rule":39}],null]},{"And":[[{"Call":[{"Rule":72},[[4,{"Call":[{"Rule":70},[[2,{"Rule":15}]]]}]]]}],null]},{"And":[[{"Call":[{"Rule":71},[[3,{"Call":[{"Rule":70},[[2,{"Rule":14}]]]}]]]}],null]}]}}],1]}]}]}},{"body":{"Pub":[65,{"Or":[{"And":[[{"Rule":68},{"Opt":{"Rule":27}},{"Token":11},{"Token":48},{"Or":[{"And":[[{"Token":33}],null]},{"And":[[{"Call":[{"Rule":71},[[3,{"Rule":1}]]]}],null]}]}],3]}]}]}},{"body":{"Pub":[66,{"Or":[{"And":[[{"Rule":68},{"Token":15},{"Opt":{"Rule":24}},{"Or":[{"And":[[{"Rule":37},{"Opt":{"Or":[{"And":[[{"Token":20},{"Rule":37}],null]}]}}],null]}]},{"Call":[{"Rule":71},[[3,{"Rep":{"WithSkip":[{"Rule":21},{"Rule":20}]}}]]]}],2]}]}]}},{"body":{"Or":[{"And":[[{"Rule":8}],null]},{"And":[[{"Rule":22}],null]}]}},{"body":{"Or":[{"And":[[{"Token":8}],null]},{"And":[[{"Token":6}],null]},{"And":[[{"Token":18}],null]}]}},{"body":{"Pub":[67,{"Or":[{"And":[[{"Rule":68},{"Opt":{"Rule":27}},{"Token":18},{"Token":48},{"Opt":{"Rule":24}},{"Opt":{"Or":[{"And":[[{"Token":32},{"Rule":37}],null]}]}},{"Token":33}],3]}]}]}},{"body":{"Pub":[68,{"Or":[{"And":[[{"Rule":68},{"Opt":{"Rule":27}},{"Token":19},{"Token":48},{"Token":34},{"Rule":37},{"Token":32},{"Rule":39},{"Token":33}],3]}]}]}},{"body":{"Pub":[69,{"Or":[{"And":[[{"Call":[{"Rule":73},[[5,{"Or":[{"And":[[{"Call":[{"Rule":70},[[2,{"Rule":26}]]]},{"Call":[{"Rule":70},[[2,{"Rule":25}]]]}],null]}]}]]]}],null]}]}]}},{"body":{"Pub":[70,{"Or":[{"And":[[{"Token":48}],null]}]}]}},{"body":{"Pub":[71,{"Or":[{"And":[[{"Token":47}],null]}]}]}},{"body":{"Pub":[72,{"Or":[{"And":[[{"Token":8}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rule":31}],null]}]}},{"body":{"Or":[{"And":[[{"Enter":[0,{"Rule":31}]}],null]}]}},{"body":{"Or":[{"And":[[{"Enter":[1,{"Rule":31}]}],null]}]}},{"body":{"Pratt":[{"Atom":{"body":{"Pub":[73,{"Or":[{"And":[[{"Opt":{"Token":35}},{"Rule":34}],null]}]}]}}},{"Postfix":{"ty":73,"op":{"Or":[{"And":[[{"Token":35},{"Rule":34}],null]}]}}}]}},{"body":{"Pub":[73,{"Or":[{"And":[[{"Opt":{"Token":35}},{"Rule":34}],null]}]}]}},{"body":{"Pub":[73,{"Or":[{"And":[[{"Rule":31},{"Or":[{"And":[[{"Token":35},{"Rule":34}],null]}]}],null]}]}]}},{"body":{"Or":[{"And":[[{"Or":[{"And":[[{"Token":48}],null]},{"And":[[{"Token":16}],null]},{"And":[[{"Token":17}],null]}]},{"Opt":{"Rule":35}}],null]}]}},{"body":{"Pub":[74,{"Or":[{"And":[[{"Or":[{"And":[[{"IsIn":0}],null]},{"And":[[{"IsIn":1},{"Token":35}],null]}]},{"Call":[{"Rule":73},[[5,{"Or":[{"And":[[{"Call":[{"Rule":70},[[2,{"Token":47}]]]},{"Call":[{"Rule":70},[[2,{"Rule":37}]]]}],null]}]}]]]}],null]}]}]}},{"body":{"Pub":[75,{"Or":[{"And":[[{"Token":3},{"Token":48}],null]}]}]}},{"body":{"Pub":[76,{"Or":[{"And":[[{"Rule":29}],null]},{"And":[[{"Token":44},{"Opt":{"Token":47}},{"Rule":37}],null]}]}]}},{"body":{"Pub":[77,{"Or":[{"And":[[{"Token":48}],null]},{"And":[[{"Token":46}],null]}]}]}},{"body":{"Pratt":[{"Atom":{"body":{"Pub":[79,{"Or":[{"And":[[{"Token":49}],null]},{"And":[[{"Token":50}],null]}]}]}}},{"Atom":{"body":{"Pub":[80,{"Or":[{"And":[[{"Not":{"IsIn":5}},{"Rule":30},{"Call":[{"Rule":71},[[3,{"Call":[{"Rule":70},[[2,{"Rule":44}]]]}]]]}],null]}]}]}}},{"Atom":{"body":{"Pub":[82,{"Or":[{"And":[[{"Rule":30}],null]}]}]}}},{"Atom":{"body":{"Pub":[83,{"Or":[{"And":[[{"Token":21},{"Call":[{"Rule":41},[[0,{"Rule":39}]]]},{"Token":22}],null]}]}]}}},{"Atom":{"body":{"Pub":[84,{"Or":[{"And":[[{"Token":45},{"Rep":{"Rule":48}},{"Token":45},{"Or":[{"And":[[{"Token":31},{"Rule":37},{"Rule":49}],null]},{"And":[[{"Call":[{"Rule":41},[[0,{"Rule":39}]]]}],null]}]}],null]}]}]}}},{"Atom":{"body":{"Pub":[85,{"Or":[{"And":[[{"Call":[{"Rule":41},[[0,{"Call":[{"Rule":71},[[3,{"Or":[{"And":[[{"Rep":{"Rule":50}},{"Opt":{"Rule":39}}],null]}]}]]]}]]]}],null]}]}]}}},{"Atom":{"body":{"Pub":[89,{"Or":[{"And":[[{"Token":12},{"Enter":[5,{"Rule":39}]},{"Rule":49},{"Opt":{"Or":[{"And":[[{"Token":13},{"Rule":49}],null]}]}}],1]}]}]}}},{"Postfix":{"ty":90,"op":{"Call":[{"Rule":41},[[0,{"Call":[{"Rule":72},[[4,{"Call":[{"Rule":70},[[2,{"Rule":57}]]]}]]]}]]]}}},{"Postfix":{"ty":91,"op":{"Or":[{"And":[[{"Token":37},{"Or":[{"And":[[{"Token":48}],null]},{"And":[[{"Token":49}],null]}]}],null]}]}}},{"Prefix":{"ty":93,"op":{"Token":44}}},{"Prefix":{"ty":94,"op":{"Token":39}}},{"Prefix":{"ty":95,"op":{"Token":43}}},{"Binary":{"ty":96,"op":{"Call":[{"Rule":64},[[1,{"Or":[{"And":[[{"Token":39}],null]},{"And":[[{"Token":40}],null]},{"And":[[{"Token":41}],null]}]}]]]},"priority":5}},{"Binary":{"ty":97,"op":{"Call":[{"Rule":64},[[1,{"Or":[{"And":[[{"Token":42}],null]},{"And":[[{"Token":43}],null]}]}]]]},"priority":4}},{"Binary":{"ty":98,"op":{"Call":[{"Rule":64},[[1,{"Or":[{"And":[[{"ContextualToken":[29,"<<"]}],null]},{"And":[[{"ContextualToken":[30,">>"]}],null]}]}]]]},"priority":3}},{"Binary":{"ty":99,"op":{"Call":[{"Rule":64},[[1,{"Token":44}]]]},"priority":2}},{"Binary":{"ty":100,"op":{"Call":[{"Rule":64},[[1,{"Token":45}]]]},"priority":1}}]}},{"body":{"Or":[{"And":[[{"PrevIs":[89,85]}],null]}]}},{"body":{"Or":[{"And":[[{"Exit":[5,{"Exit":[4,{"Var":0}]}]}],null]}]}},{"body":{"Pub":[79,{"Or":[{"And":[[{"Token":49}],null]},{"And":[[{"Token":50}],null]}]}]}},{"body":{"Pub":[80,{"Or":[{"And":[[{"Not":{"IsIn":5}},{"Rule":30},{"Call":[{"Rule":71},[[3,{"Call":[{"Rule":70},[[2,{"Rule":44}]]]}]]]}],null]}]}]}},{"body":{"Pub":[81,{"Or":[{"And":[[{"Token":48},{"Token":34},{"Rule":39}],1]}]}]}},{"body":{"Pub":[82,{"Or":[{"And":[[{"Rule":30}],null]}]}]}},{"body":{"Pub":[83,{"Or":[{"And":[[{"Token":21},{"Call":[{"Rule":41},[[0,{"Rule":39}]]]},{"Token":22}],null]}]}]}},{"body":{"Pub":[84,{"Or":[{"And":[[{"Token":45},{"Rep":{"Rule":48}},{"Token":45},{"Or":[{"And":[[{"Token":31},{"Rule":37},{"Rule":49}],null]},{"And":[[{"Call":[{"Rule":41},[[0,{"Rule":39}]]]}],null]}]}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rule":11},{"Or":[{"And":[[{"Token":36}],null]},{"And":[[{"Not":{"Not":{"Token":45}}}],null]}]}],1]}]}},{"body":{"Pub":[85,{"Or":[{"And":[[{"Call":[{"Rule":41},[[0,{"Call":[{"Rule":71},[[3,{"Or":[{"And":[[{"Rep":{"Rule":50}},{"Opt":{"Rule":39}}],null]}]}]]]}]]]}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rule":51}],null]},{"And":[[{"Rule":53}],null]},{"And":[[{"Rule":52}],null]}]}},{"body":{"Pub":[86,{"Or":[{"And":[[{"Token":7},{"Rule":38},{"Token":32},{"Rule":39},{"Token":33}],1]}]}]}},{"body":{"Pub":[87,{"Or":[{"And":[[{"Token":33}],null]}]}]}},{"body":{"Pub":[88,{"Or":[{"And":[[{"Enter":[4,{"Or":[{"And":[[{"Rule":39},{"Or":[{"And":[[{"Rule":40},{"Not":"Eof"}],null]},{"And":[[{"Token":33}],null]}]}],null]}]}]}],null]}]}]}},{"body":{"Pub":[89,{"Or":[{"And":[[{"Token":12},{"Enter":[5,{"Rule":39}]},{"Rule":49},{"Opt":{"Or":[{"And":[[{"Token":13},{"Rule":49}],null]}]}}],1]}]}]}},{"body":{"Pub":[90,{"Or":[{"And":[[{"Rule":39},{"Call":[{"Rule":41},[[0,{"Call":[{"Rule":72},[[4,{"Call":[{"Rule":70},[[2,{"Rule":57}]]]}]]]}]]]}],null]}]}]}},{"body":{"Pub":[91,{"Or":[{"And":[[{"Rule":39},{"Or":[{"And":[[{"Token":37},{"Or":[{"And":[[{"Token":48}],null]},{"And":[[{"Token":49}],null]}]}],null]}]}],null]}]}]}},{"body":{"Pub":[92,{"Or":[{"And":[[{"Rule":39}],null]}]}]}},{"body":{"Pub":[93,{"Or":[{"And":[[{"Token":44},{"Rule":39}],null]}]}]}},{"body":{"Pub":[94,{"Or":[{"And":[[{"Token":39},{"Rule":39}],null]}]}]}},{"body":{"Pub":[95,{"Or":[{"And":[[{"Token":43},{"Rule":39}],null]}]}]}},{"body":{"Pub":[96,{"Or":[{"And":[[{"Rule":39},{"Call":[{"Rule":64},[[1,{"Or":[{"And":[[{"Token":39}],null]},{"And":[[{"Token":40}],null]},{"And":[[{"Token":41}],null]}]}]]]},{"Rule":39}],null]}]}]}},{"body":{"Pub":[97,{"Or":[{"And":[[{"Rule":39},{"Call":[{"Rule":64},[[1,{"Or":[{"And":[[{"Token":42}],null]},{"And":[[{"Token":43}],null]}]}]]]},{"Rule":39}],null]}]}]}},{"body":{"Pub":[98,{"Or":[{"And":[[{"Rule":39},{"Call":[{"Rule":64},[[1,{"Or":[{"And":[[{"ContextualToken":[29,"<<"]}],null]},{"And":[[{"ContextualToken":[30,">>"]}],null]}]}]]]},{"Rule":39}],null]}]}]}},{"body":{"Or":[{"And":[[{"IsIn":4},{"Not":{"Rule":40}},{"Var":1}],null]},{"And":[[{"Not":{"IsIn":4}},{"Var":1}],null]}]}},{"body":{"Pub":[99,{"Or":[{"And":[[{"Rule":39},{"Call":[{"Rule":64},[[1,{"Token":44}]]]},{"Rule":39}],null]}]}]}},{"body":{"Pub":[100,{"Or":[{"And":[[{"Rule":39},{"Call":[{"Rule":64},[[1,{"Token":45}]]]},{"Rule":39}],null]}]}]}},{"body":{"Pub":[101,{"Or":[{"And":[[{"Token":38},{"Call":[{"Rule":74},[[6,{"Call":[{"Rule":70},[[2,{"Rule":69}]]]}]]]}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Rule":67}}],null]}]}},{"body":{"Pub":[102,{"Or":[{"And":[[{"Token":48},{"Opt":{"Or":[{"And":[[{"Token":32},{"Rule":39}],null]},{"And":[[{"Call":[{"Rule":72},[[4,{"Call":[{"Rule":70},[[2,{"Rule":69}]]]}]]]}],null]}]}}],1]}]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Var":2},{"Or":[{"And":[["Eof"],null]},{"And":[[{"Token":36}],null]}]}],1]}]}}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":75},[[7,{"Token":23}],[8,{"Token":24}],[9,{"Var":3}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":75},[[7,{"Token":21}],[8,{"Token":22}],[9,{"Var":4}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":75},[[7,{"Token":25}],[8,{"Token":26}],[9,{"Var":5}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":75},[[7,{"Token":27}],[8,{"Token":28}],[9,{"Var":6}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Var":7},{"Layer":[{"Call":[{"Rule":76},[[10,{"Var":7}],[11,{"Var":8}]]]},{"Var":9}]},{"Var":8}],1]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Var":10},{"Call":[{"Rule":76},[[10,{"Var":10}],[11,{"Var":11}]]]},{"Var":11}],1]},{"And":[[{"Or":[{"And":[[{"Not":{"Var":11}},"Any"],null]}]}],null]}]}}],null]}]}}]"##;
        let parser: Vec<SynRule> = serde_json::from_str(parser_json).unwrap();

        struct Impl { tokenizer: Vec<LexRule>, parser: Vec<SynRule> };
        impl LanguageImpl for Impl {
            fn parse(&self, text: &str) -> (FileStats, INode) {
                parse(
                    text,
                    &LANG,
                    &self.tokenizer,
                    &|tokens, stats| Parser::new(ALL_NODE_TYPES, &self.parser).parse(tokens, stats)
                )
            }

            fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
                match ty {
                    ERROR => NodeTypeInfo { name: "ERROR", whitespace_like: false },
                    WHITESPACE => NodeTypeInfo { name: "WHITESPACE", whitespace_like: true },
                    UNION => NodeTypeInfo { name: "UNION", whitespace_like: false },
                    AS => NodeTypeInfo { name: "AS", whitespace_like: false },
                    CRATE => NodeTypeInfo { name: "CRATE", whitespace_like: false },
                    EXTERN => NodeTypeInfo { name: "EXTERN", whitespace_like: false },
                    FN => NodeTypeInfo { name: "FN", whitespace_like: false },
                    LET => NodeTypeInfo { name: "LET", whitespace_like: false },
                    PUB => NodeTypeInfo { name: "PUB", whitespace_like: false },
                    STRUCT => NodeTypeInfo { name: "STRUCT", whitespace_like: false },
                    USE => NodeTypeInfo { name: "USE", whitespace_like: false },
                    MOD => NodeTypeInfo { name: "MOD", whitespace_like: false },
                    IF => NodeTypeInfo { name: "IF", whitespace_like: false },
                    ELSE => NodeTypeInfo { name: "ELSE", whitespace_like: false },
                    ENUM => NodeTypeInfo { name: "ENUM", whitespace_like: false },
                    IMPL => NodeTypeInfo { name: "IMPL", whitespace_like: false },
                    SELF => NodeTypeInfo { name: "SELF", whitespace_like: false },
                    SUPER => NodeTypeInfo { name: "SUPER", whitespace_like: false },
                    TYPE => NodeTypeInfo { name: "TYPE", whitespace_like: false },
                    CONST => NodeTypeInfo { name: "CONST", whitespace_like: false },
                    FOR => NodeTypeInfo { name: "FOR", whitespace_like: false },
                    L_PAREN => NodeTypeInfo { name: "L_PAREN", whitespace_like: false },
                    R_PAREN => NodeTypeInfo { name: "R_PAREN", whitespace_like: false },
                    L_CURLY => NodeTypeInfo { name: "L_CURLY", whitespace_like: false },
                    R_CURLY => NodeTypeInfo { name: "R_CURLY", whitespace_like: false },
                    L_ANGLE => NodeTypeInfo { name: "L_ANGLE", whitespace_like: false },
                    R_ANGLE => NodeTypeInfo { name: "R_ANGLE", whitespace_like: false },
                    L_BRACK => NodeTypeInfo { name: "L_BRACK", whitespace_like: false },
                    R_BRACK => NodeTypeInfo { name: "R_BRACK", whitespace_like: false },
                    SHL => NodeTypeInfo { name: "SHL", whitespace_like: false },
                    SHR => NodeTypeInfo { name: "SHR", whitespace_like: false },
                    THIN_ARROW => NodeTypeInfo { name: "THIN_ARROW", whitespace_like: false },
                    EQ => NodeTypeInfo { name: "EQ", whitespace_like: false },
                    SEMI => NodeTypeInfo { name: "SEMI", whitespace_like: false },
                    COLON => NodeTypeInfo { name: "COLON", whitespace_like: false },
                    COLONCOLON => NodeTypeInfo { name: "COLONCOLON", whitespace_like: false },
                    COMMA => NodeTypeInfo { name: "COMMA", whitespace_like: false },
                    DOT => NodeTypeInfo { name: "DOT", whitespace_like: false },
                    HASH => NodeTypeInfo { name: "HASH", whitespace_like: false },
                    STAR => NodeTypeInfo { name: "STAR", whitespace_like: false },
                    SLASH => NodeTypeInfo { name: "SLASH", whitespace_like: false },
                    PERCENT => NodeTypeInfo { name: "PERCENT", whitespace_like: false },
                    PLUS => NodeTypeInfo { name: "PLUS", whitespace_like: false },
                    MINUS => NodeTypeInfo { name: "MINUS", whitespace_like: false },
                    AMPERSAND => NodeTypeInfo { name: "AMPERSAND", whitespace_like: false },
                    PIPE => NodeTypeInfo { name: "PIPE", whitespace_like: false },
                    UNDERSCORE => NodeTypeInfo { name: "UNDERSCORE", whitespace_like: false },
                    LIFETIME => NodeTypeInfo { name: "LIFETIME", whitespace_like: false },
                    IDENT => NodeTypeInfo { name: "IDENT", whitespace_like: false },
                    NUMBER => NodeTypeInfo { name: "NUMBER", whitespace_like: false },
                    STRING => NodeTypeInfo { name: "STRING", whitespace_like: false },
                    FILE => NodeTypeInfo { name: "FILE", whitespace_like: false },
                    USE_DECL => NodeTypeInfo { name: "USE_DECL", whitespace_like: false },
                    USE_SPEC => NodeTypeInfo { name: "USE_SPEC", whitespace_like: false },
                    USE_SPEC_ENTRY => NodeTypeInfo { name: "USE_SPEC_ENTRY", whitespace_like: false },
                    EXTERN_CRATE_DECL => NodeTypeInfo { name: "EXTERN_CRATE_DECL", whitespace_like: false },
                    FN_DEF => NodeTypeInfo { name: "FN_DEF", whitespace_like: false },
                    VALUE_PARAM => NodeTypeInfo { name: "VALUE_PARAM", whitespace_like: false },
                    LAMBDA_VALUE_PARAM => NodeTypeInfo { name: "LAMBDA_VALUE_PARAM", whitespace_like: false },
                    SELF_PARAMETER => NodeTypeInfo { name: "SELF_PARAMETER", whitespace_like: false },
                    STRUCT_DEF => NodeTypeInfo { name: "STRUCT_DEF", whitespace_like: false },
                    STRUCT_FIELD => NodeTypeInfo { name: "STRUCT_FIELD", whitespace_like: false },
                    TUPLE_FIELD => NodeTypeInfo { name: "TUPLE_FIELD", whitespace_like: false },
                    ENUM_DEF => NodeTypeInfo { name: "ENUM_DEF", whitespace_like: false },
                    ENUM_VARIANT => NodeTypeInfo { name: "ENUM_VARIANT", whitespace_like: false },
                    MOD_DEF => NodeTypeInfo { name: "MOD_DEF", whitespace_like: false },
                    IMPL_DEF => NodeTypeInfo { name: "IMPL_DEF", whitespace_like: false },
                    TYPE_DEF => NodeTypeInfo { name: "TYPE_DEF", whitespace_like: false },
                    CONST_DEF => NodeTypeInfo { name: "CONST_DEF", whitespace_like: false },
                    TYPE_PARAMETERS => NodeTypeInfo { name: "TYPE_PARAMETERS", whitespace_like: false },
                    TYPE_PARAMETER => NodeTypeInfo { name: "TYPE_PARAMETER", whitespace_like: false },
                    LIFETIME_PARAMETER => NodeTypeInfo { name: "LIFETIME_PARAMETER", whitespace_like: false },
                    VISIBILITY => NodeTypeInfo { name: "VISIBILITY", whitespace_like: false },
                    PATH => NodeTypeInfo { name: "PATH", whitespace_like: false },
                    TYPE_ARGUMENTS => NodeTypeInfo { name: "TYPE_ARGUMENTS", whitespace_like: false },
                    ALIAS => NodeTypeInfo { name: "ALIAS", whitespace_like: false },
                    TYPE_REFERENCE => NodeTypeInfo { name: "TYPE_REFERENCE", whitespace_like: false },
                    PATTERN => NodeTypeInfo { name: "PATTERN", whitespace_like: false },
                    EXPR => NodeTypeInfo { name: "EXPR", whitespace_like: false },
                    LITERAL => NodeTypeInfo { name: "LITERAL", whitespace_like: false },
                    STRUCT_LITERAL => NodeTypeInfo { name: "STRUCT_LITERAL", whitespace_like: false },
                    STRUCT_LITERAL_FIELD => NodeTypeInfo { name: "STRUCT_LITERAL_FIELD", whitespace_like: false },
                    PATH_EXPR => NodeTypeInfo { name: "PATH_EXPR", whitespace_like: false },
                    PAREN_EXPR => NodeTypeInfo { name: "PAREN_EXPR", whitespace_like: false },
                    LAMBDA_EXPR => NodeTypeInfo { name: "LAMBDA_EXPR", whitespace_like: false },
                    BLOCK_EXPR => NodeTypeInfo { name: "BLOCK_EXPR", whitespace_like: false },
                    LET_STMT => NodeTypeInfo { name: "LET_STMT", whitespace_like: false },
                    EMPTY_STMT => NodeTypeInfo { name: "EMPTY_STMT", whitespace_like: false },
                    EXPR_STMT => NodeTypeInfo { name: "EXPR_STMT", whitespace_like: false },
                    IF_EXPR => NodeTypeInfo { name: "IF_EXPR", whitespace_like: false },
                    CALL_EXPR => NodeTypeInfo { name: "CALL_EXPR", whitespace_like: false },
                    FIELD_EXPR => NodeTypeInfo { name: "FIELD_EXPR", whitespace_like: false },
                    VALUE_ARGUMENT => NodeTypeInfo { name: "VALUE_ARGUMENT", whitespace_like: false },
                    REFERENCE_EXPR => NodeTypeInfo { name: "REFERENCE_EXPR", whitespace_like: false },
                    DEREFERENCE_EXPR => NodeTypeInfo { name: "DEREFERENCE_EXPR", whitespace_like: false },
                    NEGATION_EXPR => NodeTypeInfo { name: "NEGATION_EXPR", whitespace_like: false },
                    PRODUCT_EXPR => NodeTypeInfo { name: "PRODUCT_EXPR", whitespace_like: false },
                    SUM_EXPR => NodeTypeInfo { name: "SUM_EXPR", whitespace_like: false },
                    BIT_SHIFT => NodeTypeInfo { name: "BIT_SHIFT", whitespace_like: false },
                    BIT_AND => NodeTypeInfo { name: "BIT_AND", whitespace_like: false },
                    BIT_OR => NodeTypeInfo { name: "BIT_OR", whitespace_like: false },
                    ATTRIBUTE => NodeTypeInfo { name: "ATTRIBUTE", whitespace_like: false },
                    ATTR_VALUE => NodeTypeInfo { name: "ATTR_VALUE", whitespace_like: false },
                    _ => panic!("Unknown NodeType: {:?}", ty)
                }
            }
        }

        Language::new(Impl {
            tokenizer: vec![
                LexRule::new(WHITESPACE, "\\s+", None),
                LexRule::new(AS, "as", None),
                LexRule::new(CRATE, "crate", None),
                LexRule::new(EXTERN, "extern", None),
                LexRule::new(FN, "fn", None),
                LexRule::new(LET, "let", None),
                LexRule::new(PUB, "pub", None),
                LexRule::new(STRUCT, "struct", None),
                LexRule::new(USE, "use", None),
                LexRule::new(MOD, "mod", None),
                LexRule::new(IF, "if", None),
                LexRule::new(ELSE, "else", None),
                LexRule::new(ENUM, "enum", None),
                LexRule::new(IMPL, "impl", None),
                LexRule::new(SELF, "self", None),
                LexRule::new(SUPER, "super", None),
                LexRule::new(TYPE, "type", None),
                LexRule::new(CONST, "const", None),
                LexRule::new(FOR, "for", None),
                LexRule::new(L_PAREN, "\\(", None),
                LexRule::new(R_PAREN, "\\)", None),
                LexRule::new(L_CURLY, "\\{", None),
                LexRule::new(R_CURLY, "\\}", None),
                LexRule::new(L_ANGLE, "<", None),
                LexRule::new(R_ANGLE, ">", None),
                LexRule::new(L_BRACK, "\\[", None),
                LexRule::new(R_BRACK, "\\]", None),
                LexRule::new(THIN_ARROW, "\\->", None),
                LexRule::new(EQ, "=", None),
                LexRule::new(SEMI, ";", None),
                LexRule::new(COLON, ":", None),
                LexRule::new(COLONCOLON, "::", None),
                LexRule::new(COMMA, ",", None),
                LexRule::new(DOT, "\\.", None),
                LexRule::new(HASH, "\\#", None),
                LexRule::new(STAR, "\\*", None),
                LexRule::new(SLASH, "/", None),
                LexRule::new(PERCENT, "%", None),
                LexRule::new(PLUS, "\\+", None),
                LexRule::new(MINUS, "\\-", None),
                LexRule::new(AMPERSAND, "\\&", None),
                LexRule::new(PIPE, "\\|", None),
                LexRule::new(UNDERSCORE, "_", None),
                LexRule::new(LIFETIME, "\'\\p{XID_Continue}*", None),
                LexRule::new(IDENT, "\\p{XID_Start}\\w*", None),
                LexRule::new(NUMBER, "\\d+", None),
                LexRule::new(STRING, "\"([^\"]|\\\\\")*\"", None),
            ],
            parser: parser,
        })
    };
}


