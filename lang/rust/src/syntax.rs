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
pub const STATIC: NodeType = NodeType(119);
pub const FOR: NodeType = NodeType(120);
pub const LOOP: NodeType = NodeType(121);
pub const WHILE: NodeType = NodeType(122);
pub const MOVE: NodeType = NodeType(123);
pub const MUT: NodeType = NodeType(124);
pub const REF: NodeType = NodeType(125);
pub const TRAIT: NodeType = NodeType(126);
pub const L_PAREN: NodeType = NodeType(127);
pub const R_PAREN: NodeType = NodeType(128);
pub const L_CURLY: NodeType = NodeType(129);
pub const R_CURLY: NodeType = NodeType(130);
pub const L_ANGLE: NodeType = NodeType(131);
pub const R_ANGLE: NodeType = NodeType(132);
pub const L_BRACK: NodeType = NodeType(133);
pub const R_BRACK: NodeType = NodeType(134);
pub const SHL: NodeType = NodeType(135);
pub const SHR: NodeType = NodeType(136);
pub const AND: NodeType = NodeType(137);
pub const OR: NodeType = NodeType(138);
pub const THIN_ARROW: NodeType = NodeType(139);
pub const EQ: NodeType = NodeType(140);
pub const SEMI: NodeType = NodeType(141);
pub const COLON: NodeType = NodeType(142);
pub const COLONCOLON: NodeType = NodeType(143);
pub const COMMA: NodeType = NodeType(144);
pub const DOT: NodeType = NodeType(145);
pub const HASH: NodeType = NodeType(146);
pub const STAR: NodeType = NodeType(147);
pub const SLASH: NodeType = NodeType(148);
pub const PERCENT: NodeType = NodeType(149);
pub const PLUS: NodeType = NodeType(150);
pub const MINUS: NodeType = NodeType(151);
pub const AMPERSAND: NodeType = NodeType(152);
pub const PIPE: NodeType = NodeType(153);
pub const UNDERSCORE: NodeType = NodeType(154);
pub const LIFETIME: NodeType = NodeType(155);
pub const IDENT: NodeType = NodeType(156);
pub const NUMBER: NodeType = NodeType(157);
pub const STRING: NodeType = NodeType(158);
pub const FILE: NodeType = NodeType(159);
pub const USE_DECL: NodeType = NodeType(160);
pub const USE_SPEC: NodeType = NodeType(161);
pub const USE_SPEC_ENTRY: NodeType = NodeType(162);
pub const EXTERN_CRATE_DECL: NodeType = NodeType(163);
pub const FN_DEF: NodeType = NodeType(164);
pub const LINKAGE: NodeType = NodeType(165);
pub const VALUE_PARAM: NodeType = NodeType(166);
pub const LAMBDA_VALUE_PARAM: NodeType = NodeType(167);
pub const SELF_PARAMETER: NodeType = NodeType(168);
pub const STRUCT_DEF: NodeType = NodeType(169);
pub const STRUCT_FIELD: NodeType = NodeType(170);
pub const TUPLE_FIELD: NodeType = NodeType(171);
pub const ENUM_DEF: NodeType = NodeType(172);
pub const ENUM_VARIANT: NodeType = NodeType(173);
pub const MOD_DEF: NodeType = NodeType(174);
pub const IMPL_DEF: NodeType = NodeType(175);
pub const TRAIT_DEF: NodeType = NodeType(176);
pub const MEMBERS: NodeType = NodeType(177);
pub const TYPE_DEF: NodeType = NodeType(178);
pub const CONST_DEF: NodeType = NodeType(179);
pub const EXTERN_BLOCK: NodeType = NodeType(180);
pub const TYPE_PARAMETERS: NodeType = NodeType(181);
pub const TYPE_PARAMETER: NodeType = NodeType(182);
pub const LIFETIME_PARAMETER: NodeType = NodeType(183);
pub const VISIBILITY: NodeType = NodeType(184);
pub const PATH: NodeType = NodeType(185);
pub const TYPE_ARGUMENTS: NodeType = NodeType(186);
pub const ALIAS: NodeType = NodeType(187);
pub const PATH_TYPE: NodeType = NodeType(188);
pub const REFERENCE_TYPE: NodeType = NodeType(189);
pub const PLACEHOLDER_TYPE: NodeType = NodeType(190);
pub const UNIT_TYPE: NodeType = NodeType(191);
pub const PAREN_TYPE: NodeType = NodeType(192);
pub const TUPLE_TYPE: NodeType = NodeType(193);
pub const ARRAY_TYPE: NodeType = NodeType(194);
pub const WILDCARD_PATTERN: NodeType = NodeType(195);
pub const TUPE_STRUCT_PATTERN: NodeType = NodeType(196);
pub const BINDING_PATTERN: NodeType = NodeType(197);
pub const PATH_PATTERN: NodeType = NodeType(198);
pub const EXPR: NodeType = NodeType(199);
pub const LITERAL: NodeType = NodeType(200);
pub const STRUCT_LITERAL: NodeType = NodeType(201);
pub const STRUCT_LITERAL_FIELD: NodeType = NodeType(202);
pub const PATH_EXPR: NodeType = NodeType(203);
pub const UNIT_EXPR: NodeType = NodeType(204);
pub const PAREN_EXPR: NodeType = NodeType(205);
pub const TUPLE_EXPR: NodeType = NodeType(206);
pub const LAMBDA_EXPR: NodeType = NodeType(207);
pub const BLOCK_EXPR: NodeType = NodeType(208);
pub const LET_STMT: NodeType = NodeType(209);
pub const EMPTY_STMT: NodeType = NodeType(210);
pub const EXPR_STMT: NodeType = NodeType(211);
pub const IF_EXPR: NodeType = NodeType(212);
pub const WHILE_EXPR: NodeType = NodeType(213);
pub const LOOP_EXPR: NodeType = NodeType(214);
pub const METHOD_CALL_EXPR: NodeType = NodeType(215);
pub const CALL_EXPR: NodeType = NodeType(216);
pub const FIELD_EXPR: NodeType = NodeType(217);
pub const INDEX_EXPR: NodeType = NodeType(218);
pub const VALUE_ARGUMENT: NodeType = NodeType(219);
pub const REFERENCE_EXPR: NodeType = NodeType(220);
pub const DEREFERENCE_EXPR: NodeType = NodeType(221);
pub const NEGATION_EXPR: NodeType = NodeType(222);
pub const PRODUCT_EXPR: NodeType = NodeType(223);
pub const SUM_EXPR: NodeType = NodeType(224);
pub const BIT_SHIFT: NodeType = NodeType(225);
pub const BIT_AND: NodeType = NodeType(226);
pub const BIT_OR: NodeType = NodeType(227);
pub const LOGICAL_AND: NodeType = NodeType(228);
pub const LOGICAL_OR: NodeType = NodeType(229);
pub const ATTRIBUTE: NodeType = NodeType(230);
pub const ATTR_VALUE: NodeType = NodeType(231);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR,
            WHITESPACE, UNION, AS, CRATE, EXTERN, FN, LET, PUB, STRUCT, USE, MOD, IF, ELSE, ENUM, IMPL, SELF, SUPER, TYPE, CONST, STATIC, FOR, LOOP, WHILE, MOVE, MUT, REF, TRAIT, L_PAREN, R_PAREN, L_CURLY, R_CURLY, L_ANGLE, R_ANGLE, L_BRACK, R_BRACK, SHL, SHR, AND, OR, THIN_ARROW, EQ, SEMI, COLON, COLONCOLON, COMMA, DOT, HASH, STAR, SLASH, PERCENT, PLUS, MINUS, AMPERSAND, PIPE, UNDERSCORE, LIFETIME, IDENT, NUMBER, STRING, FILE, USE_DECL, USE_SPEC, USE_SPEC_ENTRY, EXTERN_CRATE_DECL, FN_DEF, LINKAGE, VALUE_PARAM, LAMBDA_VALUE_PARAM, SELF_PARAMETER, STRUCT_DEF, STRUCT_FIELD, TUPLE_FIELD, ENUM_DEF, ENUM_VARIANT, MOD_DEF, IMPL_DEF, TRAIT_DEF, MEMBERS, TYPE_DEF, CONST_DEF, EXTERN_BLOCK, TYPE_PARAMETERS, TYPE_PARAMETER, LIFETIME_PARAMETER, VISIBILITY, PATH, TYPE_ARGUMENTS, ALIAS, PATH_TYPE, REFERENCE_TYPE, PLACEHOLDER_TYPE, UNIT_TYPE, PAREN_TYPE, TUPLE_TYPE, ARRAY_TYPE, WILDCARD_PATTERN, TUPE_STRUCT_PATTERN, BINDING_PATTERN, PATH_PATTERN, EXPR, LITERAL, STRUCT_LITERAL, STRUCT_LITERAL_FIELD, PATH_EXPR, UNIT_EXPR, PAREN_EXPR, TUPLE_EXPR, LAMBDA_EXPR, BLOCK_EXPR, LET_STMT, EMPTY_STMT, EXPR_STMT, IF_EXPR, WHILE_EXPR, LOOP_EXPR, METHOD_CALL_EXPR, CALL_EXPR, FIELD_EXPR, INDEX_EXPR, VALUE_ARGUMENT, REFERENCE_EXPR, DEREFERENCE_EXPR, NEGATION_EXPR, PRODUCT_EXPR, SUM_EXPR, BIT_SHIFT, BIT_AND, BIT_OR, LOGICAL_AND, LOGICAL_OR, ATTRIBUTE, ATTR_VALUE,
        ];
        let parser_json = r##"[{"body":{"Pub":{"ty_idx":60,"body":{"Or":[{"And":[[{"Rule":1}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rep":{"WithSkip":[{"Rule":2},{"Rule":3}]}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":8}],null]},{"And":[[{"Token":5}],null]},{"And":[[{"Token":47}],null]},{"And":[[{"Token":6}],null]},{"And":[[{"Token":9}],null]},{"And":[[{"ContextualToken":[2,"union"]}],null]},{"And":[[{"Token":14}],null]},{"And":[[{"Token":10}],null]},{"And":[[{"Token":11}],null]},{"And":[[{"Token":15}],null]},{"And":[[{"Token":27}],null]},{"And":[[{"Token":18}],null]},{"And":[[{"Token":19}],null]}]}},{"body":{"Or":[{"And":[[{"Rule":4}],null]},{"And":[[{"Rule":7}],null]},{"And":[[{"Rule":8}],null]},{"And":[[{"Rule":14}],null]},{"And":[[{"Rule":17}],null]},{"And":[[{"Rule":19}],null]},{"And":[[{"Rule":20}],null]},{"And":[[{"Rule":21}],null]},{"And":[[{"Rule":25}],null]},{"And":[[{"Rule":26}],null]},{"And":[[{"Rule":27}],null]}]}},{"body":{"Pub":{"ty_idx":61,"body":{"Or":[{"And":[[{"Rule":94},{"Opt":{"Rule":32}},{"Token":10},{"Or":[{"And":[[{"Rule":33},{"Or":[{"And":[[{"Rule":41}],null]},{"And":[[{"Opt":{"Or":[{"And":[[{"Token":44},{"Rule":5}],null]}]}}],null]}]}],null]},{"And":[[{"Opt":{"Token":44}},{"Rule":5}],null]}]},{"Token":42}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":62,"body":{"Or":[{"And":[[{"Token":48}],null]},{"And":[[{"Call":[{"Rule":97},[[3,{"Call":[{"Rule":96},[[2,{"Rule":6}]]]}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":63,"body":{"Or":[{"And":[[{"Token":57},{"Opt":{"Rule":41}}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":64,"body":{"Or":[{"And":[[{"Rule":94},{"Token":5},{"Token":4},{"Token":57},{"Opt":{"Rule":41}},{"Token":42}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":65,"body":{"Or":[{"And":[[{"Rule":94},{"Opt":{"Rule":32}},{"Opt":{"Rule":9}},{"Token":6},{"Token":57},{"Opt":{"Rule":29}},{"Rule":10},{"Opt":{"Or":[{"And":[[{"Token":40},{"Rule":42}],null]}]}},{"Or":[{"And":[[{"Rule":67}],null]},{"And":[[{"Token":42}],null]}]}],4]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":66,"body":{"Or":[{"And":[[{"Token":5},{"Opt":{"Token":59}}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":98},[[4,{"Or":[{"And":[[{"Opt":{"Rule":13}},{"Call":[{"Rule":96},[[2,{"Rule":11}]]]}],null]}]}]]]}],null]}]}},{"body":{"Pub":{"ty_idx":67,"body":{"Or":[{"And":[[{"Rule":50},{"Token":43},{"Rule":42}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":68,"body":{"Or":[{"And":[[{"Rule":50},{"Opt":{"Or":[{"And":[[{"Token":43},{"Rule":42}],null]}]}}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":69,"body":{"Or":[{"And":[[{"Opt":{"Token":53}},{"Token":16},{"Or":[{"And":[[{"Token":45}],null]},{"And":[["Eof"],null]}]}],2]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":70,"body":{"Or":[{"And":[[{"Rule":94},{"Opt":{"Rule":32}},{"Or":[{"And":[[{"Token":9}],null]},{"And":[[{"ContextualToken":[2,"union"]}],null]}]},{"Token":57},{"Opt":{"Rule":29}},{"Or":[{"And":[[{"Call":[{"Rule":97},[[3,{"Call":[{"Rule":96},[[2,{"Rule":15}]]]}]]]}],null]},{"And":[[{"Token":42}],null]},{"And":[[{"Call":[{"Rule":98},[[4,{"Call":[{"Rule":96},[[2,{"Rule":16}]]]}]]]},{"Token":42}],null]}]}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":71,"body":{"Or":[{"And":[[{"Opt":{"Rule":32}},{"Token":57},{"Token":43},{"Rule":42}],2]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":72,"body":{"Or":[{"And":[[{"Opt":{"Rule":32}},{"Rule":42}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":73,"body":{"Or":[{"And":[[{"Rule":94},{"Opt":{"Rule":32}},{"Token":14},{"Token":57},{"Call":[{"Rule":97},[[3,{"Call":[{"Rule":96},[[2,{"Rule":18}]]]}]]]}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":74,"body":{"Or":[{"And":[[{"Token":57},{"Opt":{"Or":[{"And":[[{"Token":41},{"Rule":55}],null]},{"And":[[{"Call":[{"Rule":98},[[4,{"Call":[{"Rule":96},[[2,{"Rule":16}]]]}]]]}],null]},{"And":[[{"Call":[{"Rule":97},[[3,{"Call":[{"Rule":96},[[2,{"Rule":15}]]]}]]]}],null]}]}}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":75,"body":{"Or":[{"And":[[{"Rule":94},{"Opt":{"Rule":32}},{"Token":11},{"Token":57},{"Or":[{"And":[[{"Token":42}],null]},{"And":[[{"Call":[{"Rule":97},[[3,{"Rule":1}]]]}],null]}]}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":76,"body":{"Or":[{"And":[[{"Rule":94},{"Token":15},{"Opt":{"Rule":29}},{"Or":[{"And":[[{"Rule":42},{"Opt":{"Or":[{"And":[[{"Token":21},{"Rule":42}],null]}]}}],null]}]},{"Rule":22}],2]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":77,"body":{"Or":[{"And":[[{"Rule":94},{"Opt":{"Rule":32}},{"Token":27},{"Token":57},{"Opt":{"Rule":29}},{"Rule":22}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":78,"body":{"Or":[{"And":[[{"Call":[{"Rule":97},[[3,{"Rep":{"WithSkip":[{"Rule":24},{"Rule":23}]}}]]]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":8}],null]},{"And":[[{"Rule":25}],null]}]}},{"body":{"Or":[{"And":[[{"Token":8}],null]},{"And":[[{"Token":6}],null]},{"And":[[{"Token":18}],null]}]}},{"body":{"Pub":{"ty_idx":79,"body":{"Or":[{"And":[[{"Rule":94},{"Opt":{"Rule":32}},{"Token":18},{"Token":57},{"Opt":{"Rule":29}},{"Opt":{"Or":[{"And":[[{"Token":41},{"Rule":42}],null]}]}},{"Token":42}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":80,"body":{"Or":[{"And":[[{"Rule":94},{"Opt":{"Rule":32}},{"Or":[{"And":[[{"Token":19}],null]},{"And":[[{"Token":20}],null]}]},{"Token":57},{"Token":43},{"Rule":42},{"Opt":{"Or":[{"And":[[{"Token":41},{"Rule":55}],null]}]}},{"Token":42}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":81,"body":{"Or":[{"And":[[{"Rule":9},{"Call":[{"Rule":97},[[3,{"Rep":{"Rule":28}}]]]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":8}],null]},{"And":[[{"Rule":26}],null]}]}},{"body":{"Pub":{"ty_idx":82,"body":{"Or":[{"And":[[{"Call":[{"Rule":99},[[5,{"Or":[{"And":[[{"Call":[{"Rule":96},[[2,{"Rule":31}]]]},{"Call":[{"Rule":96},[[2,{"Rule":30}]]]}],null]}]}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":83,"body":{"Or":[{"And":[[{"Token":57}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":84,"body":{"Or":[{"And":[[{"Token":56}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":85,"body":{"Or":[{"And":[[{"Token":8}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":36}],null]}]}},{"body":{"Or":[{"And":[[{"Enter":[0,{"Rule":36}]}],null]}]}},{"body":{"Or":[{"And":[[{"Enter":[1,{"Rule":36}]}],null]}]}},{"body":{"Pratt":[{"Atom":{"body":{"Pub":{"ty_idx":86,"body":{"Or":[{"And":[[{"Opt":{"Token":44}},{"Rule":39}],null]}]},"replaceable":false}}}},{"Postfix":{"ty":86,"op":{"Or":[{"And":[[{"Token":44},{"Rule":39}],null]}]}}}]}},{"body":{"Pub":{"ty_idx":86,"body":{"Or":[{"And":[[{"Opt":{"Token":44}},{"Rule":39}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":86,"body":{"Or":[{"And":[[{"Rule":36},{"Or":[{"And":[[{"Token":44},{"Rule":39}],null]}]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Or":[{"And":[[{"Token":57}],null]},{"And":[[{"Token":16}],null]},{"And":[[{"Token":17}],null]}]},{"Opt":{"Rule":40}}],null]}]}},{"body":{"Pub":{"ty_idx":87,"body":{"Or":[{"And":[[{"Or":[{"And":[[{"IsIn":0}],null]},{"And":[[{"IsIn":1},{"Token":44}],null]}]},{"Call":[{"Rule":99},[[5,{"Or":[{"And":[[{"Call":[{"Rule":96},[[2,{"Token":56}]]]},{"Call":[{"Rule":96},[[2,{"Rule":42}]]]}],null]}]}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":88,"body":{"Or":[{"And":[[{"Token":3},{"Token":57}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":43}],null]},{"And":[[{"Rule":44}],null]},{"And":[[{"Rule":45}],null]},{"And":[[{"Rule":46}],null]},{"And":[[{"Rule":47}],null]},{"And":[[{"Rule":49}],null]}]}},{"body":{"Pub":{"ty_idx":89,"body":{"Or":[{"And":[[{"Rule":34}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":90,"body":{"Or":[{"And":[[{"Token":53},{"Opt":{"Token":56}},{"Opt":{"Token":25}},{"Rule":42}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":91,"body":{"Or":[{"And":[[{"Token":55}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":92,"body":{"Or":[{"And":[[{"Token":28},{"Token":29}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":93,"body":{"Or":[{"And":[[{"Call":[{"Rule":98},[[4,{"Or":[{"And":[[{"Rule":42},{"Opt":{"Rule":48}}],null]}]}]]]}],null]}]},"replaceable":true}}},{"body":{"PubReplace":{"ty_idx":94,"body":{"Or":[{"And":[[{"Token":45},{"Call":[{"Rule":96},[[2,{"Rule":42}]]]}],null]}]}}}},{"body":{"Pub":{"ty_idx":95,"body":{"Or":[{"And":[[{"Call":[{"Rule":100},[[6,{"Or":[{"And":[[{"Rule":42},{"Opt":{"Or":[{"And":[[{"Token":42},{"Rule":55}],null]}]}}],null]}]}]]]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":51}],null]},{"And":[[{"Rule":52}],null]},{"And":[[{"Rule":54}],null]},{"And":[[{"Rule":53}],null]}]}},{"body":{"Pub":{"ty_idx":96,"body":{"Or":[{"And":[[{"Token":55}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":97,"body":{"Or":[{"And":[[{"Rule":35},{"Call":[{"Rule":98},[[4,{"Call":[{"Rule":96},[[2,{"Rule":50}]]]}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":98,"body":{"Or":[{"And":[[{"Opt":{"Token":26}},{"Opt":{"Token":25}},{"Token":57}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":99,"body":{"Or":[{"And":[[{"Rule":35}],null]}]},"replaceable":false}}},{"body":{"Pratt":[{"Atom":{"body":{"Pub":{"ty_idx":101,"body":{"Or":[{"And":[[{"Token":58}],null]},{"And":[[{"Token":59}],null]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":102,"body":{"Or":[{"And":[[{"Not":{"IsIn":5}},{"Rule":35},{"Call":[{"Rule":97},[[3,{"Call":[{"Rule":96},[[2,{"Rule":60}]]]}]]]}],null]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":104,"body":{"Or":[{"And":[[{"Rule":35}],null]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":105,"body":{"Or":[{"And":[[{"Token":28},{"Token":29}],null]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":106,"body":{"Or":[{"And":[[{"Token":28},{"Call":[{"Rule":57},[[0,{"Rule":55}]]]},{"Opt":{"Rule":64}},{"Token":29}],null]}]},"replaceable":true}}}},{"Atom":{"body":{"Pub":{"ty_idx":108,"body":{"Or":[{"And":[[{"Opt":{"Token":24}},{"Token":54},{"Rep":{"Rule":66}},{"Token":54},{"Or":[{"And":[[{"Token":40},{"Rule":42},{"Rule":67}],null]},{"And":[[{"Call":[{"Rule":57},[[0,{"Rule":55}]]]}],null]}]}],null]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":109,"body":{"Or":[{"And":[[{"Call":[{"Rule":57},[[0,{"Call":[{"Rule":97},[[3,{"Or":[{"And":[[{"Rep":{"Rule":68}},{"Opt":{"Rule":55}}],null]}]}]]]}]]]}],null]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":113,"body":{"Or":[{"And":[[{"Token":12},{"Rule":74},{"Rule":67},{"Opt":{"Or":[{"And":[[{"Token":13},{"Rule":67}],null]}]}}],1]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":114,"body":{"Or":[{"And":[[{"Opt":{"Or":[{"And":[[{"Token":56},{"Token":43}],null]}]}},{"Token":23},{"Rule":74},{"Rule":67}],2]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":115,"body":{"Or":[{"And":[[{"Opt":{"Or":[{"And":[[{"Token":56},{"Token":43}],null]}]}},{"Token":22},{"Rule":67}],2]}]},"replaceable":false}}}},{"Postfix":{"ty":116,"op":{"Or":[{"And":[[{"Token":46},{"Token":57},{"Opt":{"Enter":[1,{"Rule":40}]}},{"Rule":78}],null]}]}}},{"Postfix":{"ty":117,"op":{"Rule":78}}},{"Postfix":{"ty":118,"op":{"Or":[{"And":[[{"Token":46},{"Or":[{"And":[[{"Token":57}],null]},{"And":[[{"Token":58}],null]}]}],null]}]}}},{"Postfix":{"ty":119,"op":{"Call":[{"Rule":100},[[6,{"Rule":55}]]]}}},{"Prefix":{"ty":121,"op":{"Token":53}}},{"Prefix":{"ty":122,"op":{"Token":48}}},{"Prefix":{"ty":123,"op":{"Token":52}}},{"Binary":{"ty":124,"op":{"Call":[{"Rule":88},[[1,{"Or":[{"And":[[{"Token":48}],null]},{"And":[[{"Token":49}],null]},{"And":[[{"Token":50}],null]}]}]]]},"priority":7}},{"Binary":{"ty":125,"op":{"Call":[{"Rule":88},[[1,{"Or":[{"And":[[{"Token":51}],null]},{"And":[[{"Token":52}],null]}]}]]]},"priority":6}},{"Binary":{"ty":126,"op":{"Call":[{"Rule":88},[[1,{"Or":[{"And":[[{"ContextualToken":[36,"<<"]}],null]},{"And":[[{"ContextualToken":[37,">>"]}],null]}]}]]]},"priority":5}},{"Binary":{"ty":127,"op":{"Call":[{"Rule":88},[[1,{"Or":[{"And":[[{"Token":53},{"Not":{"Token":53}}],null]}]}]]]},"priority":4}},{"Binary":{"ty":128,"op":{"Call":[{"Rule":88},[[1,{"Or":[{"And":[[{"Token":54},{"Not":{"Token":54}}],null]}]}]]]},"priority":3}},{"Binary":{"ty":129,"op":{"Call":[{"Rule":88},[[1,{"ContextualToken":[38,"&&"]}]]]},"priority":2}},{"Binary":{"ty":130,"op":{"Call":[{"Rule":88},[[1,{"ContextualToken":[39,"||"]}]]]},"priority":1}}]}},{"body":{"Or":[{"And":[[{"PrevIs":[113,109]}],null]}]}},{"body":{"Or":[{"And":[[{"Exit":[5,{"Exit":[4,{"Var":0}]}]}],null]}]}},{"body":{"Pub":{"ty_idx":101,"body":{"Or":[{"And":[[{"Token":58}],null]},{"And":[[{"Token":59}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":102,"body":{"Or":[{"And":[[{"Not":{"IsIn":5}},{"Rule":35},{"Call":[{"Rule":97},[[3,{"Call":[{"Rule":96},[[2,{"Rule":60}]]]}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":103,"body":{"Or":[{"And":[[{"Token":57},{"Token":43},{"Rule":55}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":104,"body":{"Or":[{"And":[[{"Rule":35}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":105,"body":{"Or":[{"And":[[{"Token":28},{"Token":29}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":106,"body":{"Or":[{"And":[[{"Token":28},{"Call":[{"Rule":57},[[0,{"Rule":55}]]]},{"Opt":{"Rule":64}},{"Token":29}],null]}]},"replaceable":true}}},{"body":{"PubReplace":{"ty_idx":107,"body":{"Or":[{"And":[[{"Token":45},{"Call":[{"Rule":96},[[2,{"Call":[{"Rule":57},[[0,{"Rule":55}]]]}]]]}],null]}]}}}},{"body":{"Pub":{"ty_idx":108,"body":{"Or":[{"And":[[{"Opt":{"Token":24}},{"Token":54},{"Rep":{"Rule":66}},{"Token":54},{"Or":[{"And":[[{"Token":40},{"Rule":42},{"Rule":67}],null]},{"And":[[{"Call":[{"Rule":57},[[0,{"Rule":55}]]]}],null]}]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":12},{"Or":[{"And":[[{"Token":45}],null]},{"And":[[{"Not":{"Not":{"Token":54}}}],null]}]}],1]}]}},{"body":{"Pub":{"ty_idx":109,"body":{"Or":[{"And":[[{"Call":[{"Rule":57},[[0,{"Call":[{"Rule":97},[[3,{"Or":[{"And":[[{"Rep":{"Rule":68}},{"Opt":{"Rule":55}}],null]}]}]]]}]]]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":69}],null]},{"And":[[{"Rule":71}],null]},{"And":[[{"Rule":70}],null]}]}},{"body":{"Pub":{"ty_idx":110,"body":{"Or":[{"And":[[{"Token":7},{"Rule":50},{"Token":41},{"Rule":55},{"Token":42}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":111,"body":{"Or":[{"And":[[{"Token":42}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":112,"body":{"Or":[{"And":[[{"Enter":[4,{"Or":[{"And":[[{"Rule":55},{"Or":[{"And":[[{"Rule":56},{"Not":"Eof"}],null]},{"And":[[{"Token":42}],null]}]}],null]}]}]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":113,"body":{"Or":[{"And":[[{"Token":12},{"Rule":74},{"Rule":67},{"Opt":{"Or":[{"And":[[{"Token":13},{"Rule":67}],null]}]}}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":114,"body":{"Or":[{"And":[[{"Opt":{"Or":[{"And":[[{"Token":56},{"Token":43}],null]}]}},{"Token":23},{"Rule":74},{"Rule":67}],2]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Enter":[5,{"Or":[{"And":[[{"Opt":{"Or":[{"And":[[{"Token":7},{"Rule":50},{"Token":41}],1]}]}},{"Rule":55}],null]}]}]}],null]}]}},{"body":{"Pub":{"ty_idx":115,"body":{"Or":[{"And":[[{"Opt":{"Or":[{"And":[[{"Token":56},{"Token":43}],null]}]}},{"Token":22},{"Rule":67}],2]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":116,"body":{"Or":[{"And":[[{"Rule":55},{"Or":[{"And":[[{"Token":46},{"Token":57},{"Opt":{"Enter":[1,{"Rule":40}]}},{"Rule":78}],null]}]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":117,"body":{"Or":[{"And":[[{"Rule":55},{"Rule":78}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":57},[[0,{"Call":[{"Rule":98},[[4,{"Call":[{"Rule":96},[[2,{"Rule":81}]]]}]]]}]]]}],null]}]}},{"body":{"Pub":{"ty_idx":118,"body":{"Or":[{"And":[[{"Rule":55},{"Or":[{"And":[[{"Token":46},{"Or":[{"And":[[{"Token":57}],null]},{"And":[[{"Token":58}],null]}]}],null]}]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":119,"body":{"Or":[{"And":[[{"Rule":55},{"Call":[{"Rule":100},[[6,{"Rule":55}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":120,"body":{"Or":[{"And":[[{"Rule":55}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":121,"body":{"Or":[{"And":[[{"Token":53},{"Rule":55}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":122,"body":{"Or":[{"And":[[{"Token":48},{"Rule":55}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":123,"body":{"Or":[{"And":[[{"Token":52},{"Rule":55}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":124,"body":{"Or":[{"And":[[{"Rule":55},{"Call":[{"Rule":88},[[1,{"Or":[{"And":[[{"Token":48}],null]},{"And":[[{"Token":49}],null]},{"And":[[{"Token":50}],null]}]}]]]},{"Rule":55}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":125,"body":{"Or":[{"And":[[{"Rule":55},{"Call":[{"Rule":88},[[1,{"Or":[{"And":[[{"Token":51}],null]},{"And":[[{"Token":52}],null]}]}]]]},{"Rule":55}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":126,"body":{"Or":[{"And":[[{"Rule":55},{"Call":[{"Rule":88},[[1,{"Or":[{"And":[[{"ContextualToken":[36,"<<"]}],null]},{"And":[[{"ContextualToken":[37,">>"]}],null]}]}]]]},{"Rule":55}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"IsIn":4},{"Not":{"Rule":56}},{"Var":1}],null]},{"And":[[{"Not":{"IsIn":4}},{"Var":1}],null]}]}},{"body":{"Pub":{"ty_idx":127,"body":{"Or":[{"And":[[{"Rule":55},{"Call":[{"Rule":88},[[1,{"Or":[{"And":[[{"Token":53},{"Not":{"Token":53}}],null]}]}]]]},{"Rule":55}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":128,"body":{"Or":[{"And":[[{"Rule":55},{"Call":[{"Rule":88},[[1,{"Or":[{"And":[[{"Token":54},{"Not":{"Token":54}}],null]}]}]]]},{"Rule":55}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":129,"body":{"Or":[{"And":[[{"Rule":55},{"Call":[{"Rule":88},[[1,{"ContextualToken":[38,"&&"]}]]]},{"Rule":55}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":130,"body":{"Or":[{"And":[[{"Rule":55},{"Call":[{"Rule":88},[[1,{"ContextualToken":[39,"||"]}]]]},{"Rule":55}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":131,"body":{"Or":[{"And":[[{"Token":47},{"Call":[{"Rule":100},[[6,{"Call":[{"Rule":96},[[2,{"Rule":95}]]]}]]]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rep":{"Rule":93}}],null]}]}},{"body":{"Pub":{"ty_idx":132,"body":{"Or":[{"And":[[{"Token":57},{"Opt":{"Or":[{"And":[[{"Token":41},{"Rule":55}],null]},{"And":[[{"Call":[{"Rule":98},[[4,{"Call":[{"Rule":96},[[2,{"Rule":95}]]]}]]]}],null]}]}}],1]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Var":2},{"Or":[{"And":[["Eof"],null]},{"And":[[{"Token":45}],null]}]}],1]}]}}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":101},[[7,{"Token":30}],[8,{"Token":31}],[9,{"Var":3}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":101},[[7,{"Token":28}],[8,{"Token":29}],[9,{"Var":4}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":101},[[7,{"Token":32}],[8,{"Token":33}],[9,{"Var":5}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":101},[[7,{"Token":34}],[8,{"Token":35}],[9,{"Var":6}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Var":7},{"Layer":[{"Call":[{"Rule":102},[[10,{"Var":7}],[11,{"Var":8}]]]},{"Var":9}]},{"Var":8}],1]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Var":10},{"Call":[{"Rule":102},[[10,{"Var":10}],[11,{"Var":11}]]]},{"Var":11}],1]},{"And":[[{"Or":[{"And":[[{"Not":{"Var":11}},"Any"],null]}]}],null]}]}}],null]}]}}]"##;
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
                    STATIC => NodeTypeInfo { name: "STATIC", whitespace_like: false },
                    FOR => NodeTypeInfo { name: "FOR", whitespace_like: false },
                    LOOP => NodeTypeInfo { name: "LOOP", whitespace_like: false },
                    WHILE => NodeTypeInfo { name: "WHILE", whitespace_like: false },
                    MOVE => NodeTypeInfo { name: "MOVE", whitespace_like: false },
                    MUT => NodeTypeInfo { name: "MUT", whitespace_like: false },
                    REF => NodeTypeInfo { name: "REF", whitespace_like: false },
                    TRAIT => NodeTypeInfo { name: "TRAIT", whitespace_like: false },
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
                    AND => NodeTypeInfo { name: "AND", whitespace_like: false },
                    OR => NodeTypeInfo { name: "OR", whitespace_like: false },
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
                    LINKAGE => NodeTypeInfo { name: "LINKAGE", whitespace_like: false },
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
                    TRAIT_DEF => NodeTypeInfo { name: "TRAIT_DEF", whitespace_like: false },
                    MEMBERS => NodeTypeInfo { name: "MEMBERS", whitespace_like: false },
                    TYPE_DEF => NodeTypeInfo { name: "TYPE_DEF", whitespace_like: false },
                    CONST_DEF => NodeTypeInfo { name: "CONST_DEF", whitespace_like: false },
                    EXTERN_BLOCK => NodeTypeInfo { name: "EXTERN_BLOCK", whitespace_like: false },
                    TYPE_PARAMETERS => NodeTypeInfo { name: "TYPE_PARAMETERS", whitespace_like: false },
                    TYPE_PARAMETER => NodeTypeInfo { name: "TYPE_PARAMETER", whitespace_like: false },
                    LIFETIME_PARAMETER => NodeTypeInfo { name: "LIFETIME_PARAMETER", whitespace_like: false },
                    VISIBILITY => NodeTypeInfo { name: "VISIBILITY", whitespace_like: false },
                    PATH => NodeTypeInfo { name: "PATH", whitespace_like: false },
                    TYPE_ARGUMENTS => NodeTypeInfo { name: "TYPE_ARGUMENTS", whitespace_like: false },
                    ALIAS => NodeTypeInfo { name: "ALIAS", whitespace_like: false },
                    PATH_TYPE => NodeTypeInfo { name: "PATH_TYPE", whitespace_like: false },
                    REFERENCE_TYPE => NodeTypeInfo { name: "REFERENCE_TYPE", whitespace_like: false },
                    PLACEHOLDER_TYPE => NodeTypeInfo { name: "PLACEHOLDER_TYPE", whitespace_like: false },
                    UNIT_TYPE => NodeTypeInfo { name: "UNIT_TYPE", whitespace_like: false },
                    PAREN_TYPE => NodeTypeInfo { name: "PAREN_TYPE", whitespace_like: false },
                    TUPLE_TYPE => NodeTypeInfo { name: "TUPLE_TYPE", whitespace_like: false },
                    ARRAY_TYPE => NodeTypeInfo { name: "ARRAY_TYPE", whitespace_like: false },
                    WILDCARD_PATTERN => NodeTypeInfo { name: "WILDCARD_PATTERN", whitespace_like: false },
                    TUPE_STRUCT_PATTERN => NodeTypeInfo { name: "TUPE_STRUCT_PATTERN", whitespace_like: false },
                    BINDING_PATTERN => NodeTypeInfo { name: "BINDING_PATTERN", whitespace_like: false },
                    PATH_PATTERN => NodeTypeInfo { name: "PATH_PATTERN", whitespace_like: false },
                    EXPR => NodeTypeInfo { name: "EXPR", whitespace_like: false },
                    LITERAL => NodeTypeInfo { name: "LITERAL", whitespace_like: false },
                    STRUCT_LITERAL => NodeTypeInfo { name: "STRUCT_LITERAL", whitespace_like: false },
                    STRUCT_LITERAL_FIELD => NodeTypeInfo { name: "STRUCT_LITERAL_FIELD", whitespace_like: false },
                    PATH_EXPR => NodeTypeInfo { name: "PATH_EXPR", whitespace_like: false },
                    UNIT_EXPR => NodeTypeInfo { name: "UNIT_EXPR", whitespace_like: false },
                    PAREN_EXPR => NodeTypeInfo { name: "PAREN_EXPR", whitespace_like: false },
                    TUPLE_EXPR => NodeTypeInfo { name: "TUPLE_EXPR", whitespace_like: false },
                    LAMBDA_EXPR => NodeTypeInfo { name: "LAMBDA_EXPR", whitespace_like: false },
                    BLOCK_EXPR => NodeTypeInfo { name: "BLOCK_EXPR", whitespace_like: false },
                    LET_STMT => NodeTypeInfo { name: "LET_STMT", whitespace_like: false },
                    EMPTY_STMT => NodeTypeInfo { name: "EMPTY_STMT", whitespace_like: false },
                    EXPR_STMT => NodeTypeInfo { name: "EXPR_STMT", whitespace_like: false },
                    IF_EXPR => NodeTypeInfo { name: "IF_EXPR", whitespace_like: false },
                    WHILE_EXPR => NodeTypeInfo { name: "WHILE_EXPR", whitespace_like: false },
                    LOOP_EXPR => NodeTypeInfo { name: "LOOP_EXPR", whitespace_like: false },
                    METHOD_CALL_EXPR => NodeTypeInfo { name: "METHOD_CALL_EXPR", whitespace_like: false },
                    CALL_EXPR => NodeTypeInfo { name: "CALL_EXPR", whitespace_like: false },
                    FIELD_EXPR => NodeTypeInfo { name: "FIELD_EXPR", whitespace_like: false },
                    INDEX_EXPR => NodeTypeInfo { name: "INDEX_EXPR", whitespace_like: false },
                    VALUE_ARGUMENT => NodeTypeInfo { name: "VALUE_ARGUMENT", whitespace_like: false },
                    REFERENCE_EXPR => NodeTypeInfo { name: "REFERENCE_EXPR", whitespace_like: false },
                    DEREFERENCE_EXPR => NodeTypeInfo { name: "DEREFERENCE_EXPR", whitespace_like: false },
                    NEGATION_EXPR => NodeTypeInfo { name: "NEGATION_EXPR", whitespace_like: false },
                    PRODUCT_EXPR => NodeTypeInfo { name: "PRODUCT_EXPR", whitespace_like: false },
                    SUM_EXPR => NodeTypeInfo { name: "SUM_EXPR", whitespace_like: false },
                    BIT_SHIFT => NodeTypeInfo { name: "BIT_SHIFT", whitespace_like: false },
                    BIT_AND => NodeTypeInfo { name: "BIT_AND", whitespace_like: false },
                    BIT_OR => NodeTypeInfo { name: "BIT_OR", whitespace_like: false },
                    LOGICAL_AND => NodeTypeInfo { name: "LOGICAL_AND", whitespace_like: false },
                    LOGICAL_OR => NodeTypeInfo { name: "LOGICAL_OR", whitespace_like: false },
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
                LexRule::new(STATIC, "static", None),
                LexRule::new(FOR, "for", None),
                LexRule::new(LOOP, "loop", None),
                LexRule::new(WHILE, "while", None),
                LexRule::new(MOVE, "move", None),
                LexRule::new(MUT, "mut", None),
                LexRule::new(REF, "ref", None),
                LexRule::new(TRAIT, "trait", None),
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


