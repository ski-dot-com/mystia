use crate::*;

#[derive(Clone, Debug)]
pub enum Stmt {
    Defun {
        name: String,
        args: Vec<String>,
        body: Expr,
    },
    If {
        cond: Expr,
        then: Expr,
        r#else: Box<Stmt>,
    },
    Declare(String),
    Assign(String, Expr),
    Expr(Expr),
}

impl Stmt {
    pub fn parse(source: &str) -> Option<Self> {
        let source = source.trim();
        if let Some(source) = source.strip_prefix("fn ") {
            let (name, source) = source.split_once("(")?;
            let (args, body) = source.split_once(")")?;
            Some(Stmt::Defun {
                name: name.trim().to_string(),
                args: args.split(",").map(|x| x.trim().to_string()).collect(),
                body: Expr::parse(body)?,
            })
        } else if let Some(source) = source.strip_prefix("if ") {
            let code = tokenize(source, SPACE.as_ref(), false)?;
            let then_pos = code.iter().position(|i| i == "then")?;
            let else_pos = code.iter().position(|i| i == "else")?;
            let cond_sec = join!(code.get(0..then_pos)?);
            let then_sec = join!(code.get(then_pos + 1..else_pos)?);
            let else_sec = join!(code.get(else_pos + 1..)?);
            Some(Stmt::If {
                cond: Expr::parse(&cond_sec)?,
                then: Expr::parse(&then_sec)?,
                r#else: Box::new(Stmt::parse(&else_sec)?),
            })
        } else if let Some(source) = source.strip_prefix("declare ") {
            Some(Stmt::Declare(source.trim().to_string()))
        } else if let Some(source) = source.strip_prefix("let ") {
            let (name, source) = source.split_once("=")?;
            Some(Stmt::Assign(name.trim().to_string(), Expr::parse(source)?))
        } else {
            Some(Stmt::Expr(Expr::parse(source)?))
        }
    }

    pub fn compile(&self, ctx: &mut Compiler) -> String {
        match self {
            Stmt::Expr(expr) => expr.compile(ctx),
            Stmt::Defun { name, args, body } => {
                let code = format!(
                    "(func ${name} {} (result i32) {})",
                    join!(
                        args.iter()
                            .map(|x| format!("(param ${x} i32)"))
                            .collect::<Vec<_>>()
                    ),
                    body.compile(ctx)
                );
                ctx.declare.push(code);
                String::new()
            }
            Stmt::If { cond, then, r#else } => {
                format!(
                    "(if (result i32) {} (then {}) (else {}))",
                    cond.compile(ctx),
                    then.compile(ctx),
                    r#else.compile(ctx)
                )
            }
            Stmt::Declare(name) => format!("(local ${name} i32)"),
            Stmt::Assign(name, expr) => format!("(local.set ${name} {})", expr.compile(ctx)),
        }
    }
}
