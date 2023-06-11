#[derive(Debug)]
pub enum Reg {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsp,
    Rbp,
    Rsi,
    Rdi,
    R15,
}

#[derive(Debug)]
pub enum MemAddr {
    MemAddr {
        base: Reg, 
        index: Option<Reg>,
        scale: Option<i64>,
        disp: i64,
    }
}

#[macro_export]
macro_rules! maddr {
    ($base:ident) => {
        MemAddr {
            base: $base,
            index: None,
            scale: None,
            disp: 0,
        }
    };
    ($base:ident + $disp:literal) => {
        MemAddr {
            base: $base,
            index: None,
            scale: None,
            disp: $disp,
        }
    };
    ($base:ident - $disp:literal) => {
        MemAddr {
            base: $base,
            index: None,
            scale: None,
            disp: -$disp,
        }
    };
    ($base:ident + $index:ident * $scale:literal) => {
        MemAddr {
            base: $base,
            index: Some($index),
            scale: Some($scale),
            disp: 0,
        }
    };
    ($base:ident + $index:ident * $scale:literal + $disp: literal) => {
        MemAddr {
            base: $base,
            index: Some($index),
            scale: Some($scale),
            disp: $disp,
        }
    };
}


#[derive(Debug)]
pub enum Arg {
    Reg(Reg),
    Imm(i64),
    RegOffset(Reg, i64),
    Label(String),
    RegAddressing(Reg, Reg, i64),
}

#[derive(Debug)]
pub enum Instr {
    Label(String),
    IMov(Arg, Arg),
    IAdd(Arg, Arg),
    ISub(Arg, Arg),
    IImul(Arg, Arg),
    ISar(Arg, Arg),
    IAnd(Arg, Arg),
    IXor(Arg, Arg),
    IOr(Arg, Arg),
    ICmp(Arg, Arg),
    ITest(Arg, Arg),
    IJmp(Arg),
    IJe(Arg),
    IJne(Arg),
    IJo(Arg),
    ICmove(Arg, Arg),
    ICmovl(Arg, Arg),
    ICmovle(Arg, Arg),
    ICmovg(Arg, Arg),
    ICmovge(Arg, Arg),
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