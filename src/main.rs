#[macro_use]
extern  crate lazy_static;

mod lexer;
use std::env::args;


fn main() {
    let mut args = args();
    let in_file = args.nth(1).unwrap();

    /* if let Ok(lines) = lexor::read_lines(in_file) {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    } */

    let tokens = lexer::lexer(&in_file);
    
    println!("{:?}", tokens);
}
