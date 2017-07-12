use std::collections::HashSet;

use fall_tree::{File, AstNode, NodeType, Node, dump_file, TextRange, TextUnit, TextEdit};
use fall_tree::visitor::{Visitor, NodeVisitor};
use fall_tree::search::{child_of_type, ancestors, find_leaf_at_offset, ast_parent};
use ::{ast, LANG_FALL, RefKind, CallKind};
use ::syntax::*;

mod actions;
mod formatter;

use self::actions::{ACTIONS, ContextActionId};

pub fn parse(text: String) -> File {
    LANG_FALL.parse(text)
}

pub fn tree_as_text(file: &File) -> String {
    dump_file(file)
}

type Spans = Vec<(TextRange, &'static str)>;

pub fn highlight(file: &File) -> Spans {
    let file = ast(file);
    let mut spans = vec![];
    Visitor(&mut spans)
        .visit_nodes(&[EOL_COMMENT], |spans, node| {
            colorize_node(node, "comment", spans)
        })
        .visit_nodes(&[HASH_STRING, SIMPLE_STRING], |spans, node| {
            colorize_node(node, "string", spans)
        })
        .visit_nodes(&[RULE, VERBATIM, TOKENIZER, AST, NODE, CLASS, PUB, EXAMPLE], |spans, node| {
            colorize_node(node, "keyword", spans)
        })
        .visit_nodes(&[ERROR], |spans, node| {
            let range = if node.range().is_empty() {
                TextRange::from_len(node.range().start(), TextUnit::from_usize(1))
            } else {
                node.range()
            };
            spans.push((range, "error"))
        })
        .visit_nodes(&[PARAMETER], |spans, node| colorize_node(node, "value_parameter", spans))
        .visit::<LexRule, _>(|spans, rule| colorize_child(rule.node(), IDENT, "token", spans))
        .visit::<SynRule, _>(|spans, rule| colorize_child(rule.node(), IDENT, "rule", spans))
        .visit::<AstNodeDef, _>(|spans, rule| colorize_child(rule.node(), IDENT, "rule", spans))
        .visit::<RefExpr, _>(|spans, ref_| {
            let color = match ref_.resolve() {
                Some(RefKind::Token(_)) => "token",
                Some(RefKind::RuleReference { .. }) => "rule",
                Some(RefKind::Param(..)) => "value_parameter",
                None => "rule"
            };
            colorize_node(ref_.node(), color, spans)
        })
        .visit::<CallExpr, _>(|spans, call| {
            let color = match call.kind() {
                Err(_) => "unresolved",
                Ok(CallKind::RuleCall(..)) => "rule",
                _ => "builtin"
            };

            colorize_child(call.node(), IDENT, color, spans);
            colorize_child(call.node(), LANGLE, color, spans);
            colorize_child(call.node(), RANGLE, color, spans);
        })
        .visit::<Attributes, _>(|spans, attrs| {
            colorize_node(attrs.node(), "meta", spans)
        })
        .walk_recursively_children_first(file.node());
    return spans;

    fn colorize_node(node: Node, color: &'static str, spans: &mut Spans) {
        spans.push((node.range(), color))
    }

    fn colorize_child(node: Node, child: NodeType, color: &'static str, spans: &mut Spans) {
        if let Some(child) = child_of_type(node, child) {
            colorize_node(child, color, spans);
        }
    }
}

pub fn extend_selection(file: &File, range: TextRange) -> Option<TextRange> {
    let node = match find_node_at_range(&file, range) {
        Some(node) => node,
        None => return None,
    };

    match ancestors(node).skip_while(|n| n.range() == range).next() {
        None => None,
        Some(parent) => Some(parent.range()),
    }
}


pub fn collect_applicable_context_actions(file: &File, offset: TextUnit) -> Vec<ContextActionId> {
    ACTIONS.iter()
        .filter(|action| action.apply(file, offset).is_some())
        .map(|action| action.id())
        .collect()
}

pub fn apply_context_action(file: &File, offset: TextUnit, action_id: &str) -> TextEdit {
    let action = ACTIONS.iter().find(|action| action.id().0 == action_id).unwrap();
    action.apply(file, offset).unwrap().into_text_edit()
}

pub struct FileStructureNode {
    pub name: String,
    pub range: TextRange,
    pub children: Vec<FileStructureNode>
}

pub fn file_structure(file: &File) -> Vec<FileStructureNode> {
    let mut nodes = Vec::new();
    Visitor(&mut nodes)
        .visit::<SynRule, _>(|nodes, rule| {
            if let Some(name) = rule.name() {
                nodes.push(FileStructureNode {
                    name: name.to_string(),
                    range: rule.node().range(),
                    children: Vec::new(),
                })
            }
        })
        .walk_recursively_children_first(file.root());

    nodes
}

pub enum Severity {
    Error,
    Warning
}

pub struct Diagnostic {
    pub range: TextRange,
    pub severity: Severity,
    pub text: String
}

impl Diagnostic {
    fn error(node: Node, message: String) -> Diagnostic {
        Diagnostic {
            range: node.range(),
            severity: Severity::Error,
            text: message,
        }
    }

    fn warning(node: Node, message: String) -> Diagnostic {
        Diagnostic {
            range: node.range(),
            severity: Severity::Warning,
            text: message,
        }
    }
}

pub fn diagnostics(file: &File) -> Vec<Diagnostic> {
    let used_rules: HashSet<Node> = descendants_of_type::<RefExpr>(file.root())
        .into_iter()
        .filter_map(|node| node.resolve())
        .filter_map(|r| match r {
            RefKind::RuleReference(rule) => Some(rule.node()),
            _ => None
        })
        .chain(
            descendants_of_type::<CallExpr>(file.root())
                .into_iter()
                .filter_map(|call| call.kind().ok())
                .filter_map(|kind| match kind {
                    CallKind::RuleCall(rule, ..) => Some(rule.node()),
                    _ => None,
                })
        )
        .chain(child_of_type(file.root(), SYN_RULE).into_iter())
        .collect();

    let mut result: Vec<Diagnostic> = Vec::new();
    Visitor(&mut result)
        .visit::<RefExpr, _>(|acc, ref_| {
            if ref_.resolve().is_none() {
                if let Some(call) = ast_parent::<CallExpr>(ref_.node()) {
                    if call.resolve_context().is_some() {
                        return;
                    }
                }

                acc.push(Diagnostic::error(ref_.node(), "Unresolved reference".to_string()))
            }
        })
        .visit::<CallExpr, _>(|acc, call| {
            if let Err(e) = call.kind() {
                acc.push(Diagnostic::error(call.node(), e.to_string()))
            }
        })
        .visit::<SynRule, _>(|acc, rule| {
            if !used_rules.contains(&rule.node()) {
                acc.push(Diagnostic::warning(rule.node(), "Unused rule".to_string()))
            }
        })
        .walk_recursively_children_first(file.root());

    result
}

pub fn resolve_reference(file: &File, offset: TextUnit) -> Option<TextRange> {
    let node = match try_find_non_ws_node_at_offset(file, offset) {
        None => return None,
        Some(node) => node
    };

    if let Some(ref_expr) = ast_parent::<RefExpr>(node) {
        let target = match ref_expr.resolve() {
            None => return None,
            Some(t) => t
        };

        return match target {
            RefKind::RuleReference(rule) => Some(rule.node().range()),
            RefKind::Param(param) => Some(param.node().range()),
            RefKind::Token(token) => Some(token.node().range()),
        }
    };

    match node.parent() {
        Some(parent) if parent.ty() == CALL_EXPR => {
            let call = CallExpr::new(parent);
            if let Ok(CallKind::RuleCall(rule, ..)) = call.kind() {
                return Some(rule.node().range());
            }
        }
        _ => {}
    }
    None
}

pub fn reformat(file: &File) -> TextEdit {
    self::formatter::reformat_file(file, self::formatter::FALL_SPACING, WHITESPACE)
}

fn descendants_of_type<'f, N: AstNode<'f>>(node: Node<'f>) -> Vec<N> {
    let mut result = Vec::new();
    Visitor(&mut result)
        .visit::<N, _>(|acc, node| acc.push(node))
        .walk_recursively_children_first(node);
    result
}

fn find_node_at_range(file: &File, range: TextRange) -> Option<Node> {
    if range.is_empty() {
        return try_find_non_ws_node_at_offset(file, range.start());
    }

    let root = file.root();
    let (left, right) = match (
        find_leaf_at_offset(root, range.start()).right_biased(),
        find_leaf_at_offset(root, range.end()).left_biased()
    ) {
        (Some(l), Some(r)) => (l, r),
        _ => return None
    };

    Some(common_ancestor(left, right))
}

fn try_find_non_ws_node_at_offset(file: &File, offset: TextUnit) -> Option<Node> {
    let leaves = find_leaf_at_offset(file.root(), offset);
    if let Some(leaf) = leaves.left_biased() {
        if file.language().node_type_info(leaf.ty()).whitespace_like {
            return leaves.right_biased();
        }
    }

    leaves.left_biased()
}

fn common_ancestor<'f>(n1: Node<'f>, n2: Node<'f>) -> Node<'f> {
    for p in ancestors(n1) {
        if ancestors(n2).any(|a| a == p) {
            return p;
        }
    }
    panic!("Can't find common ancestor of {:?} and {:?}", n1, n2)
}
