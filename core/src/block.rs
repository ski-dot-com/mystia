use crate::*;

#[derive(Clone, Debug)]
pub struct Block(Vec<Stmt>);

impl Node for Block {
    fn parse(source: &str) -> Option<Block> {
        let mut result = vec![];
        for line in tokenize(source, &[";"], false)? {
            result.push(if line.trim().is_empty() {
                Stmt::Drop
            } else {
                Stmt::parse(&line)?
            })
        }
        Some(Block(result))
    }

    fn compile(&self, ctx: &mut Compiler) -> String {
        join!(self.0.iter().map(|x| x.compile(ctx)).collect::<Vec<_>>())
    }

    fn type_infer(&self, ctx: &mut Compiler) -> Type {
        let mut result = Type::Void;
        for line in &self.0 {
            result = line.type_infer(ctx);
        }
        result
    }
}
