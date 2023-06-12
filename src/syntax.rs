#[derive(Debug)]
pub enum Op1 {
    Add1,
    Sub1,
    IsNum,
    IsBool,
}

#[derive(Debug)]
pub enum Op2 {
    Plus,
    Minus,
    Times,
    Equal,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Debug)]
pub enum Expr {
    // All expressions are values

    // primitive values
    // signed integers
    Number(i64),

    // true, false
    Boolean(bool),

    // variables
    Var(String),

    // (let ((<x_1> <e_1>) +) <e>)
    // Evaluates each e_i in order, binding the result to x_i,
    // then evaluates e in the resulting environment
    Let(Vec<(String, Expr)>, Box<Expr>),
    
    // (unop <e>)
    // Applies the unary operator unop to the value of e
    UnOp(Op1, Box<Expr>),

    // (binop <e_1> <e_2>)
    // Applies the binary operator binop to the values of e_1 and e_2
    BinOp(Op2, Box<Expr>, Box<Expr>),

    // (if <cond> <then> <else>)
    // Evaluates cond, then evaluates one of the branches depending on the result
    If(Box<Expr>, Box<Expr>, Box<Expr>),

    // (loop <e>)
    // Evaluates e indefinitely
    // Can be broken if a break is encountered
    Loop(Box<Expr>),

    // (break <e>)
    // Breaks out of the innermost loop, returning the value of e
    Break(Box<Expr>),

    // (set! <x> <e>)
    // Evaluates e, then sets the value of the variable x to the result
    Set(String, Box<Expr>),

    // (block <e_1> +)
    // Evaluates each e_i in order, returning the value of the last one
    Block(Vec<Expr>),

    // (print <e>)
    // Evaluates e, prints then returns the result
    Print(Box<Expr>),

    // (tup <e> *)
    // Heap allocated tuple of values, fixed size
    // Arity can be 0
    Tup(Vec<Expr>),

    // (tup-get <t> <i>)
    // Indexes into a tuple
    TupGet(Box<Expr>, Box<Expr>),

    // (tup-set <t> <i> <e>)
    // Sets the value at index i in tuple t to e
    TupSet(Box<Expr>, Box<Expr>, Box<Expr>),

    // (tup-len <t>)
    // Returns the length of a tuple
    TupLen(Box<Expr>),

    // (<f> <e> *)
    // Calls f with the values of e_i as parameters
    Call(String, Vec<Expr>),
}

#[derive(Debug)]
pub struct FunDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: Expr,
}

#[derive(Debug)]
pub struct Program {
    pub defs: Vec<FunDef>,
    pub main: Expr,
}