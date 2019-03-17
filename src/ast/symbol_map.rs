use std::collections::HashMap;

#[derive(Debug)]
pub enum Symbol<'a> {
    Function { name: String, params: Vec<&'a Symbol<'a>>, ret: &'a Symbol<'a> },
    Variable { name: String, typ: &'a Symbol<'a> },
    Typ { name: String },
}

#[derive(Debug)]
pub struct Scope<'a> {
    scopes: Vec<Scope<'a>>,
    symbols: HashMap<String, Symbol<'a>>,
    parent: Option<&'a Scope<'a>>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Self {
        return Scope {scopes: vec!(), symbols: HashMap::new(), parent: None }
    }

    pub fn add_scope(&mut self, s: Scope<'a>) {
        self.scopes.push(s);
    }

    pub fn add_symbol(&mut self, s: String, sy: Symbol<'a>) {
        self.symbols.insert(s, sy);
    }

    pub fn lookup(&self, s: &String) -> Option<&Symbol<'a>> {
        if self.symbols.contains_key(s) {
            return Some(&self.symbols[s]);
        } else if let Some(parent) = self.parent {
            return parent.lookup(s);
        } else {
            return None;
        }
    }
}
