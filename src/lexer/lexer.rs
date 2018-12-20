//! # Lexer
//!
//!

extern crate regex;

use regex::Regex;

pub mod lexer {
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

    enum TokenType {
        WhiteSpace,
        Identifier,
        Number,
    }

    enum LexerTokens {
        WhiteSpace,
        Identifier(str),
        Number(i64),
    }

    fn set_up() -> Vec<TokenDefinition> {
        let token_list: Vec<TokenDefinition> = Vec::new();

        // White spaces
        token_list.add(TokenDefinition::new("\s", WhiteSpace, true));

        token_list
    }

    fn lexer_string(file_string: &str) {
        let temp = set_up();
    }
}
