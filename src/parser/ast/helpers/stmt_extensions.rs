use std::io::{Error, ErrorKind};

use super::ast;

pub trait StmtExtensions {
    fn as_assign(&self) -> Result<&ast::AssignStmt, Error>;

    fn as_assign_mut(&mut self) -> Result<&mut ast::AssignStmt, Error>;

    fn as_block(&self) -> Result<&ast::BlockStmt, Error>;

    fn as_block_mut(&mut self) -> Result<&mut ast::BlockStmt, Error>;

    fn as_expr(&self) -> Result<&Box<ast::Expr>, Error>;

    fn as_expr_mut(&mut self) -> Result<&mut Box<ast::Expr>, Error>;
}

impl StmtExtensions for ast::Stmt {
    fn as_assign(&self) -> Result<&ast::AssignStmt, Error> {
        if let ast::Stmt::Assign(ref stmt) = self {
            Ok(stmt)
        } else {
            return Err(Error::new(ErrorKind::InvalidInput, "Can not convert to assign stmt"));
        }
    }

    fn as_assign_mut(&mut self) -> Result<&mut ast::AssignStmt, Error> {
        if let ast::Stmt::Assign(ref mut stmt) = self {
            Ok(stmt)
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "Can not convert to assign stmt"))
        }
    }

    fn as_block(&self) -> Result<&ast::BlockStmt, Error> {
        if let ast::Stmt::Block(ref stmt) = self {
            Ok(stmt)
        } else {
            return Err(Error::new(ErrorKind::InvalidInput, "Can not convert to block stmt"));
        }
    }

    fn as_block_mut(&mut self) -> Result<&mut ast::BlockStmt, Error> {
        if let ast::Stmt::Block(ref mut stmt) = self {
            Ok(stmt)
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "Can not convert to block stmt"))
        }
    }

    fn as_expr(&self) -> Result<&Box<ast::Expr>, Error> {
        if let ast::Stmt::Expr(ref stmt) = self {
            Ok(stmt)
        } else {
            return Err(Error::new(ErrorKind::InvalidInput, "Can not convert to assign stmt"));
        }
    }

    fn as_expr_mut(&mut self) -> Result<&mut Box<ast::Expr>, Error> {
        if let ast::Stmt::Expr(ref mut stmt) = self {
            Ok(stmt)
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "Can not convert to assign stmt"))
        }
    }
}