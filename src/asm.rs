// types

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

#[derive(Debug)]
pub enum Arg {
    Reg(Reg),
    Imm(i64),
    Mem(MemAddr),
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
    IJmp(String),
    IJe(String),
    IJne(String),
    IJo(String),
    ICmove(Arg, Arg),
    ICmovl(Arg, Arg),
    ICmovle(Arg, Arg),
    ICmovg(Arg, Arg),
    ICmovge(Arg, Arg),
    ICall(String),
    IRet,
}

// impls

pub fn maddr_b(base: Reg) -> MemAddr {
    MemAddr::MemAddr { base, index: None, scale: None, disp: 0 }
}

pub fn maddr_bd(base: Reg, disp: i64) -> MemAddr {
    MemAddr::MemAddr { base, index: None, scale: None, disp }
}

pub fn maddr_bisd(base: Reg, index: Reg, scale: i64, disp: i64) -> MemAddr {
    MemAddr::MemAddr { base, index: Some(index), scale: Some(scale), disp }
}

fn reg_to_string(r: &Reg) -> String {
    match r {
        Reg::Rax => "rax".to_string(),
        Reg::Rbx => "rbx".to_string(),
        Reg::Rcx => "rcx".to_string(),
        Reg::Rdx => "rdx".to_string(),
        Reg::Rsp => "rsp".to_string(),
        Reg::Rbp => "rbp".to_string(),
        Reg::Rsi => "rsi".to_string(),
        Reg::Rdi => "rdi".to_string(),
        Reg::R15 => "r15".to_string(),
    }
}

fn arg_to_string(v: &Arg) -> String {
    match v {
        Arg::Reg(r) => reg_to_string(r),
        Arg::Imm(i) => i.to_string(),
        Arg::Mem(MemAddr::MemAddr { base, index, scale, disp }) => {
            let mut s = format!("[{}", reg_to_string(base));
            if let Some(index) = index {
                let mut n = 1;
                if let Some(scale) = scale {
                    n = *scale;
                }
                s = format!("{} + {} * {}", s, reg_to_string(index), n);
            }
            if *disp > 0 {
                s = format!("{} + {}", s, disp);
            } else if *disp < 0 {
                s = format!("{} - {}", s, -disp);
            }
            format!("{}]", s)
        }
    }
}

pub fn instr_to_string(i: &Instr) -> String {
    match i {
        Instr::Label(l) => format!("{}:", l),
        Instr::IMov(v1, v2) => format!("mov {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::IAdd(v1, v2) => format!("add {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::ISub(v1, v2) => format!("sub {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::IImul(v1, v2) => format!("imul {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::ITest(v1, v2) => format!("test {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::ICmove(v1, v2) => format!("cmove {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::ISar(v1, v2) => format!("sar {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::IXor(v1, v2) => format!("xor {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::IAnd(v1, v2) => format!("and {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::IJne(v) => format!("jne {}", v),
        Instr::ICmp(v1, v2) => format!("cmp {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::IOr(v1, v2) => format!("or {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::ICmovl(v1, v2) => format!("cmovl {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::ICmovle(v1, v2) => format!("cmovle {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::ICmovg(v1, v2) => format!("cmovg {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::ICmovge(v1, v2) => format!("cmovge {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::IJe(v) => format!("je {}", v),
        Instr::IJmp(v) => format!("jmp {}", v),
        Instr::IJo(v) => format!("jo {}", v),
        Instr::ICall(v) => format!("call {}", v),
        Instr::IRet => "ret".to_string(),
    }
}