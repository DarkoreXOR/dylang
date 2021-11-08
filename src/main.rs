mod token;
mod lexer;
mod parser;
mod exe_writer;
mod codegen;
mod x86;

use std::{fs::File, io::Read};

use exe_writer::ExeWriter;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let mut writer = ExeWriter::new();

    writer.write();

    let mut code = String::new();
    
    let mut file = File::open("test.dl").unwrap();
    file.read_to_string(&mut code).unwrap();

    let mut lexer = Lexer::new(code.into());

    let tokens = lexer.tokenize();

    for token in tokens.iter() {
        println!("token: {:?}", token);
    }

    println!();

    let mut parser = Parser::new(tokens);

    let ast = parser.parse();

    println!("{:?}", ast.unwrap());
}
