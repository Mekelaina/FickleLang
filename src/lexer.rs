extern  crate substring;

use substring::Substring;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


const NEWLINE: char = '\n';
const WHITESPACE: char = ' ';
const TAB: char = '\t';

const SYMBOLS: [char; 8]  = [';', '%', '$', '#', '@', '?', '\'', '\"'];


#[derive(Clone, Debug)]
pub enum TokenType {
    NON,
    MainRoutineStart,
    MainRoutineEnd,
    CommentMark,
    CommentBlock,
    LabelMark,
    LabelName,
    DecNumMark,
    DecNumLiteral,
    HexNumMark,
    HexNumLiteral,
    BinNumMark,
    BinNumLiteral,
    Keyword,
    Register,
    StringLiteral,
}

#[derive(Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub value: String,
    pub line: usize,
    /* start: usize,
    end: usize, */
}

impl Token {
    pub fn new() -> Token {
        Token { 
            ttype: (TokenType::NON), 
            value: (String::new()), 
            line: (0),
            /* start: (0),
            end: (0), */
        }
    }

}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn lexer(filename: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let lines = read_lines(filename).expect("Could not read file");
    for (count, line) in lines.into_iter().enumerate() {
        let l = line.unwrap();
        let mut parts = l.trim_start().split(' ');
        let data = l.clone();


        
        if data.contains('"') {
            let local = data.clone();
            let quotes: Vec<_> = local.match_indices("\"").collect();
            tokens.push(Token { 
                ttype: (TokenType::StringLiteral), 
                value: ({
                    let v = local.as_str().substring(quotes.get(0).unwrap().0 + 1, quotes.get(1).unwrap().0).to_string();
                    println!("{}", &v);
                    v
                }), 
                line: (count + 1) })
        }
    }  
    tokens
        
}
