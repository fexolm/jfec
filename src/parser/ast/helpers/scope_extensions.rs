use std::io::{Error, ErrorKind};
use std::rc::Rc;
use indextree::{Arena, NodeId};
use super::ast::{Scope, Symbol, ScopeNode};

pub trait ScopeExtensions {
    fn try_lookup(self, s: &String, arena: &Arena<Scope>) -> Result<Rc<Symbol>, Error>;
}

impl ScopeExtensions for NodeId {
    fn try_lookup(self, s: &String, arena: &Arena<Scope>) -> Result<Rc<Symbol>, Error> {
        if let Some(sym) = self.lookup(s, arena) {
            Ok(sym)
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "Can not find symbol"))
        }
    }
}