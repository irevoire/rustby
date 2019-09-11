use lexer::Token;
use logos::Logos;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut args = env::args();
    if args.len() != 2 {
        println!("usage: cargo test --lexer [filename]");
        return Ok(());
    }
    args.next(); // filename
    let mut f = File::open(args.next().unwrap())?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    let buffer = std::str::from_utf8(&buffer).unwrap();

    let mut lexer = Token::lexer(buffer);
    loop {
        let lex = &lexer.token;

        match lex {
            Token::ParseEnd => return Ok(()),
            Token::ParseError => {
                println!("\nError on {}", lexer.slice());
                return Ok(());
            }
            Token::NewLine => println!(""),
            Token::Comments => println!("{}", lexer.slice()),
            t => print!("{:?} ", t),
        }
        lexer.advance();
    }
}
