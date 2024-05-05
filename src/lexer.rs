use std::vec::Vec;
use std::fmt::Debug;

const KEYWORDS: [&str; 3] = [
    "var", "func", "return",
];

pub enum Token {

    /* Literals like "var", or "func". */
    Keyword(String),

    /* Literals like "var_1", or "add_num". */
    Identifier(String),

    /* Symbol "=" */
    Assign,

    /* Literals like "47", or "-23". */
    Number(isize),

    LeftRoundBracket,
    RightRoundBracket,

    LeftCurlyBracket,
    RightCurlyBracket,

    /* Symbols ":". */
    VariableTypeIndicator,

    /* Symbols "->". */
    ReturnTypeIndicator,
 
    Add,
    Minus,
    Times,
    Divide,

    EndOfStatement,

    EndOfProgram,
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Keyword(text) => write!(f, "KEYWORD \"{}\"", text),
            Token::Identifier(text) => write!(f, "IDENTIFIER \"{}\"", text),
            Token::Assign => write!(f, "="),
            Token::Number(num) => write!(f, "{}", num),
            Token::LeftRoundBracket => write!(f, "("),
            Token::RightRoundBracket => write!(f, ")"),
            Token::LeftCurlyBracket => write!(f, "{{"),
            Token::RightCurlyBracket => write!(f, "}}"),
            Token::VariableTypeIndicator => write!(f, ":"),
            Token::ReturnTypeIndicator => write!(f, "=>"),
            Token::Add => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Times => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::EndOfStatement => write!(f, ";"),
            Token::EndOfProgram => write!(f, "END OF PROGRAM"),
        }
    }
}

enum State {
    Start,
    Minus,
    IdentifierChar,
    NumberChar,
}

struct Tokenizer {
    state: State,
    stream: Vec<Token>,
    identifier: String,
    number: isize,
}

#[derive(Debug)]
enum TokenizerResult {
    Continue,
    Again,
    InvalidByte,
    Done,
}

fn is_identifier_first_byte(byte: u8) -> bool {
    if (byte >= b'a' &&
        byte <= b'z') ||
       (byte >= b'A' &&
        byte <= b'Z') ||
       byte == b'_' {
        true
    } else {
        false
    }
}

fn is_identifier_other_byte(byte: u8) -> bool {
    if (byte >= b'a' &&
        byte <= b'z') ||
       (byte >= b'A' &&
        byte <= b'Z') ||
       (byte >= b'0' &&
        byte <= b'9') ||
       byte == b'_' {
        true
    } else {
        false
    }
}

fn is_number_byte(byte: u8) -> bool {
    if byte >= b'0' &&
       byte <= b'9' {
        true
    } else {
        false
    }
}

fn is_space_byte(byte: u8) -> bool {
    if byte == b' ' ||
       byte == b'\r' ||
       byte == b'\n' {
        true
    } else {
        false
    }
}

fn fsm_proc(tokenizer: &mut Tokenizer, element: isize) -> TokenizerResult {
    match tokenizer.state {
        State::Start => {
            if element == -1 {
                return TokenizerResult::Done;
            }

            let byte = element as u8;

            if is_identifier_first_byte(byte) {
                tokenizer.identifier.clear();
                tokenizer.identifier.push(byte as char);

                tokenizer.state = State::IdentifierChar;
            } else if is_number_byte(byte) {
                let value = byte - b'0';

                tokenizer.number = value as isize;

                tokenizer.state = State::NumberChar;
            } else if byte == b'=' {
                tokenizer.stream.push(Token::Assign);
            } else if byte == b':' {
                tokenizer.stream.push(Token::VariableTypeIndicator);
            } else if byte == b'+' {
                tokenizer.stream.push(Token::Add);
            } else if byte == b'-' {
                tokenizer.state = State::Minus;
            } else if byte == b'*' {
                tokenizer.stream.push(Token::Times);
            } else if byte == b'/' {
                tokenizer.stream.push(Token::Divide);
            } else if byte == b'(' {
                tokenizer.stream.push(Token::LeftRoundBracket);
            } else if byte == b')' {
                tokenizer.stream.push(Token::RightRoundBracket);
            } else if byte == b'{' {
                tokenizer.stream.push(Token::LeftCurlyBracket);
            } else if byte == b'}' {
                tokenizer.stream.push(Token::RightCurlyBracket);
            } else if byte == b';' {
                tokenizer.stream.push(Token::EndOfStatement);
            } else if is_space_byte(byte) {
            } else {
                return TokenizerResult::InvalidByte;
            }
        },

        State::Minus => {
            if element == -1 {
                tokenizer.stream.push(Token::Minus);

                return TokenizerResult::Done;
            }

            let byte = element as u8;

            if byte == b'>' {
                tokenizer.stream.push(Token::ReturnTypeIndicator);

                tokenizer.state = State::Start;
            } else {
                tokenizer.stream.push(Token::Minus);

                tokenizer.state = State::Start;

                return TokenizerResult::Again;
            }
        },

        State::IdentifierChar => {
            if element == -1 {
                return TokenizerResult::Done;
            }

            let byte = element as u8;

            if is_identifier_other_byte(byte) {
                tokenizer.identifier.push(byte as char);
            } else {
                let identifier = tokenizer.identifier.to_owned();
                let token: Token;

                if KEYWORDS.contains(&identifier.as_str()) {
                    token = Token::Keyword(identifier);
                } else {
                    token = Token::Identifier(identifier);
                }

                tokenizer.stream.push(token);

                tokenizer.state = State::Start;

                return TokenizerResult::Again;
            }
        },

        State::NumberChar => {
            if element == -1 {
                return TokenizerResult::Done;
            }

            let byte = element as u8;

            if is_number_byte(byte) {
                let value = byte - b'0';

                tokenizer.number *= 10;
                tokenizer.number += value as isize;
            } else {
                let token = Token::Number(tokenizer.number);

                tokenizer.stream.push(token);

                tokenizer.state = State::Start;

                return TokenizerResult::Again;
            }
        }
    }

    TokenizerResult::Continue
}

impl Tokenizer {
    fn new() -> Tokenizer {
        Tokenizer {
            state: State::Start,
            stream: Vec::new(),
            identifier: String::new(),
            number: 0,
        }
    }

    fn feed(&mut self, byte: isize) -> TokenizerResult {
        let mut result: TokenizerResult;

        result = fsm_proc(self, byte);
        match result {
            TokenizerResult::Again => {
                result = fsm_proc(self, byte);
            },
            _ => {},
        }

        result
    }

    fn scan(&mut self, text: &str) {
        let text_buf = text.as_bytes();
        let text_len = text.len();

        for index in 0..text_len {
            let byte = text_buf[index] as isize;

            self.feed(byte);
        }
        
        self.feed(-1);
    }

    fn finalize(&mut self) {
        self.stream.push(Token::EndOfProgram);
    }
}

pub fn tokenize(text: &str) -> Vec<Token> {
    let mut tokenizer = Tokenizer::new();

    tokenizer.scan(text);
    tokenizer.finalize();

    tokenizer.stream
}
