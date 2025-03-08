mod block;
mod expr;
mod lexer;
mod node;
mod oper;
mod stmt;
mod utils;
mod value;

use {
    block::Block,
    expr::Expr,
    lexer::tokenize,
    node::Node,
    oper::Oper,
    std::collections::HashMap,
    stmt::Stmt,
    utils::{OPERATOR, SPACE, include_letter},
    value::{Type, Value},
};

#[derive(Debug, Clone)]
pub struct Compiler {
    declare: Vec<String>,
    variable: HashMap<String, Type>,
    function: HashMap<String, Type>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            declare: vec![],
            variable: HashMap::new(),
            function: HashMap::new(),
        }
    }

    pub fn build(&mut self, source: &str) -> Option<String> {
        Some(format!(
            r#"(module {1} (func (export "_start") (result i32) {0}))"#,
            Block::parse(source).map(|x| x.compile(self))?,
            join!(self.declare)
        ))
    }
}
