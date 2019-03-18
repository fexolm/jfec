use std::collections::HashMap;
use std::rc::Rc;
#[derive(Debug)]
pub enum Symbol {
    Function { name: String, params: Vec<Rc<Symbol>>, ret: Rc<Symbol> },
    Variable { name: String, typ: Rc<Symbol>},
    Typ (String),
}

#[derive(Debug)]
pub struct Scope {
    scopes: Vec<Rc<Scope>>,
    symbols: HashMap<String, Rc<Symbol>>,
    parent: Option<Rc<Scope>>,
}

impl Scope {
    pub fn new() -> Self {
        let mut s = Scope {scopes: vec!(), symbols: HashMap::new(), parent: None };
        s.init_types();
        return s;
    }

    pub fn add_scope(&mut self, s: Scope)-> Rc<Scope> {
        self.scopes.push(Rc::new(s));
        return self.scopes.last().cloned().unwrap();
    }

    pub fn add_symbol(&mut self, s: &String, sy: Symbol) -> Rc<Symbol>{
        self.symbols.insert(s.clone(), Rc::new(sy));
        return self.symbols[s].clone();
    }

    pub fn lookup(&self, s: &String) -> Option<Rc<Symbol>> {
        if self.symbols.contains_key(s) {
            return Some(self.symbols[s].clone());
        } else if let Some(ref parent) = &self.parent {
            return parent.lookup(s);
        } else {
            return None;
        }
    }

    fn init_types(&mut self) {
        self.add_symbol(&"int".to_string(), Symbol::Typ("int".to_string()));
        self.add_symbol(&"float".to_string(), Symbol::Typ("float".to_string()));
    }
}
