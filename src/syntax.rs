#[derive(Debug)]
pub enum Val {
    Reg(Reg),
    Imm(i64),
    RegOffset(Reg, i64),
    Label(String),
    RegAddressing(Reg, Reg, i64),
}

#[derive(Debug)]
pub enum Reg {
    RAX,
    RBX,
    RCX,
    RSP,
    RDI,
    R15,
}

#[derive(Debug)]
pub enum Instr {
    Label(String),
    IMov(Val, Val),
    IAdd(Val, Val),
    ISub(Val, Val),
    IImul(Val, Val),
    ISar(Val, Val),
    IAnd(Val, Val),
    IXor(Val, Val),
    IOr(Val, Val),
    ICmp(Val, Val),
    ITest(Val, Val),
    IJmp(Val),
    IJe(Val),
    IJne(Val),
    IJo(Val),
    ICmove(Val, Val),
    ICmovl(Val, Val),
    ICmovle(Val, Val),
    ICmovg(Val, Val),
    ICmovge(Val, Val),
    ICall(String),
    IRet,
}

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
    Number(i64),
    Boolean(bool),
    Id(String),
    Let(Vec<(String, Expr)>, Box<Expr>),
    UnOp(Op1, Box<Expr>),
    BinOp(Op2, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Loop(Box<Expr>),
    Break(Box<Expr>),
    Set(String, Box<Expr>),
    Block(Vec<Expr>),
    Print(Box<Expr>),
    Tuple(Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    FunCall(String, Vec<Expr>),
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