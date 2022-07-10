mod parser;
mod exe_writer;
mod x86;

use std::{fs::File, io::Read};
use exe_writer::ExeWriter;
use parser::{lexer::Lexer, parser::Parser};

use parser::ast_visitor::DefaultAstVisitor;

fn main() {
    let mut writer = ExeWriter::new();

    writer.write();

    let mut code = String::new();    
    let mut file = File::open("examples/let_statement.dl").unwrap();

    file.read_to_string(&mut code).unwrap();

    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();

    for token in tokens.iter() {
        println!("token: {:?}", token);
    }

    println!();

    let mut parser = Parser::new(tokens);
    let parse_result = parser.parse();
    let ref mut ast = parse_result.unwrap();

    println!("{:?}", ast);

    // ast visitor

    let ast_visitor = DefaultAstVisitor::<(), ()>::new();

    ast.accept(ast_visitor).unwrap();
}
