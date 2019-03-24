use indextree::{Arena, NodeId};
use std::collections::HashMap;
use std::rc::Rc;

pub type Typ = String;

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub typ: Rc<Symbol>,
}

#[derive(Debug)]
pub enum Symbol {
    Variable(Variable),
    Typ(Typ),
}

impl Symbol {
    pub fn new_var(name: String, typ: Rc<Symbol>) -> Self {
        Symbol::Variable(Variable { name, typ })
    }

    pub fn new_typ(name: String) -> Self {
        Symbol::Typ(name)
    }
}

#[derive(Debug, Default)]
pub struct Scope {
    symbols: HashMap<String, Rc<Symbol>>,
}

impl Scope {
    pub fn new(arena: &mut Arena<Scope>) -> NodeId {
        arena.new_node(Scope::default())
    }
}

pub trait ScopeNode {
    fn add_child(self, arena: &mut Arena<Scope>) -> NodeId;

    fn add_symbol(self, s: &String, sy: Symbol, arena: &mut Arena<Scope>) -> Rc<Symbol>;

    fn lookup(self, s: &String, arena: &Arena<Scope>) -> Option<Rc<Symbol>>;
}

impl ScopeNode for NodeId {
    fn add_child(self, arena: &mut Arena<Scope>) -> NodeId{
        let id = arena.new_node(Scope::default());
        self.append(id, arena).expect("Compiler exception. Failed to create new node.");
        id
    }

    fn add_symbol(self, s: &String, sy: Symbol, arena: &mut Arena<Scope>) -> Rc<Symbol> {
        let scope = &mut arena.get_mut(self).unwrap().data;
        scope.symbols.insert(s.clone(), Rc::new(sy));
        return scope.symbols[s].clone();
    }

    fn lookup(self, s: &String, arena: &Arena<Scope>) -> Option<Rc<Symbol>> {
        let node = arena.get(self).unwrap();

        if node.data.symbols.contains_key(s) {
            return Some(node.data.symbols[s].clone());
        } else if let Some(ref parent) = node.parent() {
            return parent.lookup(s, arena);
        } else {
            return None;
        }
    }
}