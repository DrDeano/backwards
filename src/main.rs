//! # Backwards
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

fn slice_at_new_line(string: &str) -> &str {
    // Convert the string into a byte array
    let bytes = string.as_bytes();
    
    // For each byte in the string, check if it is a new line. If so, return the slice of the
    // string, else, return the whole string
    for (i, &item) in bytes.iter().enumerate() {
        if item == b'\n' {
            return &string[0..i];
        }
    }
    &string[..]
}

fn lexer_error(previous_token: Token, current_string: &str) -> &str {
    slice_at_new_line(current_string)
}

/// A set up function for creating all the tokens with there regex's and enum types and whether
/// the token is ignored.
fn set_up_lexer() -> Vec<TokenDefinition> {
    println!("Setting up the regex's");
    let mut token_list: Vec<TokenDefinition> = Vec::new();
    
    // White spaces
    token_list.push(TokenDefinition::new(r"[ \t\n\r\f\v]+", TokenType::WhiteSpace, false));
    
    // Number
    token_list.push(TokenDefinition::new(r"\-?[1-9][0-9]*", TokenType::Number, false));
    
    // Identifier
    token_list.push(TokenDefinition::new(r"[a-zA-Z][a-zA-Z0-9]*", TokenType::Identifier, false));
    
    // Binary operator
    token_list.push(TokenDefinition::new(r"[\+|\-|/|\*|<|>|<=|>=|=|!=]",
                                         TokenType::BinOperator, false));
    
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

/// Takes a program string and covert it into tokens that the parser will used
fn lexer_string(file_string: &str) -> Vec<Token> {
    // This stores the lex tokens for the program that will then be piped into the parser
    let mut program_lex: Vec<Token> = Vec::new();
    
    // This is the list of available tokens that are valid for the backwards programming language
    let token_list: Vec<TokenDefinition> = set_up_lexer();
    
    println!("TokenDefinition: {:?}", token_list);
    
    // The current index into the program string that the lexer is working at
    let mut index: usize = 0;
    
    // While the index into the program string is less than the length of the program string
    while index < file_string.len() {
        // Slice the file string to get the working part, which will be from the index of the
        // current position of the program to the end of the string
        let working_file_string: &str = &file_string[index..];
        
        println!("Working with: {}", working_file_string);
        
        // Whether there was a previous match before so can check for user mistakes or no match has
        // been found
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
                if matched_item.start() == 0 {
                    println!("matched item: {:?} with token: {:?}", matched_item, token);
                    
                    if has_matched {
                        // There has been a previous match for this string, maybe the user as used
                        // a reserved word like 'int' for a identifier
                        // Maybe check what mistake the user made in the parser
                        // TODO Have a function that takes the previous match and the working
                        // TODO string and produce a string that explains what is wrong
                        panic!("Have already got a match at index: {}, string: {}\nExiting", index,
                               lexer_error(program_lex.get(program_lex.len()), working_file_string));
                    }
                    
                    has_matched = true;
                    
                    // If the token match has it's 'ignored' bool set, then continue. Else add it
                    // to the program_lex list
                    if token.ignored {
                        continue;
                    } else {
                        program_lex.push(Token::new(token.token_type.clone(),
                                                    matched_item.as_str(), index));
                    }
                    
                    // Advance the index
                    // matched_item.start() is 0, so end() will be the total length
                    index += matched_item.end();
                    
                    // Continue the loop to check for other possible matches as the user may have
                    // used a reserved keyword for a identifier
                }
            }
        }
        
        // If there was no match, then there was an invalid token at the start of the working
        // program string
        if has_matched == false {
            panic!("Unable to match at index: {}, string: '{}'", index,
                   lexer_error(program_lex.get(program_lex.len()), working_file_string));
        }
    }
    
    return program_lex;
}

/// Take the program lex from the lexer and parse it using the backwards grammar.
fn parse_program(program_lex: Vev<Token>) {

}

/// Take the parsed program and interpreted/run it
//fn interpreted_program(program_parse) {

//}

fn main() {
    // Get the argument for the program
    let args: Vec<String> = env::args().collect();
    
    // Make sure a argument was passed to the program
    if args.len() < 2 {
        println!("Please enter the program file");
        return;
    }
    
    // Get the argument passed to the program. Should be the file to open and parse
    let file_name: &str = &args[1];
    
    println!("Input file: {}", file_name);
    
    // Try to open and read the file, else fail
    let program_string: String = fs::read_to_string(file_name).expect("Unable to read file");
    
    // Lex the program
    let program_lex: Vec<Token> = lexer_string(&program_string);
    for lex in &program_lex {
        println!("{:?}", lex);
    }
    
    // Parse the program
    let program_parsed = parse_program(program_lex);
    for parse in program_parsed {
        println!("{:?}", parse);
    }
}
