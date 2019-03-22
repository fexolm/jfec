use std::io::{Error, ErrorKind};
use std::rc::Rc;

use super::ast;

pub trait ScopeExtensions {
    fn try_lookup(&self, s: &String) -> Result<Rc<ast::Symbol>, Error>;
}

impl ScopeExtensions for ast::Scope {
    fn try_lookup(&self, s: &String) -> Result<Rc<ast::Symbol>, Error> {
        if let Some(sym) = self.lookup(s) {
            Ok(sym)
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "Can not find symbol"))
        }
    }
}