//^
//^ HEAD
//^

//> HEAD -> STD
use std::collections::{
    HashMap as Map,
    HashSet as Set
};

//> HEAD -> SUPER
use super::{
    constants::DERIVATION_LENGTH,
    production::Production,
    rule::Rule,
    symbol::Symbol
};

//> HEAD -> CRATE
use crate::tokenizer::token::Token;

//> HEAD -> LIBUTILS
use libutils::stack_array::Array;


//^
//^ FOREST
//^

//> FOREST -> NODE ID
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(usize);

//> FOREST -> PRODUCTION ID
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProductionId {
    rule: &'static Rule,
    derivation: usize,
    symbols: &'static Array<Symbol, DERIVATION_LENGTH>
}

//> PRODUCTION ID -> IMPLEMENTATION
impl ProductionId {
    pub(crate) fn new(production: &Production) -> Self {return Self {
        rule: production.rule,
        derivation: production.derivation as *const _ as usize,
        symbols: production.derivation
    }}
    pub fn rule(&self) -> &'static Rule {return self.rule}
    pub fn symbols(&self) -> &'static [Symbol] {return self.symbols.as_ref()}
}

//> FOREST -> NODE LABEL
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NodeLabel<'input> {
    Terminal(&'input Token<'input>),
    EndOfFile,
    Nonterminal(&'static Rule),
    Intermediate {
        production: ProductionId,
        slot: usize
    }
}

//> FOREST -> PACKED NODE
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PackedNode {
    production: ProductionId,
    pivot: usize,
    left: Option<NodeId>,
    right: Option<NodeId>
}

//> PACKED NODE -> IMPLEMENTATION
impl PackedNode {
    pub fn production(&self) -> ProductionId {return self.production}
    pub fn pivot(&self) -> usize {return self.pivot}
    pub fn left(&self) -> Option<NodeId> {return self.left}
    pub fn right(&self) -> Option<NodeId> {return self.right}
}

//> FOREST -> NODE
pub struct Node<'input> {
    label: NodeLabel<'input>,
    left_extent: usize,
    right_extent: usize,
    families: Vec<PackedNode>,
    family_set: Set<PackedNode>
}

//> NODE -> IMPLEMENTATION
impl<'input> Node<'input> {
    pub fn label(&self) -> NodeLabel<'input> {return self.label}
    pub fn left_extent(&self) -> usize {return self.left_extent}
    pub fn right_extent(&self) -> usize {return self.right_extent}
    pub fn families(&self) -> &[PackedNode] {return &self.families}
}

//> FOREST -> NODE KEY
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct NodeKey<'input> {
    label: NodeLabel<'input>,
    left_extent: usize,
    right_extent: usize
}

//> FOREST -> STRUCT
pub struct Forest<'input> {
    nodes: Vec<Node<'input>>,
    interned: Map<NodeKey<'input>, NodeId>,
    roots: Vec<NodeId>,
    root_set: Set<NodeId>
}

//> FOREST -> DEFAULT
impl<'input> Default for Forest<'input> {
    fn default() -> Self {return Self {
        nodes: Vec::new(),
        interned: Map::new(),
        roots: Vec::new(),
        root_set: Set::new()
    }}
}

//> FOREST -> IMPLEMENTATION
impl<'input> Forest<'input> {
    pub fn nodes(&self) -> &[Node<'input>] {return &self.nodes}
    pub fn node(&self, node: NodeId) -> &Node<'input> {return &self.nodes[node.0]}
    pub fn roots(&self) -> &[NodeId] {return &self.roots}
    pub fn node_count(&self) -> usize {return self.nodes.len()}
    pub fn packed_node_count(&self) -> usize {return self.nodes.iter().map(|node| {
        node.families.len()
    }).sum()}
    pub(crate) fn terminal(
        &mut self,
        token: &'input Token<'input>,
        index: usize
    ) -> NodeId {return if matches!(token, Token::EndOfFile) {
        self.end_of_file(index)
    } else {
        self.intern(NodeLabel::Terminal(token), index, index + 1)
    }}
    pub(crate) fn end_of_file(&mut self, index: usize) -> NodeId {
        return self.intern(NodeLabel::EndOfFile, index, index + 1)
    }
    pub(crate) fn nonterminal(
        &mut self,
        rule: &'static Rule,
        left_extent: usize,
        right_extent: usize
    ) -> NodeId {
        return self.intern(
            NodeLabel::Nonterminal(rule),
            left_extent,
            right_extent
        );
    }
    pub(crate) fn intermediate(
        &mut self,
        production: ProductionId,
        slot: usize,
        left_extent: usize,
        right_extent: usize
    ) -> NodeId {
        return self.intern(
            NodeLabel::Intermediate {production, slot},
            left_extent,
            right_extent
        );
    }
    pub(crate) fn add_family(
        &mut self,
        parent: NodeId,
        production: ProductionId,
        pivot: usize,
        left: Option<NodeId>,
        right: Option<NodeId>
    ) -> bool {
        let node = &self.nodes[parent.0];
        debug_assert!(node.left_extent <= pivot && pivot <= node.right_extent);
        if let Some(left) = left {
            let left = &self.nodes[left.0];
            debug_assert_eq!(left.left_extent, node.left_extent);
            debug_assert_eq!(left.right_extent, pivot);
        }
        if let Some(right) = right {
            let right = &self.nodes[right.0];
            debug_assert_eq!(right.left_extent, pivot);
            debug_assert_eq!(right.right_extent, node.right_extent);
        }
        let family = PackedNode {production, pivot, left, right};
        let node = &mut self.nodes[parent.0];
        if node.family_set.insert(family) {
            node.families.push(family);
            return true;
        }
        return false;
    }
    pub(crate) fn add_root(&mut self, root: NodeId) -> () {
        if self.root_set.insert(root) {self.roots.push(root)}
    }
    fn intern(
        &mut self,
        label: NodeLabel<'input>,
        left_extent: usize,
        right_extent: usize
    ) -> NodeId {
        let key = NodeKey {label, left_extent, right_extent};
        if let Some(&node) = self.interned.get(&key) {return node}
        let node = NodeId(self.nodes.len());
        self.nodes.push(Node {
            label,
            left_extent,
            right_extent,
            families: Vec::new(),
            family_set: Set::new()
        });
        self.interned.insert(key, node);
        return node;
    }
}
