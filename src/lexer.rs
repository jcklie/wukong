use crate::token::{self, Token};

struct Lexer {
    input: Vec<char>,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: Option<char>,     // current char under examination
}

impl Lexer {
    fn new<S: AsRef<str>>(input: S) -> Self {
        let mut lexer = Lexer {
            input: input.as_ref().chars().collect(),
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        // We are at the end of input
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input[self.read_position]);
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Option<token::Token> {
        if self.ch.is_none() {
            return None;
        }

        let ch = self.ch.unwrap().to_string();

        let token = match ch.as_str() {
            "=" => token::Token {
                kind: token::TokenKind::ASSIGN,
                literal: ch,
            },
            ";" => token::Token {
                kind: token::TokenKind::SEMICOLON,
                literal: ch,
            },
            "(" => token::Token {
                kind: token::TokenKind::LPAREN,
                literal: ch,
            },
            ")" => token::Token {
                kind: token::TokenKind::RPAREN,
                literal: ch,
            },
            "," => token::Token {
                kind: token::TokenKind::COMMA,
                literal: ch,
            },
            "+" => token::Token {
                kind: token::TokenKind::PLUS,
                literal: ch,
            },
            "{" => token::Token {
                kind: token::TokenKind::LBRACE,
                literal: ch,
            },
            "}" => token::Token {
                kind: token::TokenKind::RBRACE,
                literal: ch,
            },
            _ => unimplemented!(),
        };

        self.read_char();
        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token;

    #[test]
    fn test_next_token() {
        let expected_values = vec![
            (token::TokenKind::ASSIGN, "="),
            (token::TokenKind::PLUS, "+"),
            (token::TokenKind::LPAREN, "("),
            (token::TokenKind::RPAREN, ")"),
            (token::TokenKind::LBRACE, "{"),
            (token::TokenKind::RBRACE, "}"),
            (token::TokenKind::COMMA, ","),
            (token::TokenKind::SEMICOLON, ";"),
        ];

        let input = "=+(){},;";
        let mut lexer = Lexer::new(input);

        for (expected_kind, expected_literal) in expected_values.into_iter() {
            let token = lexer.next_token().expect("Has token");

            assert_eq!(expected_kind, token.kind);
            assert_eq!(expected_literal, token.literal);
        }

        assert_eq!(None, lexer.next_token());
    }
}
