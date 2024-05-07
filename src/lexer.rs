use std::vec::Vec;
use std::fmt::Debug;

#[derive(Clone)]
pub enum Token {

    /* Keywords like "var", or "func". */
    Variable,
    Function,
    Return,

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
            Token::Variable => write!(f, "VARIABLE"),
            Token::Function => write!(f, "FUNCTION"),
            Token::Return => write!(f, "RETURN"),
            Token::Identifier(text) => write!(f, "IDENTIFIER \"{}\"", text),
            Token::Assign => write!(f, "ASSIGN"),
            Token::Number(num) => write!(f, "NUMBER {}", num),
            Token::LeftRoundBracket => write!(f, "("),
            Token::RightRoundBracket => write!(f, ")"),
            Token::LeftCurlyBracket => write!(f, "{{"),
            Token::RightCurlyBracket => write!(f, "}}"),
            Token::VariableTypeIndicator => write!(f, "VARIABLE TYPE INDICATOR"),
            Token::ReturnTypeIndicator => write!(f, "RETURN TYPE INDICATOR"),
            Token::Add => write!(f, "ADD"),
            Token::Minus => write!(f, "MINUS"),
            Token::Times => write!(f, "TIMES"),
            Token::Divide => write!(f, "DIVIDE"),
            Token::EndOfStatement => write!(f, "END OF STATEMENT"),
            Token::EndOfProgram => write!(f, "END OF PROGRAM"),
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Variable, Token::Variable) |
            (Token::Function, Token::Function) |
            (Token::Return, Token::Return) |
            (Token::Identifier(_), Token::Identifier(_)) |
            (Token::Assign, Token::Assign) |
            (Token::Number(_), Token::Number(_)) |
            (Token::LeftRoundBracket, Token::LeftRoundBracket) |
            (Token::RightRoundBracket, Token::RightRoundBracket) |
            (Token::LeftCurlyBracket, Token::LeftCurlyBracket) |
            (Token::RightCurlyBracket, Token::RightCurlyBracket) |
            (Token::VariableTypeIndicator, Token::VariableTypeIndicator) |
            (Token::ReturnTypeIndicator, Token::ReturnTypeIndicator) |
            (Token::Add, Token::Add) |
            (Token::Minus, Token::Minus) |
            (Token::Times, Token::Times) |
            (Token::Divide, Token::Divide) |
            (Token::EndOfStatement, Token::EndOfStatement) |
            (Token::EndOfProgram, Token::EndOfProgram) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Stream {
    tokens: Vec<Token>,
    current: usize,
}

impl Stream {
    pub fn new(tokens: Vec<Token>) -> Stream {
        Stream {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn consume(&mut self) -> Option<Token>{
        if self.current < self.tokens.len() {
            let token = self.tokens[self.current].clone();

            self.current += 1;

            Some(token)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        if self.current < self.tokens.len() {
            Some(&self.tokens[self.current])
        } else {
            None
        }
    }

    pub fn match_token(&mut self, expected: Token) -> bool {
        if self.peek() == Some(&expected) {
            true
        } else {
            false
        }
    }
}

enum State {
    Start,
    Minus,
    Slash,
    IdentifierChar,
    NumberChar,
    SingleLineComment,
    MultiLineCommentHead,
    MultiLineCommentTailAsterisk,
}

pub struct Tokenizer {
    state: State,
    tokens: Vec<Token>,
    identifier: String,
    number: isize,
}

#[derive(Debug)]
enum Result {
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

fn fsm_proc(tokenizer: &mut Tokenizer, element: isize) -> Result {
    match tokenizer.state {
        State::Start => {
            if element == -1 {
                return Result::Done;
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
                tokenizer.tokens.push(Token::Assign);
            } else if byte == b':' {
                tokenizer.tokens.push(Token::VariableTypeIndicator);
            } else if byte == b'+' {
                tokenizer.tokens.push(Token::Add);
            } else if byte == b'-' {
                tokenizer.state = State::Minus;
            } else if byte == b'*' {
                tokenizer.tokens.push(Token::Times);
            } else if byte == b'/' {
                tokenizer.state = State::Slash;
            } else if byte == b'(' {
                tokenizer.tokens.push(Token::LeftRoundBracket);
            } else if byte == b')' {
                tokenizer.tokens.push(Token::RightRoundBracket);
            } else if byte == b'{' {
                tokenizer.tokens.push(Token::LeftCurlyBracket);
            } else if byte == b'}' {
                tokenizer.tokens.push(Token::RightCurlyBracket);
            } else if byte == b';' {
                tokenizer.tokens.push(Token::EndOfStatement);
            } else if is_space_byte(byte) {
            } else {
                return Result::InvalidByte;
            }
        },

        State::Minus => {
            if element == -1 {
                tokenizer.tokens.push(Token::Minus);

                return Result::Done;
            }

            let byte = element as u8;

            if byte == b'>' {
                tokenizer.tokens.push(Token::ReturnTypeIndicator);

                tokenizer.state = State::Start;
            } else {
                tokenizer.tokens.push(Token::Minus);

                tokenizer.state = State::Start;

                return Result::Again;
            }
        },

        State::IdentifierChar => {
            if element == -1 {
                return Result::Done;
            }

            let byte = element as u8;

            if is_identifier_other_byte(byte) {
                tokenizer.identifier.push(byte as char);
            } else {
                let identifier = tokenizer.identifier.to_owned();
                let text = identifier.as_str();
                let token: Token;

                if text == "var" {
                    token = Token::Variable;
                } else if text == "func" {
                    token = Token::Function;
                } else if text == "return" {
                    token = Token::Return;
                } else {
                    token = Token::Identifier(identifier);
                }

                tokenizer.tokens.push(token);

                tokenizer.state = State::Start;

                return Result::Again;
            }
        },

        State::NumberChar => {
            if element == -1 {
                return Result::Done;
            }

            let byte = element as u8;

            if is_number_byte(byte) {
                let value = byte - b'0';

                tokenizer.number *= 10;
                tokenizer.number += value as isize;
            } else {
                let token = Token::Number(tokenizer.number);

                tokenizer.tokens.push(token);

                tokenizer.state = State::Start;

                return Result::Again;
            }
        },

        State::Slash => {
            if element == -1 {
                tokenizer.tokens.push(Token::Divide);

                return Result::Done;
            }

            let byte = element as u8;

            if byte == b'/' {
                tokenizer.state = State::SingleLineComment;
            } else if byte == b'*' {
                tokenizer.state = State::MultiLineCommentHead;
            } else {
                tokenizer.tokens.push(Token::Divide);

                tokenizer.state = State::Start;

                return Result::Again;
            }
        },

        State::SingleLineComment => {
            if element == -1 {
                return Result::Done;
            }

            let byte = element as u8;

            if byte == b'\r' ||
               byte == b'\n' {
                tokenizer.state = State::Start;
            }
        },

        State::MultiLineCommentHead => {
            if element == -1 {
                return Result::Done;
            }

            let byte = element as u8;

            if byte == b'*' {
                tokenizer.state = State::MultiLineCommentTailAsterisk;
            }
        },

        State::MultiLineCommentTailAsterisk => {
            if element == -1 {
                return Result::Done;
            }

            let byte = element as u8;

            if byte == b'/' {
                tokenizer.state = State::Start;
            } else {
                tokenizer.state = State::MultiLineCommentHead;
            }
        },
    }

    Result::Continue
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            state: State::Start,
            tokens: Vec::new(),
            identifier: String::new(),
            number: 0,
        }
    }

    fn feed(&mut self, byte: isize) -> Result {
        let mut result: Result;

        result = fsm_proc(self, byte);
        match result {
            Result::Again => {
                result = fsm_proc(self, byte);
            },
            _ => {},
        }

        result
    }

    pub fn scan(&mut self, text: &str) {
        let text_buf = text.as_bytes();
        let text_len = text.len();

        for index in 0..text_len {
            let byte = text_buf[index] as isize;

            self.feed(byte);
        }

        self.feed(-1);

        self.tokens.push(Token::EndOfProgram);
    }

    pub fn extract(&mut self) -> Stream {
        let tokens = self.tokens.to_owned();

        self.state = State::Start;
        self.tokens = Vec::new();
        self.identifier = String::new();
        self.number = 0;

        Stream::new(tokens)
    }
}
