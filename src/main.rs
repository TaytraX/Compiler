use crate::lexer::Lexer;
use crate::parser::Parser;

mod lexer;
mod token;
mod parser;

fn main() {
    let main = "32 / (2 - 23) * 4";

    let mut lexer = Lexer::new(main, "main");
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);

    let ast = parser.parse();

    let answer = ast.eval();
    println!("= {}", answer);
}