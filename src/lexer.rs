extern  crate substring;
extern  crate regex; 

use substring::Substring;
use regex::{Regex, RegexSet};
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
    PercentDecimal,
    DollarHex,
    HashtagBinary,
    SemicolonComment,
    AtLabel,
    AmpersandVariable,
    Colon,
    Comma,
    QuestionArgument,
    Register,
    StringLiteral,
    CharLiteral,
    Number,
    Word
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn lexer(filename: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let set = RegexSet::new(&[
        r"[%]", //0: decimal mark
        r"[\$]", //1: hex mark
        r"[\#]", //2: binary mark
        r"[;]", //3: comment mark
        r"[@]", //4: label mark
        r"[\&]", //5: var ref mark
        r"[:]", //6: colon
        r"[,]", //7: comma
        r"[\?]", //8: arg mark
        r"\b(s0|s1|b0|b1|w0|w1|h0|h1|f0|f1|a0|a1|x|y|c|i|o)\b", //9: registers
        r#"""#, //10: string finder
        r#"'"#, //11: char finder
        r#"[0-9a-fA-F.]+"#, //12: number finder
        r#"[a-zA-Z_0-9]+"# //13: word finder
    ]).unwrap();

    let regexes: Vec<_> = set.patterns().iter()
    .map(|pat| Regex::new(pat).unwrap())
    .collect();

    let lines = read_lines(filename).expect("Could not read file");
    for (count, line) in lines.into_iter().enumerate() {
        let l = line.unwrap();
        let matches_loc: Vec<usize> = set.matches(&l).into_iter().collect();
        let matches_val: Vec<&str> = set.matches(&l).into_iter()
            .map(|match_idx| &regexes[match_idx])
            .map(|pat| pat.find(&l).unwrap().as_str())
            .collect();
        
        for result in matches_loc.into_iter() {
            for value in matches_val.to_owned().into_iter() {
                println!("{}: {}", result, value);
            }    
        
        }

    }  
    tokens
        
}


