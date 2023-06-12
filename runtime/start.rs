use std::{collections::HashSet, env};

type SnekVal = i64;

const TRUE: SnekVal = 7;
const FALSE: SnekVal = 3;
const NIL: SnekVal = 1;

#[link(name = "our_code")]
extern "C" {
    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    // Courtesy of Max New (https://maxsnew.com/teaching/eecs-483-fa22/hw_adder_assignment.html)
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input: u64, memory: *mut u64) -> u64;
}

#[no_mangle]
#[export_name = "\x01snek_error"]
pub extern "C" fn snek_error(errcode: i64) {
    eprint!("error {}: ", errcode);
    // TODO: move error code defs to a separate file
    match errcode {
        1 => eprintln!("invalid argument for ="),
        2 => eprintln!("invalid argument for arithmetic op"),
        3 => eprintln!("overflow"),
        4 => eprintln!("tuple value expected"),
        _ => eprintln!("an error ocurred"),
    }
    std::process::exit(1);
}

#[no_mangle]
#[export_name = "\x01snek_print"]
// TODO: might need to return an u64 for some reason
pub unsafe extern "C" fn snek_print(val: i64) -> i64 {
    println!("{}", snek_str(val, &mut HashSet::new()));
    return val;
}

#[no_mangle]
#[export_name = "\x01snek_str"]
unsafe fn snek_str(val: i64, seen: &mut HashSet<i64>) -> String {
    if val & 1 == 0 { format!("{}", val >> 1)}
    else if val == FALSE { format!("false") }
    else if val == TRUE { format!("true") }
    else if val == NIL { format!("nil") }
    else if seen.contains(&val) { format!("(...)") }
    else if val & 1 == 1 {
        seen.insert(val);
        let mut addr = (val - 1) as *mut u64;
        let size = (addr.read() as usize) >> 1;
        let mut res = format!("(");
        for i in 0..size {
            let elem = addr.add(i + 1).read() as i64;
            res = format!("{}{}", res, snek_str(elem, seen));
            if i < size - 1 { res = format!("{}, ", res); }
        }
        seen.remove(&val);
        format!("{})", res)
    } 
    else { format!("unexpected value: {}", val) }
}

fn parse_input(input: &str) -> u64 {
    if input == "true" { 7 }
    else if input == "false" { 3 }
    else if let Ok(i) = input.parse::<i64>() {
        if i <= (1i64 << 62 - 1) || i >= -(1i64 << 62) { (i as u64) << 1 }
        else { panic!("Invalid input: overflow {}", input) }
    }
    else { panic!("Invalid input: {}", input) }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() == 2 { &args[1] } else { "false" };
    let input = parse_input(&input);

    let mut memory = Vec::<u64>::with_capacity(1000000);
    let buffer: *mut u64 = memory.as_mut_ptr();

    let i: u64 = unsafe { our_code_starts_here(input, buffer) };
    unsafe { snek_print(i as i64) };
}
