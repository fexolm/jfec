use std::io::{Error, ErrorKind};

use super::ast;

pub trait StmtExtensions {
    fn as_local(&self) -> Result<&ast::Local, Error>;

    fn as_assign_mut(&mut self) -> Result<&mut ast::Local, Error>;

    fn as_block(&self) -> Result<&ast::Block, Error>;

    fn as_block_mut(&mut self) -> Result<&mut ast::Block, Error>;

    fn as_expr(&self) -> Result<&Box<ast::Expr>, Error>;

    fn as_expr_mut(&mut self) -> Result<&mut Box<ast::Expr>, Error>;
}

impl StmtExtensions for ast::StmtKind {
    fn as_local(&self) -> Result<&ast::Local, Error> {
        if let ast::StmtKind::Local(ref stmt) = self {
            Ok(stmt)
        } else {
            return Err(Error::new(ErrorKind::InvalidInput, "Can not convert to assign stmt"));
        }
    }

    fn as_assign_mut(&mut self) -> Result<&mut ast::Local, Error> {
        if let ast::StmtKind::Local(ref mut stmt) = self {
            Ok(stmt)
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "Can not convert to assign stmt"))
        }
    }

    fn as_block(&self) -> Result<&ast::Block, Error> {
        if let ast::StmtKind::Block(ref stmt) = self {
            Ok(stmt)
        } else {
            return Err(Error::new(ErrorKind::InvalidInput, "Can not convert to block stmt"));
        }
    }

    fn as_block_mut(&mut self) -> Result<&mut ast::Block, Error> {
        if let ast::StmtKind::Block(ref mut stmt) = self {
            Ok(stmt)
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "Can not convert to block stmt"))
        }
    }

    fn as_expr(&self) -> Result<&Box<ast::Expr>, Error> {
        if let ast::StmtKind::Expr(ref stmt) = self {
            Ok(stmt)
        } else {
            return Err(Error::new(ErrorKind::InvalidInput, "Can not convert to assign stmt"));
        }
    }

    fn as_expr_mut(&mut self) -> Result<&mut Box<ast::Expr>, Error> {
        if let ast::StmtKind::Expr(ref mut stmt) = self {
            Ok(stmt)
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "Can not convert to assign stmt"))
        }
    }
}