use im::HashMap;

use crate::syntax::*;
use crate::asm::*;

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
        Instr::Mov(Arg::Reg(Reg::Rdi), Arg::Reg(Reg::Rbx)),
        // TODO: is this call or jump?
        Instr::Call("snek_error".to_string()),
    ]
}

// Instructions that error with code 1 if the values in RAX and RCX are of different types
// Not using RBX since we are using that to store the error code for the error handler
fn error_rax_rcx_diff_type() -> Vec<Instr> {
    vec![
        // Set error code to 1
        Instr::Mov(Arg::Reg(Reg::Rbx), Arg::Imm(1)),
        // Get the matching bits of RAX and RCX
        Instr::Xor(Arg::Reg(Reg::Rcx), Arg::Reg(Reg::Rax)),
        // and test if 1 bit is set (1st bits are equal)
        Instr::Test(Arg::Reg(Reg::Rcx), Arg::Imm(1)),
        // If not, jump to error handler
        Instr::Jne("snek_error_handler".to_string()),
    ]
}

// Instructions that error with code 2 if the value in RAX is not a number
fn error_rax_not_num() -> Vec<Instr> {
    vec![
        Instr::Mov(Arg::Reg(Reg::Rbx), Arg::Imm(2)),
        Instr::Test(Arg::Reg(Reg::Rax), Arg::Imm(1)),
        Instr::Jne("snek_error_handler".to_string()),
    ]
}

// Instructions that error with code 3 if there is an overflow
fn error_overflow() -> Vec<Instr> {
    vec![
        Instr::Mov(Arg::Reg(Reg::Rbx), Arg::Imm(3)),
        Instr::Jo("snek_error_handler".to_string()),
    ]
}
// Instructions that error with code 4 if the value in RAX is not a tuple
fn error_rax_not_tuple() -> Vec<Instr> {
    vec![
        // Copy the value to rcx
        Instr::Mov(Arg::Reg(Reg::Rcx), Arg::Reg(Reg::Rax)),
        // get the two least significant bits
        Instr::And(Arg::Reg(Reg::Rcx), Arg::Imm(3)),
        // and test if they are 1 (the value is a tuple)
        Instr::Cmp(Arg::Reg(Reg::Rcx), Arg::Imm(1)),
        // If not, jump to error handler with code 4
        Instr::Mov(Arg::Reg(Reg::Rbx), Arg::Imm(4)),
        Instr::Jne("snek_error_handler".to_string()),
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
                vec![Instr::Mov(Arg::Reg(Reg::Rax), Arg::Imm(our_n))]
            } else {
                panic!("Invalid integer constant: overflow {}", n)
            }
        }
        Expr::Boolean(true) => vec![Instr::Mov(Arg::Reg(Reg::Rax), Arg::Imm(repr_true()))],
        Expr::Boolean(false) => vec![Instr::Mov(Arg::Reg(Reg::Rax), Arg::Imm(repr_false()))],
        Expr::Var(id) if id == "input" => vec![Instr::Mov(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rdi))],
        Expr::Var(id) => {
            if env.contains_key(id) {
                vec![Instr::Mov(
                    Arg::Reg(Reg::Rax),
                    Arg::Mem(maddr_bd(Reg::Rsp, -*env.get(id).unwrap())),
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
                instrs.push(Instr::Mov(
                    Arg::Mem(maddr_bd(Reg::Rsp, -stack_offset)),
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
                    instrs.push(Instr::Add(Arg::Reg(Reg::Rax), Arg::Imm(2)));
                    instrs.append(&mut error_overflow());
                }
                Op1::Sub1 => {
                    instrs.append(&mut error_rax_not_num());
                    instrs.push(Instr::Sub(Arg::Reg(Reg::Rax), Arg::Imm(2)));
                    instrs.append(&mut error_overflow());
                }
                Op1::IsNum => {
                    let mut comp_instrs = vec![
                        // Test rax & 1, this will be 0 iff rax represents a number
                        Instr::Test(Arg::Reg(Reg::Rax), Arg::Imm(1)),
                        Instr::Mov(Arg::Reg(Reg::Rax), Arg::Imm(repr_false())),
                        Instr::Mov(Arg::Reg(Reg::Rbx), Arg::Imm(repr_true())),
                        Instr::Cmove(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx)),
                    ];
                    instrs.append(&mut comp_instrs);
                }
                Op1::IsBool => {
                    let mut comp_instrs = vec![
                        Instr::Test(Arg::Reg(Reg::Rax), Arg::Imm(1)),
                        Instr::Mov(Arg::Reg(Reg::Rax), Arg::Imm(repr_true())),
                        Instr::Mov(Arg::Reg(Reg::Rbx), Arg::Imm(repr_false())),
                        Instr::Cmove(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx)),
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
                    instrs.push(Instr::Mov(
                        Arg::Mem(maddr_bd(Reg::Rsp, -stack_offset)),
                        Arg::Reg(Reg::Rax),
                    ));
                    // Then evaluate e2
                    instrs.append(&mut compile_expr(e2, si + 1, env, brake, l));
                    // Copy the result of e1 to rcx
                    instrs.push(Instr::Mov(
                        Arg::Reg(Reg::Rcx),
                        Arg::Mem(maddr_bd(Reg::Rsp, -stack_offset)),
                    ));
                    // Error if e1 and e2 are of different types
                    instrs.append(&mut error_rax_rcx_diff_type());
                    // If they are of the same type, do a regular comparison
                    // and set the result accordingly
                    instrs.append(&mut vec![
                        Instr::Cmp(Arg::Reg(Reg::Rax), Arg::Mem(maddr_bd(Reg::Rsp, -stack_offset))),
                        Instr::Mov(Arg::Reg(Reg::Rax), Arg::Imm(repr_false())),
                        Instr::Mov(Arg::Reg(Reg::Rbx), Arg::Imm(repr_true())),
                        Instr::Cmove(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx)),
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
                    instrs.push(Instr::Mov(
                        Arg::Mem(maddr_bd(Reg::Rsp, -stack_offset)), 
                        Arg::Reg(Reg::Rax),
                    ));
                    // Evaluate e2
                    instrs.append(&mut compile_expr(e2, si + 1, env, brake, l));
                    // error if the result is not a number
                    instrs.append(&mut error_rax_not_num());
                    // the remaining instructions depend on the operator
                    match op {
                        Op2::Plus => {
                            instrs.push(Instr::Add(
                                Arg::Reg(Reg::Rax),
                                Arg::Mem(maddr_bd(Reg::Rsp, -stack_offset)),
                            ));
                            instrs.append(&mut error_overflow());
                        }
                        Op2::Minus => {
                            // The expected order is the opposite than the sub instruction
                            instrs.push(Instr::Sub(
                                Arg::Mem(maddr_bd(Reg::Rsp, -stack_offset)),
                                Arg::Reg(Reg::Rax),
                            ));
                            instrs.append(&mut error_overflow());
                            instrs.push(Instr::Mov(
                                Arg::Reg(Reg::Rax),
                                Arg::Mem(maddr_bd(Reg::Rsp, -stack_offset)),
                            ));
                        }
                        Op2::Times => {
                            // We need to divide one of the operands by 2 due to our representation
                            instrs.append(&mut vec![
                                Instr::Sar(Arg::Reg(Reg::Rax), Arg::Imm(1)),
                                Instr::Imul(
                                    Arg::Reg(Reg::Rax),
                                    Arg::Mem(maddr_bd(Reg::Rsp, -stack_offset)),
                                ),
                            ]);
                            instrs.append(&mut error_overflow());
                        }
                        // for the comparison operators
                        _ => {
                            instrs.append(&mut vec![
                                // cmp and set the result to rax accordingly
                                // This is the right order for the comparison
                                Instr::Cmp(
                                    Arg::Mem(maddr_bd(Reg::Rsp, -stack_offset)),
                                    Arg::Reg(Reg::Rax),
                                ),
                                Instr::Mov(Arg::Reg(Reg::Rbx), Arg::Imm(repr_true())),
                                Instr::Mov(Arg::Reg(Reg::Rax), Arg::Imm(repr_false())),
                            ]);
                            match op {
                                Op2::Less => instrs
                                    .push(Instr::Cmovl(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx))),
                                Op2::LessEqual => instrs
                                    .push(Instr::Cmovle(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx))),
                                Op2::Greater => instrs
                                    .push(Instr::Cmovg(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx))),
                                Op2::GreaterEqual => instrs
                                    .push(Instr::Cmovge(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx))),
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
            instrs.push(Instr::Cmp(Arg::Reg(Reg::Rax), Arg::Imm(repr_false())));
            instrs.push(Instr::Je(else_label.clone()));
            // We execute thn instructions and jump to end
            instrs.append(&mut compile_expr(thn, si, env, brake, l));
            instrs.push(Instr::Jmp(end_label.clone()));
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
            instrs.push(Instr::Jmp(start_label.clone()));
            instrs.push(Instr::Label(end_label.clone()));
            instrs
        }
        Expr::Break(e) => {
            if !brake.is_empty() {
                let mut instrs = compile_expr(e, si, env, brake, l);
                instrs.push(Instr::Jmp(brake.clone()));
                instrs
            } else {
                panic!("break outside of loop: {:?}", e);
            }
        }
        Expr::Set(id, e) => {
            if env.contains_key(id) {
                let id_offset = env.get(id).unwrap();
                let mut instrs = compile_expr(e, si, env, brake, l);
                instrs.push(Instr::Mov(
                    Arg::Mem(maddr_bd(Reg::Rsp, -*id_offset)),
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
                Instr::Mov(Arg::Mem(maddr_bd(Reg::Rsp, -stack_offset)), Arg::Reg(Reg::Rdi)),
                // Set the argument (rdi) to the value we want to print (rax)
                Instr::Mov(Arg::Reg(Reg::Rdi), Arg::Reg(Reg::Rax)),
                // Move rsp to point to the top of the stack
                Instr::Sub(Arg::Reg(Reg::Rsp), Arg::Imm(stack_offset)),
                // Finally, call the print function
                Instr::Call("snek_print".to_string()),
                // Restore rsp
                Instr::Add(Arg::Reg(Reg::Rsp), Arg::Imm(stack_offset)),
                // and restore rdi
                Instr::Mov(Arg::Reg(Reg::Rdi), Arg::Mem(maddr_bd(Reg::Rsp, -stack_offset))),
            ]);
            instrs
        }
        Expr::Tup(es) => {
            let size = es.len();
            let mut instrs = vec![];
            for (i, expr) in es.into_iter().enumerate() {
                let current_si = si + i as i64;
                // Compile each member of the tuple
                instrs.append(&mut compile_expr(expr, current_si, env, brake, l));
                // Save the result in the current stack index
                instrs.push(Instr::Mov(
                    // Arg::RegOffset(Reg::Rsp, -current_si * 8),
                    Arg::Mem(maddr_bd(Reg::Rsp, -current_si * 8)),
                    Arg::Reg(Reg::Rax),
                ));
            }
            // Set rbs to the size of the tuple
            instrs.push(Instr::Mov(Arg::Reg(Reg::Rbx), Arg::Imm((size << 1) as i64)));
            // Set r15 (next new heap location) to the size of the tuple
            // The size need not to be in the internal representation format (shifted)
            // however, this will make checking out of bounds easier
            instrs.push(Instr::Mov(Arg::Mem(maddr_b(Reg::R15)), Arg::Reg(Reg::Rbx)));
            // Move values from the stack to the heap
            for i in 0..size {
                let current_si = si + i as i64;
                // Since we cannot move memory to memory, we need to use rax
                // Move the value from the stack to rax
                instrs.push(Instr::Mov(
                    Arg::Reg(Reg::Rax),
                    Arg::Mem(maddr_bd(Reg::Rsp, -current_si * 8)),
                ));
                // Move the value from rax to the heap
                instrs.push(Instr::Mov(
                    // Arg::RegOffset(Reg::R15, (i as i64 + 1) * 8),
                    Arg::Mem(maddr_bd(Reg::R15, (i as i64 + 1) * 8)),
                    Arg::Reg(Reg::Rax),
                ));
            }
            // Get the address at r15 and save it in rax
            instrs.push(Instr::Mov(Arg::Reg(Reg::Rax), Arg::Reg(Reg::R15)));
            // Add 1 to rax, to represent a tuple
            instrs.push(Instr::Add(Arg::Reg(Reg::Rax), Arg::Imm(1)));
            // Add size + 1 of the tuple to r15, since we are also storing the size
            instrs.push(Instr::Add(Arg::Reg(Reg::R15), Arg::Imm((size + 1) as i64 * 8)));
            instrs
        }
        Expr::TupGet(e, idx) => {
            // first evaluate the index
            let mut instrs = compile_expr(idx, si, env, brake, l);
            // error if idx is not a number
            instrs.append(&mut error_rax_not_num());
            // save the result in the current stack index
            instrs.push(Instr::Mov(
                Arg::Mem(maddr_bd(Reg::Rsp, -si * 8)),
                Arg::Reg(Reg::Rax),
            ));
            // evaluate the tuple
            instrs.append(&mut compile_expr(e, si + 1, env, brake, l));
            // error if the value is not a tuple
            instrs.append(&mut error_rax_not_tuple());
            // get the actual address by subtracting 1 from rax
            instrs.push(Instr::Sub(Arg::Reg(Reg::Rax), Arg::Imm(1)));
            // TODO: check if the index is out of bounds
            // get the index to rbx
            instrs.push(Instr::Mov(
                Arg::Reg(Reg::Rbx),
                Arg::Mem(maddr_bd(Reg::Rsp, -si * 8)),
            ));
            // get the correct index by shifting it to the right and adding 1
            instrs.push(Instr::Sar(Arg::Reg(Reg::Rbx), Arg::Imm(1)));
            instrs.push(Instr::Add(Arg::Reg(Reg::Rbx), Arg::Imm(1)));
            // use this index to index the tuple in the heap
            instrs.push(Instr::Mov(
                Arg::Reg(Reg::Rax),
                Arg::Mem(maddr_bisd(Reg::Rax, Reg::Rbx, 8, 0)),
            ));
            instrs
        }
        Expr::TupSet(t, i, e) => {
            // first evaluate the index
            let mut instrs = compile_expr(i, si, env, brake, l);
            // error if idx is not a number
            instrs.append(&mut error_rax_not_num());
            // save the result in the current stack index
            instrs.push(Instr::Mov(
                Arg::Mem(maddr_bd(Reg::Rsp, -si * 8)),
                Arg::Reg(Reg::Rax),
            ));
            // evaluate e
            instrs.append(&mut compile_expr(e, si + 1, env, brake, l));
            // save the result in the current stack index
            instrs.push(Instr::Mov(
                Arg::Mem(maddr_bd(Reg::Rsp, -(si + 1) * 8)),
                Arg::Reg(Reg::Rax),
            ));
            // evaluate the tuple
            instrs.append(&mut compile_expr(t, si + 2, env, brake, l));
            // error if the value is not a tuple
            instrs.append(&mut error_rax_not_tuple());
            // get the actual address by subtracting 1 from rax
            instrs.push(Instr::Sub(Arg::Reg(Reg::Rax), Arg::Imm(1)));
            // TODO: check if the index is out of bounds
            // get idx to rbx
            instrs.push(Instr::Mov(
                Arg::Reg(Reg::Rbx),
                Arg::Mem(maddr_bd(Reg::Rsp, -si * 8)),
            ));
            // get the correct index by shifting it to the right and adding 1
            instrs.push(Instr::Sar(Arg::Reg(Reg::Rbx), Arg::Imm(1)));
            instrs.push(Instr::Add(Arg::Reg(Reg::Rbx), Arg::Imm(1)));
            // get e to rcx
            instrs.push(Instr::Mov(
                Arg::Reg(Reg::Rcx),
                Arg::Mem(maddr_bd(Reg::Rsp, -(si + 1) * 8)),
            ));
            // move the value of e to rax + 8 * rbx
            instrs.push(Instr::Mov(
                Arg::Mem(maddr_bisd(Reg::Rax, Reg::Rbx, 8, 0)),
                Arg::Reg(Reg::Rcx),
            ));
            // add 1 to rax to return the representation of a tuple
            instrs.push(Instr::Add(Arg::Reg(Reg::Rax), Arg::Imm(1)));
            instrs
        }
        Expr::TupLen(t) => {
            // evaluate the tuple
            let mut instrs = compile_expr(t, si, env, brake, l);
            // error if the value is not a tuple
            instrs.append(&mut error_rax_not_tuple());
            // get the actual address by subtracting 1 from rax
            instrs.push(Instr::Sub(Arg::Reg(Reg::Rax), Arg::Imm(1)));
            // the size of the tuple is stored exactly at the address of the tuple
            instrs.push(Instr::Mov(
                Arg::Reg(Reg::Rax),
                Arg::Mem(maddr_b(Reg::Rax)),
            ));
            instrs
        }
        Expr::Call(fname, args) => {
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
                instrs.push(Instr::Mov(
                    Arg::Mem(maddr_bd(Reg::Rsp, (i as i64 - new_rsp_offset) * 8)),
                    Arg::Reg(Reg::Rax),
                ));
            };
            // Save rdi to the actual stack index
            instrs.push(Instr::Mov(
                Arg::Mem(maddr_bd(Reg::Rsp, -si * 8)),
                Arg::Reg(Reg::Rdi),
            ));
            // Move rsp to point to the top of the stack
            instrs.push(Instr::Sub(Arg::Reg(Reg::Rsp), Arg::Imm(new_rsp_offset * 8)));
            // Finally, call the function
            instrs.push(Instr::Call(fname.clone()));
            // Restore rsp
            instrs.push(Instr::Add(Arg::Reg(Reg::Rsp), Arg::Imm(new_rsp_offset * 8)));
            // and restore rdi
            instrs.push(Instr::Mov(
                Arg::Reg(Reg::Rdi),
                Arg::Mem(maddr_bd(Reg::Rsp, -si * 8)),
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
    instrs.push(Instr::Ret);
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

pub fn compile(p: &Program) -> String {
    let prelude = instrs_to_string(error_handler());

    let (defs_instrs, main_instrs) = compile_program(&p);
    let defs_asm = instrs_to_string(defs_instrs);
    let main_asm = instrs_to_string(main_instrs);

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
