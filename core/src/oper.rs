use crate::*;

#[derive(Debug, Clone)]
pub enum Oper {
    Add(Expr, Expr),
    Sub(Expr, Expr),
    Mul(Expr, Expr),
    Div(Expr, Expr),
    Mod(Expr, Expr),
    Eql(Expr, Expr),
    Neq(Expr, Expr),
    Lt(Expr, Expr),
    Gt(Expr, Expr),
    LtEq(Expr, Expr),
    GtEq(Expr, Expr),
}

impl Node for Oper {
    fn parse(source: &str) -> Option<Self> {
        let token_list: Vec<String> = tokenize(source, SPACE.as_ref(), true)?;
        let token = Expr::parse(token_list.last()?)?;
        let operator = token_list.get(token_list.len().checked_sub(2)?)?;
        let has_lhs = |len: usize| Expr::parse(&join!(token_list.get(..token_list.len() - len)?));
        Some(match operator.as_str() {
            "+" => Oper::Add(has_lhs(2)?, token),
            "-" => Oper::Sub(has_lhs(2)?, token),
            "*" => Oper::Mul(has_lhs(2)?, token),
            "/" => Oper::Div(has_lhs(2)?, token),
            "%" => Oper::Mod(has_lhs(2)?, token),
            "==" => Oper::Eql(has_lhs(2)?, token),
            "!=" => Oper::Neq(has_lhs(2)?, token),
            "<" => Oper::Lt(has_lhs(2)?, token),
            ">" => Oper::Gt(has_lhs(2)?, token),
            ">=" => Oper::GtEq(has_lhs(2)?, token),
            "<=" => Oper::LtEq(has_lhs(2)?, token),
            _ => return None,
        })
    }

    fn compile(&self, ctx: &mut Compiler) -> String {
        match self {
            Oper::Add(lhs, rhs) => format!(
                "({}.add {} {})",
                self.type_infer(ctx).compile(ctx),
                lhs.compile(ctx),
                rhs.compile(ctx)
            ),
            Oper::Sub(lhs, rhs) => format!(
                "({}.sub {} {})",
                self.type_infer(ctx).compile(ctx),
                lhs.compile(ctx),
                rhs.compile(ctx)
            ),
            Oper::Mul(lhs, rhs) => format!(
                "({}.mul {} {})",
                self.type_infer(ctx).compile(ctx),
                lhs.compile(ctx),
                rhs.compile(ctx)
            ),
            Oper::Div(lhs, rhs) => format!(
                "({}.div_s {} {})",
                self.type_infer(ctx).compile(ctx),
                lhs.compile(ctx),
                rhs.compile(ctx)
            ),
            Oper::Mod(lhs, rhs) => format!(
                "({}.rem_s {} {})",
                self.type_infer(ctx).compile(ctx),
                lhs.compile(ctx),
                rhs.compile(ctx)
            ),
            Oper::Eql(lhs, rhs) => format!(
                "({}.eq {} {})",
                self.type_infer(ctx).compile(ctx),
                lhs.compile(ctx),
                rhs.compile(ctx)
            ),
            Oper::Neq(lhs, rhs) => format!(
                "({}.ne {} {})",
                self.type_infer(ctx).compile(ctx),
                lhs.compile(ctx),
                rhs.compile(ctx)
            ),
            Oper::Lt(lhs, rhs) => {
                let ret = self.type_infer(ctx).compile(ctx);
                format!(
                    "({}.lt{} {} {})",
                    ret,
                    if ret == "i32" { "_s" } else { "" },
                    lhs.compile(ctx),
                    rhs.compile(ctx)
                )
            }
            Oper::LtEq(lhs, rhs) => {
                let ret = self.type_infer(ctx).compile(ctx);
                format!(
                    "({}.le{} {} {})",
                    ret,
                    if ret == "i32" { "_s" } else { "" },
                    lhs.compile(ctx),
                    rhs.compile(ctx)
                )
            }
            Oper::Gt(lhs, rhs) => {
                let ret = self.type_infer(ctx).compile(ctx);
                format!(
                    "({}.gt{} {} {})",
                    ret,
                    if ret == "i32" { "_s" } else { "" },
                    lhs.compile(ctx),
                    rhs.compile(ctx)
                )
            }
            Oper::GtEq(lhs, rhs) => {
                let ret = self.type_infer(ctx).compile(ctx);
                format!(
                    "({}.ge{} {} {})",
                    ret,
                    if ret == "i32" { "_s" } else { "" },
                    lhs.compile(ctx),
                    rhs.compile(ctx)
                )
            }
        }
    }

    fn type_infer(&self, ctx: &mut Compiler) -> Type {
        match self {
            Oper::Add(lhs, rhs) => {
                lhs.type_infer(ctx);
                rhs.type_infer(ctx)
            }
            Oper::Sub(lhs, rhs) => {
                lhs.type_infer(ctx);
                rhs.type_infer(ctx)
            }
            Oper::Mul(lhs, rhs) => {
                lhs.type_infer(ctx);
                rhs.type_infer(ctx)
            }
            Oper::Div(lhs, rhs) => {
                lhs.type_infer(ctx);
                rhs.type_infer(ctx)
            }
            Oper::Mod(lhs, rhs) => {
                lhs.type_infer(ctx);
                rhs.type_infer(ctx)
            }
            Oper::Eql(lhs, rhs) => {
                lhs.type_infer(ctx);
                rhs.type_infer(ctx)
            }
            Oper::Neq(lhs, rhs) => {
                lhs.type_infer(ctx);
                rhs.type_infer(ctx)
            }
            Oper::Lt(lhs, rhs) => {
                lhs.type_infer(ctx);
                rhs.type_infer(ctx)
            }
            Oper::Gt(lhs, rhs) => {
                lhs.type_infer(ctx);
                rhs.type_infer(ctx)
            }
            Oper::LtEq(lhs, rhs) => {
                lhs.type_infer(ctx);
                rhs.type_infer(ctx)
            }
            Oper::GtEq(lhs, rhs) => {
                lhs.type_infer(ctx);
                rhs.type_infer(ctx)
            }
        }
    }
}
