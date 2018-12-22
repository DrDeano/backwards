//! # Lexer
//!

extern crate regex;
use regex::Regex;
use regex::Match;
use std::env;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenType {
    WhiteSpace,
    Identifier,
    Number,
    BinOperator,
    Assignment,
    SemiColon,
    Colon,
    Bar,
    BasicType,
    Return,
    Print,
    OpenRoundBracket,
    CloseRoundBracket,
    OpenCurlyBracket,
    CloseCurlyBracket,
    OpenSquareBracket,
    CloseSquareBracket,
}

#[derive(Debug)]
struct TokenDefinition {
    regex: Regex,
    token_type: TokenType,
    is_ignored: bool,
}

/*#[derive(Debug)]
struct TokenPosition {
    column: usize,
    index: usize,
    line: usize,
}*/

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    token_value: String,
    token_position: usize,
}

impl TokenDefinition {
    fn new(regex_str: &str, token_type: TokenType, is_ignored: bool) -> TokenDefinition {
        TokenDefinition {
            regex: Regex::new(regex_str).unwrap(),
            token_type,
            is_ignored,
        }
    }
}

/*impl TokenPosition {
    fn new(column: usize, index: usize, line: usize) -> TokenPosition {
        TokenPosition {
            column,
            index,
            line,
        }
    }
}*/

impl Token {
    fn new(token_type: TokenType, token_value: &str, token_position: usize) -> Token {
        Token {
            token_type,
            token_value: String::from(token_value),
            token_position,
        }
    }
}

fn set_up() -> Vec<TokenDefinition> {
    println!("Setting up the regex's");
    let mut token_list: Vec<TokenDefinition> = Vec::new();
    
    // White spaces
    token_list.push(TokenDefinition::new(r"[ \t\n\r\f\v]+", TokenType::WhiteSpace, false));
    
    // Number
    token_list.push(TokenDefinition::new(r"[1-9][0-9]*", TokenType::Number, false));
    
    // Identifier
    token_list.push(TokenDefinition::new(r"[a-zA-Z][a-zA-Z0-9]*", TokenType::Identifier, false));
    
    // Binary operator
    token_list.push(TokenDefinition::new(r"[\+|\-|/|\*|<|>|<=|>=|=|!=]", TokenType::BinOperator, false));
    
    // Assignment operator
    token_list.push(TokenDefinition::new(r":=", TokenType::Assignment, false));
    
    // Semi colon
    token_list.push(TokenDefinition::new(r";", TokenType::SemiColon, false));
    
    // Colon
    token_list.push(TokenDefinition::new(r":", TokenType::Colon, false));
    
    // Bar, for function arguments
    token_list.push(TokenDefinition::new(r"\|", TokenType::Bar, false));
    
    // Bar, for function arguments
    token_list.push(TokenDefinition::new(r"[int|bool|string|char|real]",
                                         TokenType::BasicType, false));
    
    // Return
    token_list.push(TokenDefinition::new(r"return",  TokenType::Return, false));
    
    // Print
    token_list.push(TokenDefinition::new(r"print",  TokenType::Print, false));
    
    // OpenRoundBracket
    token_list.push(TokenDefinition::new(r"\(",  TokenType::OpenRoundBracket, false));
    
    // CloseRoundBracket
    token_list.push(TokenDefinition::new(r"\)",  TokenType::CloseRoundBracket, false));
    
    // OpenCurlyBracket
    token_list.push(TokenDefinition::new(r"\{",  TokenType::OpenCurlyBracket, false));
    
    // CloseCurlyBracket
    token_list.push(TokenDefinition::new(r"\}",  TokenType::CloseCurlyBracket, false));
    
    // OpenSquareBracket
    token_list.push(TokenDefinition::new(r"\[",  TokenType::OpenSquareBracket, false));
    
    // CloseSquareBracket
    token_list.push(TokenDefinition::new(r"\]",  TokenType::CloseSquareBracket, false));
    
    token_list
}

fn lexer_string(file_string: &str) -> Vec<Token> {
    let mut program_lex: Vec<Token> = Vec::new();
    
    let token_list: Vec<TokenDefinition> = set_up();
    
    println!("TokenDefinition: {:?}", token_list);
    
    let mut index: usize = 0;
    
    // While the index into the program string is less than the length of the program string
    while index < file_string.len() {
        // Slice the file string to get the working part, which will be from the index of the
        // current position of the program to the end of the string
        let working_file_string: &str = &file_string[index..];
        
        println!("Working with: {}", working_file_string);
        
        // This stores the matched token at the 'index' position of the program string.
        // let mut matched_token_definition: Option<&TokenDefinition> = None;
        
        // This stores the length of the match
        // let mut matched_len: usize = 0;
        
        let mut has_matched: bool = false;
        
        // For each token to be matched against the program string, get the match that starts at
        // the 'index' position of the program string
        for token in &token_list {
            // Run the regex over the working program string
            let matched: Option<Match> = token.regex.find(working_file_string);
            
            // If there was a match, then get the information out
            if matched.is_some() {
                // Get the information out the Option enum
                let matched_item: Match = matched.unwrap();
                
                // Make sure that the match is at the beginning of the working program string
                // Also make sure that the match is the longest match possible from the list of
                // possible tokens | This has been removed for now
                // If all are true, then create a token to be added to the list of lexer tokens
                
                // For finding best match, make a list of all matches from matched_item.start() == 0
                // and return the one with the longest matched string
                if matched_item.start() == 0 /*&& matched_item.end() - matched_item.start() > matched_len*/ {
                    println!("matched item: {:?} with token: {:?}", matched_item, token);
                    has_matched = true;
                    
                    // Add the token
                    program_lex.push(Token::new(token.token_type.clone(), matched_item.as_str(), index));
                    
                    // Advance the index
                    index += matched_item.end() - matched_item.start();
                    
                    println!("index: {}", index);
                    
                    // Can exit the for loop as have a match
                    break;
                }
            }
        }
        
        if has_matched == false {
            panic!("Unable to match at index: {}, string: '{}'", index, working_file_string);
        }
    }
    return program_lex;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Please enter the program file");
        return;
    }
    
    let file_name: &str = &args[1];
    
    println!("Input file: {}", file_name);
    
    let program_string: String = fs::read_to_string(file_name).expect("Unable to read file");
    
    let program_lex: Vec<Token> = lexer_string(&program_string);
    for lex in &program_lex {
        println!("{:?}", lex);
    }
}
