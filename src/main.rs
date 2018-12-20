//! # Lexer
//!
//!

extern crate regex;
use regex::Regex;
use regex::Match;

enum TokenType {
    WhiteSpace,
    Identifier,
    Number,
}

struct TokenDefinition {
    regex: Regex,
    token_type: TokenType,
    is_ignored: bool,
}

impl TokenDefinition {
    fn new(regex: &str, token_type: TokenType, is_ignored: bool) -> TokenDefinition {
        TokenDefinition {
            regex: Regex::new(regex).unwrap(),
            token_type,
            is_ignored,
        }
    }
}

fn set_up() -> Vec<TokenDefinition> {
    let mut token_list: Vec<TokenDefinition> = Vec::new();

    // White spaces
    token_list.push(TokenDefinition::new(r"[ \t\n\r]*", TokenType::WhiteSpace, true));

    // Number
    token_list.push(TokenDefinition::new(r"[1-9][0-9]*", TokenType::Number, false));

    // Identifier
    token_list.push(TokenDefinition::new(r"[a-zA-Z][a-zA-Z0-9]*", TokenType::Identifier, false));

    token_list
}

fn lexer_string(file_string: &str) -> Option<Vec<(TokenDefinition, &str)>> {
    let mut program_lex: Vec<(TokenDefinition, &str)> = Vec::new();

    let token_list: Vec<TokenDefinition> = set_up();
    let mut index: usize = 0;

    // While the index into the program string is less than the length of the program string
    while index < file_string.len() {
        // Slice the file string to get the working part, which will be from the index of the current
        // position of the program to the end of the string
        let working_file_string: &str = &file_string[index..];

        // This stores the matched token at the 'index' position of the program string.
        let mut matched_token_definition: &TokenDefinition;

        // This stores the length of the match
        let mut matched_len: usize = 0;

        let string_match: &str;

        // For each token to be matched against the program string, get the match that starts at the 'index'
        // position of the program string
        for token in &token_list {
            let matched: Option<Match> = token.regex.find(working_file_string);
            match matched {
                Some(matched_item) => if matched_item.start() - index == 0 && matched_item.end() - matched_item.start() > matched_len {
                    matched_token_definition = token;
                    matched_len = matched_item.end() - matched_item.start();

                    // Get the sub string that was matched
                    string_match = matched_item.as_str()
                },
                // Include the position of the program string in the print
                None => {
                    println!("Unrecognised token, exiting");
                    None
                }
            }
        }

        if matched_token_definition.is_ignored {
            program_lex.push((matched_token_definition.token_type, string_match))
        }

    }
}

fn main() {
    lexer_string("123");
    println!("Hello, world!");
}
