use std::io;
use std::io::Read;

fn main() {
    let mut lookahead = [0];
    
    io::stdin().read(&mut lookahead)
        .expect("Read failure");

    let lookahead = lookahead[0] as char;

    if lookahead.is_ascii_digit() {
        println!("movl {}, %eax", lookahead);
    } else {
        println!("Integer expected");
    }
}
