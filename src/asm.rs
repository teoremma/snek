// types

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
// x86 registers
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
// Memory address
pub enum MemAddr {
    // [base + index * scale + disp]
    MemAddr {
        base: Reg, 
        index: Option<Reg>,
        scale: Option<i64>,
        disp: i64,
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
// An argument to an instruction is either a register, an immediate value, or a value in memory
pub enum Arg {
    Reg(Reg),
    Imm(i64),
    Mem(MemAddr),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Instr {
    // a_label:
    Label(String),

    // Moves 
    Mov(Arg, Arg), // dst <- src

    // Arithmetic operations

    // // Binary operations
    Add(Arg, Arg), // dst += src
    Sub(Arg, Arg), // dst -= src 
    Imul(Arg, Arg), // dst *= src

    // // Shifts
    Sar(Arg, Arg), // dst >>= src, arithmetic

    // // Logic operations
    And(Arg, Arg), // dst &= src
    Or(Arg, Arg), // dst |= src
    Xor(Arg, Arg), // dst ^= src
    
    // Comparisons
    Cmp(Arg, Arg), // dst - src, sets flags
    Test(Arg, Arg), // dst & src, sets flags

    // Jumps
    Jmp(String), // unconditional jump
    Je(String), // jump if ==
    Jne(String), // jump if !=
    Jo(String), // jump if overflow

    // Conditional moves
    Cmove(Arg, Arg), // dst <- src if ==
    Cmovl(Arg, Arg), // dst <- src if <
    Cmovle(Arg, Arg), // dst <- src if <=
    Cmovg(Arg, Arg), // dst <- src if >
    Cmovge(Arg, Arg), // dst <- src if >=
    
    // Function call related
    Call(String), // push return address, jump to label
    Ret, // pop return address, jump to it
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
        Instr::Mov(v1, v2) => format!("mov {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::Add(v1, v2) => format!("add {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::Sub(v1, v2) => format!("sub {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::Imul(v1, v2) => format!("imul {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::Test(v1, v2) => format!("test {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::Cmove(v1, v2) => format!("cmove {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::Sar(v1, v2) => format!("sar {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::Xor(v1, v2) => format!("xor {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::And(v1, v2) => format!("and {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::Jne(v) => format!("jne {}", v),
        Instr::Cmp(v1, v2) => format!("cmp {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::Or(v1, v2) => format!("or {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::Cmovl(v1, v2) => format!("cmovl {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::Cmovle(v1, v2) => format!("cmovle {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::Cmovg(v1, v2) => format!("cmovg {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::Cmovge(v1, v2) => format!("cmovge {}, {}", arg_to_string(v1), arg_to_string(v2)),
        Instr::Je(v) => format!("je {}", v),
        Instr::Jmp(v) => format!("jmp {}", v),
        Instr::Jo(v) => format!("jo {}", v),
        Instr::Call(v) => format!("call {}", v),
        Instr::Ret => "ret".to_string(),
    }
}