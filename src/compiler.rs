use im::HashMap;

use crate::syntax::*;

fn reg_to_str(r: &Reg) -> String {
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

fn val_to_str(v: &Arg) -> String {
    match v {
        Arg::Reg(r) => reg_to_str(r),
        Arg::Imm(i) => i.to_string(),
        Arg::RegOffset(r, i) => {
            if *i == 0 {
                return format!("[{}]", reg_to_str(r));
            } else if *i < 0 {
                return format!("[{} - {}]", reg_to_str(r), -i);
            } else {
                return format!("[{} + {}]", reg_to_str(r), i);
            }
        }
        Arg::Label(l) => l.to_string(),
        Arg::RegAddressing(base, index, scale) => format!("[{} + {} * {}]", reg_to_str(base), reg_to_str(index), scale),
    }
}

fn instr_to_str(i: &Instr) -> String {
    match i {
        Instr::Label(l) => format!("{}:", l),
        Instr::IMov(v1, v2) => format!("mov {}, {}", val_to_str(v1), val_to_str(v2)),
        Instr::IAdd(v1, v2) => format!("add {}, {}", val_to_str(v1), val_to_str(v2)),
        Instr::ISub(v1, v2) => format!("sub {}, {}", val_to_str(v1), val_to_str(v2)),
        Instr::IImul(v1, v2) => format!("imul {}, {}", val_to_str(v1), val_to_str(v2)),
        Instr::ITest(v1, v2) => format!("test {}, {}", val_to_str(v1), val_to_str(v2)),
        Instr::ICmove(v1, v2) => format!("cmove {}, {}", val_to_str(v1), val_to_str(v2)),
        Instr::ISar(v1, v2) => format!("sar {}, {}", val_to_str(v1), val_to_str(v2)),
        Instr::IXor(v1, v2) => format!("xor {}, {}", val_to_str(v1), val_to_str(v2)),
        Instr::IAnd(v1, v2) => format!("and {}, {}", val_to_str(v1), val_to_str(v2)),
        Instr::IJne(v) => format!("jne {}", val_to_str(v)),
        Instr::ICmp(v1, v2) => format!("cmp {}, {}", val_to_str(v1), val_to_str(v2)),
        Instr::IOr(v1, v2) => format!("or {}, {}", val_to_str(v1), val_to_str(v2)),
        Instr::ICmovl(v1, v2) => format!("cmovl {}, {}", val_to_str(v1), val_to_str(v2)),
        Instr::ICmovle(v1, v2) => format!("cmovle {}, {}", val_to_str(v1), val_to_str(v2)),
        Instr::ICmovg(v1, v2) => format!("cmovg {}, {}", val_to_str(v1), val_to_str(v2)),
        Instr::ICmovge(v1, v2) => format!("cmovge {}, {}", val_to_str(v1), val_to_str(v2)),
        Instr::IJe(v) => format!("je {}", val_to_str(v)),
        Instr::IJmp(v) => format!("jmp {}", val_to_str(v)),
        Instr::IJo(v) => format!("jo {}", val_to_str(v)),
        Instr::ICall(v) => format!("call {}", v),
        Instr::IRet => "ret".to_string(),
    }
}

// Returns the internal representation of constant values
fn repr(e: &Expr) -> i64 {
    match e {
        Expr::Number(n) => n << 1,
        Expr::Boolean(true) => 7,
        Expr::Boolean(false) => 3,
        _ => panic!("Not a constant value: {:?}", e),
    }
}

fn repr_true() -> i64 { repr(&Expr::Boolean(true)) }
fn repr_false() -> i64 { repr(&Expr::Boolean(false)) }
fn repr_n(n: i64) -> i64 { repr(&Expr::Number(n)) }

fn new_label(l: &mut i64, s: &str) -> String {
    let current = *l;
    *l += 1;
    format!("{s}_{current}")
}

// A set of instructions that copy the error code in RBX to RDI and call snek_error
fn error_handler() -> Vec<Instr> {
    vec![
        Instr::Label("snek_error_handler".to_string()),
        Instr::IMov(Arg::Reg(Reg::Rdi), Arg::Reg(Reg::Rbx)),
        // TODO: is this call or jump?
        Instr::ICall("snek_error".to_string()),
    ]
}

// Instructions that error with code 1 if the values in RAX and RCX are of different types
// Not using RBX since we are using that to store the error code for the error handler
fn error_rax_rcx_diff_type() -> Vec<Instr> {
    vec![
        // Set error code to 1
        Instr::IMov(Arg::Reg(Reg::Rbx), Arg::Imm(1)),
        // Get the matching bits of RAX and RCX
        Instr::IXor(Arg::Reg(Reg::Rcx), Arg::Reg(Reg::Rax)),
        // and test if 1 bit is set (1st bits are equal)
        Instr::ITest(Arg::Reg(Reg::Rcx), Arg::Imm(1)),
        // If not, jump to error handler
        Instr::IJne(Arg::Label("snek_error_handler".to_string())),
    ]
}

// Instructions that error with code 2 if the value in RAX is not a number
fn error_rax_not_num() -> Vec<Instr> {
    vec![
        Instr::IMov(Arg::Reg(Reg::Rbx), Arg::Imm(2)),
        Instr::ITest(Arg::Reg(Reg::Rax), Arg::Imm(1)),
        Instr::IJne(Arg::Label("snek_error_handler".to_string())),
    ]
}

// Instructions that error with code 3 if there is an overflow
fn error_overflow() -> Vec<Instr> {
    vec![
        Instr::IMov(Arg::Reg(Reg::Rbx), Arg::Imm(3)),
        Instr::IJo(Arg::Label("snek_error_handler".to_string())),
    ]
}
// Instructions that error with code 4 if the value in RAX is not a tuple
fn error_rax_not_tuple() -> Vec<Instr> {
    vec![
        // Copy the value to rcx
        Instr::IMov(Arg::Reg(Reg::Rcx), Arg::Reg(Reg::Rax)),
        // get the two least significant bits
        Instr::IAnd(Arg::Reg(Reg::Rcx), Arg::Imm(3)),
        // and test if they are 1 (the value is a tuple)
        Instr::ICmp(Arg::Reg(Reg::Rcx), Arg::Imm(1)),
        // If not, jump to error handler with code 4
        Instr::IMov(Arg::Reg(Reg::Rbx), Arg::Imm(4)),
        Instr::IJne(Arg::Label("snek_error_handler".to_string())),
    ]
}

// Integer values are shifted left by 1
fn compile_expr(
    e: &Expr,
    si: i64,
    env: &HashMap<String, i64>,
    brake: &String,
    l: &mut i64,
) -> Vec<Instr> {
    match e {
        Expr::Number(n) => {
            if let (our_n, false) = n.overflowing_mul(2) {
                vec![Instr::IMov(Arg::Reg(Reg::Rax), Arg::Imm(our_n))]
            } else {
                panic!("Invalid integer constant: overflow {}", n)
            }
        }
        Expr::Boolean(true) => vec![Instr::IMov(Arg::Reg(Reg::Rax), Arg::Imm(repr_true()))],
        Expr::Boolean(false) => vec![Instr::IMov(Arg::Reg(Reg::Rax), Arg::Imm(repr_false()))],
        Expr::Id(id) if id == "input" => vec![Instr::IMov(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rdi))],
        Expr::Id(id) => {
            if env.contains_key(id) {
                vec![Instr::IMov(
                    Arg::Reg(Reg::Rax),
                    Arg::RegOffset(Reg::Rsp, - *env.get(id).unwrap()),
                )]
            } else {
                panic!("Unbound variable identifier {}", id)
            }
        }
        Expr::Let(bindings, body) => {
            let mut instrs = vec![];
            let mut new_env = env.clone();
            for (i, (id, e)) in bindings.into_iter().enumerate() {
                let stack_offset = (si + i as i64) * 8;
                instrs.append(&mut compile_expr(e, si + 1, &new_env, brake, l));
                instrs.push(Instr::IMov(
                    Arg::RegOffset(Reg::Rsp, -stack_offset),
                    Arg::Reg(Reg::Rax),
                ));
                new_env.insert(id.to_string(), stack_offset);
            }
            instrs.append(&mut compile_expr(
                body,
                si + bindings.len() as i64,
                &new_env,
                brake,
                l,
            ));
            instrs
        }
        Expr::UnOp(op, e) => {
            let mut instrs = compile_expr(e, si, env, brake, l);
            match op {
                Op1::Add1 => {
                    instrs.append(&mut error_rax_not_num());
                    instrs.push(Instr::IAdd(Arg::Reg(Reg::Rax), Arg::Imm(2)));
                    instrs.append(&mut error_overflow());
                }
                Op1::Sub1 => {
                    instrs.append(&mut error_rax_not_num());
                    instrs.push(Instr::ISub(Arg::Reg(Reg::Rax), Arg::Imm(2)));
                    instrs.append(&mut error_overflow());
                }
                Op1::IsNum => {
                    let mut comp_instrs = vec![
                        // Test rax & 1, this will be 0 iff rax represents a number
                        Instr::ITest(Arg::Reg(Reg::Rax), Arg::Imm(1)),
                        Instr::IMov(Arg::Reg(Reg::Rax), Arg::Imm(repr_false())),
                        Instr::IMov(Arg::Reg(Reg::Rbx), Arg::Imm(repr_true())),
                        Instr::ICmove(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx)),
                    ];
                    instrs.append(&mut comp_instrs);
                }
                Op1::IsBool => {
                    let mut comp_instrs = vec![
                        Instr::ITest(Arg::Reg(Reg::Rax), Arg::Imm(1)),
                        Instr::IMov(Arg::Reg(Reg::Rax), Arg::Imm(repr_true())),
                        Instr::IMov(Arg::Reg(Reg::Rbx), Arg::Imm(repr_false())),
                        Instr::ICmove(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx)),
                    ];
                    instrs.append(&mut comp_instrs);
                }
                _ => panic!("Unimplemented unary operator: {:?}", op),
            }
            instrs
        }
        Expr::BinOp(op, e1, e2) => {
            match op {
                Op2::Equal => {
                    // First evaluate e1
                    let mut instrs = compile_expr(e1, si, env, brake, l);
                    // Save the result in the current stack index
                    let stack_offset = si * 8;
                    instrs.push(Instr::IMov(Arg::RegOffset(Reg::Rsp, -stack_offset), Arg::Reg(Reg::Rax)));
                    // Then evaluate e2
                    instrs.append(&mut compile_expr(e2, si + 1, env, brake, l));
                    // Copy the result of e1 to rcx
                    instrs.push(Instr::IMov(Arg::Reg(Reg::Rcx), Arg::RegOffset(Reg::Rsp, -stack_offset)));
                    // Error if e1 and e2 are of different types
                    instrs.append(&mut error_rax_rcx_diff_type());
                    // If they are of the same type, do a regular comparison
                    // and set the result accordingly
                    instrs.append(&mut vec![
                        Instr::ICmp(Arg::Reg(Reg::Rax), Arg::RegOffset(Reg::Rsp, -stack_offset)),
                        Instr::IMov(Arg::Reg(Reg::Rax), Arg::Imm(repr_false())),
                        Instr::IMov(Arg::Reg(Reg::Rbx), Arg::Imm(repr_true())),
                        Instr::ICmove(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx)),
                    ]);
                    instrs
                }
                // These operator expect both operands to be numbers
                Op2::Plus
                | Op2::Minus
                | Op2::Times
                | Op2::Less
                | Op2::LessEqual
                | Op2::Greater
                | Op2::GreaterEqual => {
                    // Evaluate e1
                    let mut instrs = compile_expr(e1, si, env, brake, l);
                    // error if the result is not a number
                    instrs.append(&mut error_rax_not_num());
                    // Otherwise, save result in the current stack index
                    let stack_offset = si * 8;
                    instrs.push(Instr::IMov(Arg::RegOffset(Reg::Rsp, -stack_offset), Arg::Reg(Reg::Rax)));
                    // Evaluate e2
                    instrs.append(&mut compile_expr(e2, si + 1, env, brake, l));
                    // error if the result is not a number
                    instrs.append(&mut error_rax_not_num());
                    // the remaining instructions depend on the operator
                    match op {
                        Op2::Plus => {
                            instrs.push(Instr::IAdd(
                                Arg::Reg(Reg::Rax),
                                Arg::RegOffset(Reg::Rsp, -stack_offset),
                            ));
                            instrs.append(&mut error_overflow());
                        }
                        Op2::Minus => {
                            // The expected order is the opposite than the sub instruction
                            instrs.push(Instr::ISub(
                                Arg::RegOffset(Reg::Rsp, -stack_offset),
                                Arg::Reg(Reg::Rax),
                            ));
                            instrs.append(&mut error_overflow());
                            instrs.push(Instr::IMov(
                                Arg::Reg(Reg::Rax),
                                Arg::RegOffset(Reg::Rsp, -stack_offset),
                            ));
                        }
                        Op2::Times => {
                            // We need to divide one of the operands by 2 due to our representation
                            instrs.append(&mut vec![
                                Instr::ISar(Arg::Reg(Reg::Rax), Arg::Imm(1)),
                                Instr::IImul(
                                    Arg::Reg(Reg::Rax),
                                    Arg::RegOffset(Reg::Rsp, -stack_offset),
                                ),
                            ]);
                            instrs.append(&mut error_overflow());
                        }
                        // for the comparison operators
                        _ => {
                            instrs.append(&mut vec![
                                // cmp and set the result to rax accordingly
                                // This is the right order for the comparison
                                Instr::ICmp(
                                    Arg::RegOffset(Reg::Rsp, -stack_offset),
                                    Arg::Reg(Reg::Rax),
                                ),
                                Instr::IMov(Arg::Reg(Reg::Rbx), Arg::Imm(repr_true())),
                                Instr::IMov(Arg::Reg(Reg::Rax), Arg::Imm(repr_false())),
                            ]);
                            match op {
                                Op2::Less => instrs
                                    .push(Instr::ICmovl(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx))),
                                Op2::LessEqual => instrs
                                    .push(Instr::ICmovle(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx))),
                                Op2::Greater => instrs
                                    .push(Instr::ICmovg(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx))),
                                Op2::GreaterEqual => instrs
                                    .push(Instr::ICmovge(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx))),
                                _ => panic!("This should literally never happen"),
                            }; 
                        }
                    } 
                    instrs
                }
            }
        }
        Expr::If(cond, thn, els) => {
            let else_label = new_label(l, "ifelse");
            let end_label = new_label(l, "ifend");
            let mut instrs = compile_expr(cond, si, env, brake, l);
            // TODO: We might want to check that the result is a boolean
            // If result of cond is false, jump to else
            instrs.push(Instr::ICmp(Arg::Reg(Reg::Rax), Arg::Imm(repr_false())));
            instrs.push(Instr::IJe(Arg::Label(else_label.clone())));
            // We execute thn instructions and jump to end
            instrs.append(&mut compile_expr(thn, si, env, brake, l));
            instrs.push(Instr::IJmp(Arg::Label(end_label.clone())));
            // We define the else label, no need to jump after
            instrs.push(Instr::Label(else_label.clone()));
            instrs.append(&mut compile_expr(els, si, env, brake, l));
            instrs.push(Instr::Label(end_label.clone()));
            instrs
        }
        Expr::Loop(body) => {
            let start_label = new_label(l, "loop");
            let end_label = new_label(l, "loopend");
            let mut instrs = vec![Instr::Label(start_label.clone())];
            instrs.append(&mut compile_expr(body, si, env, &end_label, l));
            instrs.push(Instr::IJmp(Arg::Label(start_label.clone())));
            instrs.push(Instr::Label(end_label.clone()));
            instrs
        }
        Expr::Break(e) => {
            if !brake.is_empty() {
                let mut instrs = compile_expr(e, si, env, brake, l);
                instrs.push(Instr::IJmp(Arg::Label(brake.clone())));
                instrs
            } else {
                panic!("break outside of loop: {:?}", e);
            }
        }
        Expr::Set(id, e) => {
            if env.contains_key(id) {
                let id_offset = env.get(id).unwrap();
                let mut instrs = compile_expr(e, si, env, brake, l);
                instrs.push(Instr::IMov(
                    Arg::RegOffset(Reg::Rsp, - *id_offset),
                    Arg::Reg(Reg::Rax),
                ));
                instrs
            } else {
                panic!("Unbound variable identifier {}", id);
            }
        }
        Expr::Block(es) => {
            es.into_iter()
            .map(|e| compile_expr(e, si, env, brake, l))
            .flatten()
            .collect()
        }
        Expr::Print(e) => {
            let mut instrs = compile_expr(e, si, env, brake, l);
            // We need to use stack offset that is 16 bit aligned
            let index = if si % 2 == 0 { si } else { si + 1 };
            let stack_offset = index * 8;
            instrs.append(&mut vec![
                // Save rdi before the call
                Instr::IMov(Arg::RegOffset(Reg::Rsp, -stack_offset), Arg::Reg(Reg::Rdi)),
                // Set the argument (rdi) to the value we want to print (rax)
                Instr::IMov(Arg::Reg(Reg::Rdi), Arg::Reg(Reg::Rax)),
                // Move rsp to point to the top of the stack
                Instr::ISub(Arg::Reg(Reg::Rsp), Arg::Imm(stack_offset)),
                // Finally, call the print function
                Instr::ICall("snek_print".to_string()),
                // Restore rsp
                Instr::IAdd(Arg::Reg(Reg::Rsp), Arg::Imm(stack_offset)),
                // and restore rdi
                Instr::IMov(Arg::Reg(Reg::Rdi), Arg::RegOffset(Reg::Rsp, -stack_offset)),
            ]);
            instrs
        }
        Expr::Tuple(es) => {
            let size = es.len();
            let mut instrs = vec![];
            for (i, expr) in es.into_iter().enumerate() {
                let current_si = si + i as i64;
                // Compile each member of the tuple
                instrs.append(&mut compile_expr(expr, current_si, env, brake, l));
                // Save the result in the current stack index
                instrs.push(Instr::IMov(
                    Arg::RegOffset(Reg::Rsp, -current_si * 8),
                    Arg::Reg(Reg::Rax),
                ));
            }
            // Set rbs to the size of the tuple
            instrs.push(Instr::IMov(Arg::Reg(Reg::Rbx), Arg::Imm((size << 1) as i64)));
            // Set r15 (next new heap location) to the size of the tuple
            // The size need not to be in the internal representation format (shifted)
            // however, this will make checking out of bounds easier
            instrs.push(Instr::IMov(Arg::RegOffset(Reg::R15, 0), Arg::Reg(Reg::Rbx)));
            // Move values from the stack to the heap
            for i in 0..size {
                let current_si = si + i as i64;
                // Since we cannot move memory to memory, we need to use rax
                // Move the value from the stack to rax
                instrs.push(Instr::IMov(
                    Arg::Reg(Reg::Rax),
                    Arg::RegOffset(Reg::Rsp, -current_si * 8),
                ));
                // Move the value from rax to the heap
                instrs.push(Instr::IMov(
                    Arg::RegOffset(Reg::R15, (i as i64 + 1) * 8),
                    Arg::Reg(Reg::Rax),
                ));
            }
            // Get the address at r15 and save it in rax
            instrs.push(Instr::IMov(Arg::Reg(Reg::Rax), Arg::Reg(Reg::R15)));
            // Add 1 to rax, to represent a tuple
            instrs.push(Instr::IAdd(Arg::Reg(Reg::Rax), Arg::Imm(1)));
            // Add size + 1 of the tuple to r15, since we are also storing the size
            instrs.push(Instr::IAdd(Arg::Reg(Reg::R15), Arg::Imm((size + 1) as i64 * 8)));
            instrs
        }
        Expr::Index(e, idx) => {
            // first evaluate the index
            let mut instrs = compile_expr(idx, si, env, brake, l);
            // error if idx is not a number
            instrs.append(&mut error_rax_not_num());
            // save the result in the current stack index
            instrs.push(Instr::IMov(Arg::RegOffset(Reg::Rsp, -si * 8), Arg::Reg(Reg::Rax)));
            // evaluate the tuple
            instrs.append(&mut compile_expr(e, si + 1, env, brake, l));
            // error if the value is not a tuple
            instrs.append(&mut error_rax_not_tuple());
            // get the actual address by subtracting 1 from rax
            instrs.push(Instr::ISub(Arg::Reg(Reg::Rax), Arg::Imm(1)));
            // TODO: check if the index is out of bounds
            // get the index to rbx
            instrs.push(Instr::IMov(Arg::Reg(Reg::Rbx), Arg::RegOffset(Reg::Rsp, -si * 8)));
            // get the correct index by shifting it to the right and adding 1
            instrs.push(Instr::ISar(Arg::Reg(Reg::Rbx), Arg::Imm(1)));
            instrs.push(Instr::IAdd(Arg::Reg(Reg::Rbx), Arg::Imm(1)));
            // use this index to index the tuple in the heap
            instrs.push(Instr::IMov(Arg::Reg(Reg::Rax), Arg::RegAddressing(Reg::Rax, Reg::Rbx, 8)));
            instrs
        }
        Expr::FunCall(fname, args) => {
            // TODO: Check that the function exists and has the right arity
            let n_args = args.len();
            // After setting up the call, rsp will move by 8 * n_args
            let new_rsp_offset = si + n_args as i64;
            // We actually can compile the arguments using this stack index + 1
            // since those will be untouched, and will make our life easier by
            // avoiding flipping the order of the arguments for the call
            let mut instrs = vec![];
            for (i, arg) in args.iter().enumerate() {
                // compile using an index that is always safe
                instrs.append(&mut compile_expr(arg, new_rsp_offset + 1, env, brake, l));
                // start populating the args from the new rsp offset up
                instrs.push(Instr::IMov(
                    Arg::RegOffset(Reg::Rsp, (i as i64 - new_rsp_offset) * 8),
                    Arg::Reg(Reg::Rax),
                ));
            };
            // Save rdi to the actual stack index
            instrs.push(Instr::IMov(
                Arg::RegOffset(Reg::Rsp, -si * 8),
                Arg::Reg(Reg::Rdi),
            ));
            // Move rsp to point to the top of the stack
            instrs.push(Instr::ISub(Arg::Reg(Reg::Rsp), Arg::Imm(new_rsp_offset * 8)));
            // Finally, call the function
            instrs.push(Instr::ICall(fname.clone()));
            // Restore rsp
            instrs.push(Instr::IAdd(Arg::Reg(Reg::Rsp), Arg::Imm(new_rsp_offset * 8)));
            // and restore rdi
            instrs.push(Instr::IMov(
                Arg::Reg(Reg::Rdi),
                Arg::RegOffset(Reg::Rsp, -si * 8),
            ));
            instrs
        }
        _ => panic!("Unimplemented expression: {:?}", e),
    }
}

// fun_env is a map from function names to their arity
// fn compile_fundef(def: &FunDef, labels: &mut i64, fun_env: &mut HashMap<String, i64>) -> Vec<Instr> {
fn compile_fundef(def: &FunDef, labels: &mut i64) -> Vec<Instr> {
    let body_env: HashMap<String, i64> = def
        .params
        .iter()
        .enumerate()
        .map(|(i, id)| (id.clone(), - (i as i64 + 1) * 8 ))
        .collect();
    let mut body_instrs = compile_expr(&def.body, 2, &body_env, &String::new(), labels);
    let mut instrs = vec![Instr::Label(def.name.clone())];
    instrs.append(&mut body_instrs);
    instrs.push(Instr::IRet);
    instrs
}

fn compile_program(p: &Program) -> (Vec<Instr>, Vec<Instr>) {
    let mut labels = 0;
    let mut defs = vec![];
    for def in &p.defs {
        defs.append(&mut compile_fundef(def, &mut labels));
    }
    let main = compile_expr(&p.main, 2, &HashMap::new(), &String::new(), &mut labels);
    (defs, main)
}

fn instrs_to_asm(instrs: Vec<Instr>) -> String {
    instrs
        .iter()
        .map(|i| instr_to_str(i))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn compile(p: &Program) -> String {
    let prelude = instrs_to_asm(error_handler());

    let (defs_instrs, main_instrs) = compile_program(&p);
    let defs_asm = instrs_to_asm(defs_instrs);
    let main_asm = instrs_to_asm(main_instrs);

    let asm_program = format!(
        "
section .text
global our_code_starts_here
extern snek_error
extern snek_print
{}
{}
our_code_starts_here:
mov r15, rsi
{}
ret
", prelude, defs_asm, main_asm);
    asm_program
}
