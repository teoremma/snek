use im::HashSet;
use sexp::Atom::*;
use sexp::*;

use crate::types::*;

const RESERVED_WORDS: &[&str] = &[
    "true", "false", "input", "let", "set!", "if", "block", "loop", "break", "print", "fun", "tuple", "index",
    // unary operators
    "add1", "sub1", "isnum", "isbool", 
    // binary operators
    "+", "-", "*", "=", "<", "<=", ">", ">=",
];

fn is_reserved_word(s: &String) -> bool {
    RESERVED_WORDS.contains(&s.as_str())
}

fn is_valid_id(id: &String) -> bool {
    // A valid identifier starts with a lowercase letter
    id.chars().nth(0).unwrap().is_lowercase()
    // contains only alphanumeric characters and underscores
    && id.chars().all(|c| c.is_alphanumeric() || c == '_')
    // and is not a reserved word
    && !is_reserved_word(id)
}

fn parse_binding(s: &Sexp) -> (String, Expr) {
    match s {
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(id)), e] => {
                if is_valid_id(id) {
                    (id.to_string(), parse_expr(e))
                } else {
                    panic!("Invalid identifier or keyword: {}", id)
                }
            }
            _ => panic!("Invalid binding: {:?}", s),
        },
        _ => panic!("Invalid binding: {:?}", s),
    }
}

fn parse_bindings(s: &Sexp) -> Vec<(String, Expr)> {
    match s {
        Sexp::List(vec) => {
            if vec.len() == 0 {
                panic!("Invalid bindings: {:?}", s);
            }
            let bindings: Vec<(String, Expr)> = vec.iter().map(|s| parse_binding(s)).collect();
            let vars: HashSet<String> = bindings.iter().map(|(id, _)| id).collect();
            if vars.len() != bindings.len() {
                panic!("Invalid bindings: Duplicate binding {:?}", s);
            }
            bindings
        }
        _ => panic!("Invalid bindings: {:?}", s),
    }
}

fn parse_expr(s: &Sexp) -> Expr {
    match s {
        Sexp::Atom(I(i)) => Expr::Number(*i as i64),
        Sexp::Atom(S(s)) if s == "true" => Expr::Boolean(true),
        Sexp::Atom(S(s)) if s == "false" => Expr::Boolean(false),
        // TODO: We want to panic if we see "input" in a function definition
        Sexp::Atom(S(s)) if s == "input" => Expr::Id(s.to_string()),
        Sexp::Atom(S(s)) => {
            if is_valid_id(s) {
                Expr::Id(s.to_string())
            } else {
                panic!("Invalid identifier or keyword: {}", s)
            }
        }
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(op)), bindings, body] if op == "let" => {
                Expr::Let(parse_bindings(bindings), Box::new(parse_expr(body)))
            }
            [Sexp::Atom(S(op)), e] if op == "add1" => {
                Expr::UnOp(Op1::Add1, Box::new(parse_expr(e)))
            }
            [Sexp::Atom(S(op)), e] if op == "sub1" => {
                Expr::UnOp(Op1::Sub1, Box::new(parse_expr(e)))
            }
            [Sexp::Atom(S(op)), e] if op == "isnum" => {
                Expr::UnOp(Op1::IsNum, Box::new(parse_expr(e)))
            }
            [Sexp::Atom(S(op)), e] if op == "isbool" => {
                Expr::UnOp(Op1::IsBool, Box::new(parse_expr(e)))
            }
            [Sexp::Atom(S(op)), e1, e2] if op == "+" => Expr::BinOp(
                Op2::Plus,
                Box::new(parse_expr(e1)),
                Box::new(parse_expr(e2)),
            ),
            [Sexp::Atom(S(op)), e1, e2] if op == "-" => Expr::BinOp(
                Op2::Minus,
                Box::new(parse_expr(e1)),
                Box::new(parse_expr(e2)),
            ),
            [Sexp::Atom(S(op)), e1, e2] if op == "*" => Expr::BinOp(
                Op2::Times,
                Box::new(parse_expr(e1)),
                Box::new(parse_expr(e2)),
            ),
            [Sexp::Atom(S(op)), e1, e2] if op == "=" => Expr::BinOp(
                Op2::Equal,
                Box::new(parse_expr(e1)),
                Box::new(parse_expr(e2)),
            ),
            [Sexp::Atom(S(op)), e1, e2] if op == "<" => Expr::BinOp(
                Op2::Less,
                Box::new(parse_expr(e1)),
                Box::new(parse_expr(e2)),
            ),
            [Sexp::Atom(S(op)), e1, e2] if op == "<=" => Expr::BinOp(
                Op2::LessEqual,
                Box::new(parse_expr(e1)),
                Box::new(parse_expr(e2)),
            ),
            [Sexp::Atom(S(op)), e1, e2] if op == ">" => Expr::BinOp(
                Op2::Greater,
                Box::new(parse_expr(e1)),
                Box::new(parse_expr(e2)),
            ),
            [Sexp::Atom(S(op)), e1, e2] if op == ">=" => Expr::BinOp(
                Op2::GreaterEqual,
                Box::new(parse_expr(e1)),
                Box::new(parse_expr(e2)),
            ),
            [Sexp::Atom(S(op)), cond, thn, els] if op == "if" => Expr::If(
                Box::new(parse_expr(cond)),
                Box::new(parse_expr(thn)),
                Box::new(parse_expr(els)),
            ),
            [Sexp::Atom(S(op)), body] if op == "loop" => Expr::Loop(Box::new(parse_expr(body))),
            [Sexp::Atom(S(op)), e] if op == "break" => Expr::Break(Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), Sexp::Atom(S(id)), e] if op == "set!" => {
                if is_valid_id(id) {
                    Expr::Set(id.to_string(), Box::new(parse_expr(e)))
                } else {
                    panic!("Invalid identifier or keyword: {}", id)
                }
            }
            [Sexp::Atom(S(op)), exprs @ ..] if op == "block" => {
                if exprs.len() == 0 {
                    panic!("Invalid block: {:?}", s);
                } else {
                    Expr::Block(exprs.into_iter().map(parse_expr).collect())
                }
            }
            [Sexp::Atom(S(op)), e] if op == "print" => {
                Expr::Print(Box::new(parse_expr(e)))
            }
            // We allow tuples to have 0 elements
            [Sexp::Atom(S(op)), exprs @ ..] if op == "tuple" => {
                Expr::Tuple(exprs.into_iter().map(parse_expr).collect())
            }
            [Sexp::Atom(S(op)), e1, e2] if op == "index" => {
                Expr::Index(Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
            }
            // Function calls must be the last case since funname will capture anything
            [Sexp::Atom(S(funname)), args @ ..] => {
                if is_valid_id(funname) {
                    Expr::FunCall(funname.to_string(), args.into_iter().map(parse_expr).collect())
                } else {
                    panic!("Invalid function name in call: {}", funname)
                }
            }
            _ => panic!("Invalid expression: {:?}", s),
        },
        _ => panic!("Invalid expression: {:?}", s),
    }
}

fn parse_signature(s: &Sexp) -> (String, Vec<String>) {
    match s {
        Sexp::List(vec) => {
            let idents: Vec<String> = vec.into_iter()
            .map(|s| match s {
                Sexp::Atom(S(s)) => {
                    if is_valid_id(s) {
                        s.to_string()
                    } else {
                        panic!("Invalid identifier or keyword: {}", s)
                    }
                }
                _ => panic!("All elements in function signature must be identifiers: {:?}", s),
            }).collect();
            if idents.is_empty() { panic!("Function must have a name: {:?}", s) }
            let name = idents[0].clone();
            let params = idents[1..].to_vec();
            let unique_params: HashSet<String> = params.iter().cloned().collect();
            if params.len() != unique_params.len() {
                panic!("Function parameters must be unique: {:?}", s)
            }
            (name, params)
        },
        _ => panic!("Function signature must be a list: {:?}", s),
    }
}

fn is_fundef(s: &Sexp) -> bool {
    match s {
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(op)), Sexp::List(_), _] if op == "fun" => true,
            _ => false,
        },
        _ => false,
    }
}

fn parse_fundef(s: &Sexp, fun_env: &HashSet<String>) -> FunDef {
    match s {
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(op)), signature, body] if op == "fun" => {
                let (name, params) = parse_signature(signature);
                if fun_env.contains(&name) {
                    panic!("Function name must be unique: {:?}", s)
                }
                let body_expr = parse_expr(body);
                FunDef { name, params, body: body_expr }
            },
            _ => panic!("Invalid function definition format: {:?}", s),
        },
        _ => panic!("Function definition must be a list: {:?}", s),
    }
}

fn parse_program(s: &Sexp) -> Program {
    match s {
        Sexp::List(vec) => {
            let mut defs: Vec<FunDef> = vec![];
            let mut fun_env: HashSet<String> = HashSet::new();
            for def_or_exp in vec {
                if is_fundef(def_or_exp) {
                    let def = parse_fundef(def_or_exp, &fun_env);
                    fun_env.insert(def.name.clone());
                    defs.push(def);
                } else {
                    // if there are more than one expression in the program,
                    // only the first one is the main expression
                    return Program { defs, main: parse_expr(def_or_exp) }
                }
            };
            panic!("Only definitions found in program: {:?}", s);
        },
        _ => panic!("Program must be a list: {:?}", s),
    }
}

pub fn parse(s: &str) -> Program {
    // wrap s in parens to make it a valid S-expression
    let input_prog = format!("({})", s);
    let sexp_parse = match sexp::parse(&input_prog) {
        Ok(sexp) => sexp,
        Err(e) => panic!("Invalid expression: {}", e),
    };
    parse_program(&sexp_parse)
}
