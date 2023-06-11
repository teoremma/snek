use im::HashMap;

use crate::types::*;

fn reg_to_str(r: &Reg) -> String {
    match r {
        Reg::RAX => "rax".to_string(),
        Reg::RBX => "rbx".to_string(),
        Reg::RCX => "rcx".to_string(),
        Reg::RSP => "rsp".to_string(),
        Reg::RDI => "rdi".to_string(),
        Reg::R15 => "r15".to_string(),
    }
}

fn val_to_str(v: &Val) -> String {
    match v {
        Val::Reg(r) => reg_to_str(r),
        Val::Imm(i) => i.to_string(),
        Val::RegOffset(r, i) => {
            if *i == 0 {
                return format!("[{}]", reg_to_str(r));
            } else if *i < 0 {
                return format!("[{} - {}]", reg_to_str(r), -i);
            } else {
                return format!("[{} + {}]", reg_to_str(r), i);
            }
        }
        Val::Label(l) => l.to_string(),
        Val::RegAddressing(base, index, scale) => format!("[{} + {} * {}]", reg_to_str(base), reg_to_str(index), scale),
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
        Instr::IMov(Val::Reg(Reg::RDI), Val::Reg(Reg::RBX)),
        // TODO: is this call or jump?
        Instr::ICall("snek_error".to_string()),
    ]
}

// Instructions that error with code 1 if the values in RAX and RCX are of different types
// Not using RBX since we are using that to store the error code for the error handler
fn error_rax_rcx_diff_type() -> Vec<Instr> {
    vec![
        // Set error code to 1
        Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(1)),
        // Get the matching bits of RAX and RCX
        Instr::IXor(Val::Reg(Reg::RCX), Val::Reg(Reg::RAX)),
        // and test if 1 bit is set (1st bits are equal)
        Instr::ITest(Val::Reg(Reg::RCX), Val::Imm(1)),
        // If not, jump to error handler
        Instr::IJne(Val::Label("snek_error_handler".to_string())),
    ]
}

// Instructions that error with code 2 if the value in RAX is not a number
fn error_rax_not_num() -> Vec<Instr> {
    vec![
        Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(2)),
        Instr::ITest(Val::Reg(Reg::RAX), Val::Imm(1)),
        Instr::IJne(Val::Label("snek_error_handler".to_string())),
    ]
}

// Instructions that error with code 3 if there is an overflow
fn error_overflow() -> Vec<Instr> {
    vec![
        Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(3)),
        Instr::IJo(Val::Label("snek_error_handler".to_string())),
    ]
}
// Instructions that error with code 4 if the value in RAX is not a tuple
fn error_rax_not_tuple() -> Vec<Instr> {
    vec![
        // Copy the value to rcx
        Instr::IMov(Val::Reg(Reg::RCX), Val::Reg(Reg::RAX)),
        // get the two least significant bits
        Instr::IAnd(Val::Reg(Reg::RCX), Val::Imm(3)),
        // and test if they are 1 (the value is a tuple)
        Instr::ICmp(Val::Reg(Reg::RCX), Val::Imm(1)),
        // If not, jump to error handler with code 4
        Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(4)),
        Instr::IJne(Val::Label("snek_error_handler".to_string())),
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
                vec![Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(our_n))]
            } else {
                panic!("Invalid integer constant: overflow {}", n)
            }
        }
        Expr::Boolean(true) => vec![Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(repr_true()))],
        Expr::Boolean(false) => vec![Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(repr_false()))],
        Expr::Id(id) if id == "input" => vec![Instr::IMov(Val::Reg(Reg::RAX), Val::Reg(Reg::RDI))],
        Expr::Id(id) => {
            if env.contains_key(id) {
                vec![Instr::IMov(
                    Val::Reg(Reg::RAX),
                    Val::RegOffset(Reg::RSP, - *env.get(id).unwrap()),
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
                    Val::RegOffset(Reg::RSP, -stack_offset),
                    Val::Reg(Reg::RAX),
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
                    instrs.push(Instr::IAdd(Val::Reg(Reg::RAX), Val::Imm(2)));
                    instrs.append(&mut error_overflow());
                }
                Op1::Sub1 => {
                    instrs.append(&mut error_rax_not_num());
                    instrs.push(Instr::ISub(Val::Reg(Reg::RAX), Val::Imm(2)));
                    instrs.append(&mut error_overflow());
                }
                Op1::IsNum => {
                    let mut comp_instrs = vec![
                        // Test rax & 1, this will be 0 iff rax represents a number
                        Instr::ITest(Val::Reg(Reg::RAX), Val::Imm(1)),
                        Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(repr_false())),
                        Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(repr_true())),
                        Instr::ICmove(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)),
                    ];
                    instrs.append(&mut comp_instrs);
                }
                Op1::IsBool => {
                    let mut comp_instrs = vec![
                        Instr::ITest(Val::Reg(Reg::RAX), Val::Imm(1)),
                        Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(repr_true())),
                        Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(repr_false())),
                        Instr::ICmove(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)),
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
                    instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, -stack_offset), Val::Reg(Reg::RAX)));
                    // Then evaluate e2
                    instrs.append(&mut compile_expr(e2, si + 1, env, brake, l));
                    // Copy the result of e1 to rcx
                    instrs.push(Instr::IMov(Val::Reg(Reg::RCX), Val::RegOffset(Reg::RSP, -stack_offset)));
                    // Error if e1 and e2 are of different types
                    instrs.append(&mut error_rax_rcx_diff_type());
                    // If they are of the same type, do a regular comparison
                    // and set the result accordingly
                    instrs.append(&mut vec![
                        Instr::ICmp(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, -stack_offset)),
                        Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(repr_false())),
                        Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(repr_true())),
                        Instr::ICmove(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)),
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
                    instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, -stack_offset), Val::Reg(Reg::RAX)));
                    // Evaluate e2
                    instrs.append(&mut compile_expr(e2, si + 1, env, brake, l));
                    // error if the result is not a number
                    instrs.append(&mut error_rax_not_num());
                    // the remaining instructions depend on the operator
                    match op {
                        Op2::Plus => {
                            instrs.push(Instr::IAdd(
                                Val::Reg(Reg::RAX),
                                Val::RegOffset(Reg::RSP, -stack_offset),
                            ));
                            instrs.append(&mut error_overflow());
                        }
                        Op2::Minus => {
                            // The expected order is the opposite than the sub instruction
                            instrs.push(Instr::ISub(
                                Val::RegOffset(Reg::RSP, -stack_offset),
                                Val::Reg(Reg::RAX),
                            ));
                            instrs.append(&mut error_overflow());
                            instrs.push(Instr::IMov(
                                Val::Reg(Reg::RAX),
                                Val::RegOffset(Reg::RSP, -stack_offset),
                            ));
                        }
                        Op2::Times => {
                            // We need to divide one of the operands by 2 due to our representation
                            instrs.append(&mut vec![
                                Instr::ISar(Val::Reg(Reg::RAX), Val::Imm(1)),
                                Instr::IImul(
                                    Val::Reg(Reg::RAX),
                                    Val::RegOffset(Reg::RSP, -stack_offset),
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
                                    Val::RegOffset(Reg::RSP, -stack_offset),
                                    Val::Reg(Reg::RAX),
                                ),
                                Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(repr_true())),
                                Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(repr_false())),
                            ]);
                            match op {
                                Op2::Less => instrs
                                    .push(Instr::ICmovl(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX))),
                                Op2::LessEqual => instrs
                                    .push(Instr::ICmovle(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX))),
                                Op2::Greater => instrs
                                    .push(Instr::ICmovg(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX))),
                                Op2::GreaterEqual => instrs
                                    .push(Instr::ICmovge(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX))),
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
            instrs.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(repr_false())));
            instrs.push(Instr::IJe(Val::Label(else_label.clone())));
            // We execute thn instructions and jump to end
            instrs.append(&mut compile_expr(thn, si, env, brake, l));
            instrs.push(Instr::IJmp(Val::Label(end_label.clone())));
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
            instrs.push(Instr::IJmp(Val::Label(start_label.clone())));
            instrs.push(Instr::Label(end_label.clone()));
            instrs
        }
        Expr::Break(e) => {
            if !brake.is_empty() {
                let mut instrs = compile_expr(e, si, env, brake, l);
                instrs.push(Instr::IJmp(Val::Label(brake.clone())));
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
                    Val::RegOffset(Reg::RSP, - *id_offset),
                    Val::Reg(Reg::RAX),
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
                Instr::IMov(Val::RegOffset(Reg::RSP, -stack_offset), Val::Reg(Reg::RDI)),
                // Set the argument (rdi) to the value we want to print (rax)
                Instr::IMov(Val::Reg(Reg::RDI), Val::Reg(Reg::RAX)),
                // Move rsp to point to the top of the stack
                Instr::ISub(Val::Reg(Reg::RSP), Val::Imm(stack_offset)),
                // Finally, call the print function
                Instr::ICall("snek_print".to_string()),
                // Restore rsp
                Instr::IAdd(Val::Reg(Reg::RSP), Val::Imm(stack_offset)),
                // and restore rdi
                Instr::IMov(Val::Reg(Reg::RDI), Val::RegOffset(Reg::RSP, -stack_offset)),
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
                    Val::RegOffset(Reg::RSP, -current_si * 8),
                    Val::Reg(Reg::RAX),
                ));
            }
            // Set rbs to the size of the tuple
            instrs.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm((size << 1) as i64)));
            // Set r15 (next new heap location) to the size of the tuple
            // The size need not to be in the internal representation format (shifted)
            // however, this will make checking out of bounds easier
            instrs.push(Instr::IMov(Val::RegOffset(Reg::R15, 0), Val::Reg(Reg::RBX)));
            // Move values from the stack to the heap
            for i in 0..size {
                let current_si = si + i as i64;
                // Since we cannot move memory to memory, we need to use rax
                // Move the value from the stack to rax
                instrs.push(Instr::IMov(
                    Val::Reg(Reg::RAX),
                    Val::RegOffset(Reg::RSP, -current_si * 8),
                ));
                // Move the value from rax to the heap
                instrs.push(Instr::IMov(
                    Val::RegOffset(Reg::R15, (i as i64 + 1) * 8),
                    Val::Reg(Reg::RAX),
                ));
            }
            // Get the address at r15 and save it in rax
            instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Reg(Reg::R15)));
            // Add 1 to rax, to represent a tuple
            instrs.push(Instr::IAdd(Val::Reg(Reg::RAX), Val::Imm(1)));
            // Add size + 1 of the tuple to r15, since we are also storing the size
            instrs.push(Instr::IAdd(Val::Reg(Reg::R15), Val::Imm((size + 1) as i64 * 8)));
            instrs
        }
        Expr::Index(e, idx) => {
            // first evaluate the index
            let mut instrs = compile_expr(idx, si, env, brake, l);
            // error if idx is not a number
            instrs.append(&mut error_rax_not_num());
            // save the result in the current stack index
            instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, -si * 8), Val::Reg(Reg::RAX)));
            // evaluate the tuple
            instrs.append(&mut compile_expr(e, si + 1, env, brake, l));
            // error if the value is not a tuple
            instrs.append(&mut error_rax_not_tuple());
            // get the actual address by subtracting 1 from rax
            instrs.push(Instr::ISub(Val::Reg(Reg::RAX), Val::Imm(1)));
            // TODO: check if the index is out of bounds
            // get the index to rbx
            instrs.push(Instr::IMov(Val::Reg(Reg::RBX), Val::RegOffset(Reg::RSP, -si * 8)));
            // get the correct index by shifting it to the right and adding 1
            instrs.push(Instr::ISar(Val::Reg(Reg::RBX), Val::Imm(1)));
            instrs.push(Instr::IAdd(Val::Reg(Reg::RBX), Val::Imm(1)));
            // use this index to index the tuple in the heap
            instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::RegAddressing(Reg::RAX, Reg::RBX, 8)));
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
                    Val::RegOffset(Reg::RSP, (i as i64 - new_rsp_offset) * 8),
                    Val::Reg(Reg::RAX),
                ));
            };
            // Save rdi to the actual stack index
            instrs.push(Instr::IMov(
                Val::RegOffset(Reg::RSP, -si * 8),
                Val::Reg(Reg::RDI),
            ));
            // Move rsp to point to the top of the stack
            instrs.push(Instr::ISub(Val::Reg(Reg::RSP), Val::Imm(new_rsp_offset * 8)));
            // Finally, call the function
            instrs.push(Instr::ICall(fname.clone()));
            // Restore rsp
            instrs.push(Instr::IAdd(Val::Reg(Reg::RSP), Val::Imm(new_rsp_offset * 8)));
            // and restore rdi
            instrs.push(Instr::IMov(
                Val::Reg(Reg::RDI),
                Val::RegOffset(Reg::RSP, -si * 8),
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
