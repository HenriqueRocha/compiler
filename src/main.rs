use std::io;
use std::io::Read;
use std::process;

static mut LOOKAHEAD : char = '\0';

unsafe fn expression() {
    term();
    while LOOKAHEAD == '+' || LOOKAHEAD == '-' {
        emitln("push %rax");
        match LOOKAHEAD {
            '+' => add(),
            '-' => subtract(),
            _ => expected("Addop"),
        }
    }
}

unsafe fn add() {
    match_char('+');
    term();
    emitln("pop %rbx");
    emitln("add %rbx, %rax")
}

unsafe fn subtract() {
    match_char('-');
    term();
    emitln("pop %rbx");
    emitln("sub %rbx, %rax");
    emitln("neg %rax");
}

unsafe fn term() {
    emitln(&format!("mov ${}, %rax", get_digit()));
}

unsafe fn next_char() {
    let mut lookahead_str = [0];

    io::stdin().read(&mut lookahead_str)
        .expect("Read failure");

    LOOKAHEAD = lookahead_str[0] as char;
}

fn expected(s: &str) {
    println!("Error: {} expected.", s);
    process::exit(1);
}

unsafe fn match_char(c: char) {
    if LOOKAHEAD == c {
        next_char();
    } else {
        expected(&c.to_string());
    }
}

unsafe fn get_digit() -> char {
    if !LOOKAHEAD.is_ascii_digit() {
        expected("Integer");
    }
    let digit = LOOKAHEAD;
    next_char();
    return digit;
}

fn emitln(s: &str) {
    println!("    {}", s);
}

fn main() {
    unsafe {
        next_char();
        
        println!(".global _start");
        println!();
        println!(".text");
        println!("_start:");

        expression();

        emitln("");
        emitln("mov $1, %eax");
        emitln("mov $0, %ebx");
        emitln("int $0x80");
    }
}
