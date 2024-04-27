use std::{fs::File, io::{BufReader, Read}};

use lexer::Lexer;

use crate::lexer::TokenType;


mod lexer;
mod parser;

fn main() {
    let file = File::open("./examples/not.idle").unwrap();
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content).unwrap();
    let mut lexer = Lexer::new(content);
    loop { 
        let token = lexer.next_token();
        if token.ttype == TokenType::Empty {
            break;
        }
        println!("{token:?}");
    }
}
