use std::vec::Vec;
use std::fmt::Debug;

/// Tokens scanned out by the lexer.
#[derive(Clone)]
pub enum Token {

    /// Keyword `var`.
    Variable,

    /// Keyword `func`.
    Function,

    /// Keyword `return`.
    Return,

    /// Identifiers like `var_1`, or `add_num`.
    Identifier(String),

    /// Numeric literals like `0`, and `47`.
    Number(isize),

    /// String literals enclosed by double quote.
    /// For example, `"Hello"` and `"Alex Chen"`.
    String(String),

    /// Symbol `,`.
    Comma,

    /// Symbol `=`.
    Assign,

    /// Symbol `(`.
    LeftRoundBracket,

    /// Symbol `)`.
    RightRoundBracket,

    /// Symbol `{`.
    LeftCurlyBracket,

    /// Symbol `}`.
    RightCurlyBracket,

    /// Symbol `:`.
    VariableTypeIndicator,

    /// Symbol `->`.
    ReturnTypeIndicator,

    /// Symbol `==`.
    Equal,

    /// Symbol `!=`.
    NotEqual,

    /// Symbol `+`.
    Add,

    /// Symbol `-`.
    Minus,

    /// Symbol `*`.
    Times,

    /// Symbol `/`.
    Divide,

    /// Symbol `;`.
    EndOfStatement,

    /// End of program.
    EndOfProgram,
}

fn escape_string(str: &String) -> String {
    let str_buf = str.as_bytes();
    let str_len = str_buf.len();
    let mut line = String::new();

    for index in 0..str_len {
        let byte = str_buf[index];

        if byte == b'\r' {
            line.push_str("\\r");
        } else if byte == b'\n' {
            line.push_str("\\n");
        } else if byte == b'"' {
            line.push_str("\\\"");
        } else if byte >= 32 &&
                  byte <= 126 {
            line.push(byte as char);
        } else {
            line.push_str(&format!("\\x{:02X}", byte))
        }
    }

    line
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Variable => write!(f, "VARIABLE"),
            Token::Function => write!(f, "FUNCTION"),
            Token::Return => write!(f, "RETURN"),
            Token::Identifier(text) => write!(f, "IDENTIFIER \"{}\"", text),
            Token::Number(num) => write!(f, "NUMBER {}", num),
            Token::String(str) => write!(f, "STRING \"{}\"", escape_string(str)),
            Token::Comma => write!(f, "COMMA"),
            Token::Assign => write!(f, "ASSIGN"),
            Token::LeftRoundBracket => write!(f, "("),
            Token::RightRoundBracket => write!(f, ")"),
            Token::LeftCurlyBracket => write!(f, "{{"),
            Token::RightCurlyBracket => write!(f, "}}"),
            Token::VariableTypeIndicator => write!(f, "VARIABLE TYPE INDICATOR"),
            Token::ReturnTypeIndicator => write!(f, "RETURN TYPE INDICATOR"),
            Token::Equal => write!(f, "EQUAL"),
            Token::NotEqual => write!(f, "NOT EQUAL"),
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
            (Token::Number(_), Token::Number(_)) |
            (Token::String(_), Token::String(_)) |
            (Token::Comma, Token::Comma) |
            (Token::Assign, Token::Assign) |
            (Token::LeftRoundBracket, Token::LeftRoundBracket) |
            (Token::RightRoundBracket, Token::RightRoundBracket) |
            (Token::LeftCurlyBracket, Token::LeftCurlyBracket) |
            (Token::RightCurlyBracket, Token::RightCurlyBracket) |
            (Token::VariableTypeIndicator, Token::VariableTypeIndicator) |
            (Token::ReturnTypeIndicator, Token::ReturnTypeIndicator) |
            (Token::Equal, Token::Equal) |
            (Token::NotEqual, Token::NotEqual) |
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

/// States used for the lexer's FSM.
enum State {
    Start,

    /// Have character `=`.
    HaveCharEqual,

    /// Have character `!`.
    HaveCharExclamationMark,

    /// Have character `-`.
    HaveCharHyphen,

    /// Have character `/`.
    HaveCharForwardSlash,

    /// Have identifier character.
    HaveIdentifierChar,

    /// Have numeric character.
    HaveNumericChar,

    /// Have string start `"`.
    HaveStringStart,

    /// Have single-line comment start `//`.
    HaveSingleLineCommentStart,

    /// Have multi-line comment start `/*`.
    HaveMultiLineCommentStart,

    /// Have the character `*` possibly
    /// from the multi-line comment end `*/`.
    HaveMultiLineCommentEndCharAsterisk,
}

pub struct Tokenizer {
    state: State,
    tokens: Vec<Token>,
    identifier: String,
    number: isize,
    string: String,
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

fn is_ascii_printable_byte(byte: u8) -> bool {
    if byte >= 32 &&
       byte <= 126 {
        true
    } else {
        false
    }
}

fn fsm_proc(tokenizer: &mut Tokenizer, byte: Option<u8>) -> Result {
    match tokenizer.state {
        State::Start => {
            let byte = match byte {
                None => return Result::Done,
                Some(byte) => byte,
            };

            if is_identifier_first_byte(byte) {
                tokenizer.identifier.clear();
                tokenizer.identifier.push(byte as char);

                tokenizer.state = State::HaveIdentifierChar;
            } else if is_number_byte(byte) {
                let value = byte - b'0';

                tokenizer.number = value as isize;

                tokenizer.state = State::HaveNumericChar;
            } else if byte == b'"' {
                tokenizer.string.clear();

                tokenizer.state = State::HaveStringStart;
            } else if byte == b',' {
                tokenizer.tokens.push(Token::Comma);
            } else if byte == b'=' {
                tokenizer.state = State::HaveCharEqual;
            } else if byte == b'!' {
                tokenizer.state = State::HaveCharExclamationMark;
            } else if byte == b':' {
                tokenizer.tokens.push(Token::VariableTypeIndicator);
            } else if byte == b'+' {
                tokenizer.tokens.push(Token::Add);
            } else if byte == b'-' {
                tokenizer.state = State::HaveCharHyphen;
            } else if byte == b'*' {
                tokenizer.tokens.push(Token::Times);
            } else if byte == b'/' {
                tokenizer.state = State::HaveCharForwardSlash;
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

        State::HaveCharEqual => {
            let byte = match byte {
                None => {
                    tokenizer.tokens.push(Token::Assign);

                    return Result::Done;
                },
                Some(byte) => byte,
            };

            if byte == b'=' {
                tokenizer.tokens.push(Token::Equal);

                tokenizer.state = State::Start;
            } else {
                tokenizer.tokens.push(Token::Assign);

                tokenizer.state = State::Start;

                return Result::Again;
            }
        },

        State::HaveCharExclamationMark => {
            let byte = match byte {
                None => return Result::InvalidByte,
                Some(byte) => byte,
            };

            if byte == b'=' {
                tokenizer.tokens.push(Token::NotEqual);

                tokenizer.state = State::Start;
            } else {
                tokenizer.state = State::Start;

                return Result::InvalidByte;
            }
        },

        State::HaveCharHyphen => {
            let byte = match byte {
                None => {
                    tokenizer.tokens.push(Token::Minus);

                    return Result::Done;
                },
                Some(byte) => byte,
            };

            if byte == b'>' {
                tokenizer.tokens.push(Token::ReturnTypeIndicator);

                tokenizer.state = State::Start;
            } else {
                tokenizer.tokens.push(Token::Minus);

                tokenizer.state = State::Start;

                return Result::Again;
            }
        },

        State::HaveIdentifierChar => {
            let byte = match byte {
                None => return Result::Done,
                Some(byte) => byte,
            };

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

        State::HaveNumericChar => {
            let byte = match byte {
                None => return Result::Done,
                Some(byte) => byte,
            };

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

        State::HaveStringStart => {
            let byte = match byte {
                None => return Result::Done,
                Some(byte) => byte,
            };

            if byte == b'"' {
                let string = tokenizer.string.to_owned();
                let token = Token::String(string);

                tokenizer.tokens.push(token);

                tokenizer.state = State::Start;
            } else if byte == b'\r' ||
                      byte == b'\n' ||
                      is_ascii_printable_byte(byte) {
                tokenizer.string.push(byte as char);
            } else {
                return Result::InvalidByte;
            }
        },

        State::HaveCharForwardSlash => {
            let byte = match byte {
                None => {
                    tokenizer.tokens.push(Token::Divide);

                    return Result::Done
                },
                Some(byte) => byte,
            };

            if byte == b'/' {
                tokenizer.state = State::HaveSingleLineCommentStart;
            } else if byte == b'*' {
                tokenizer.state = State::HaveMultiLineCommentStart;
            } else {
                tokenizer.tokens.push(Token::Divide);

                tokenizer.state = State::Start;

                return Result::Again;
            }
        },

        State::HaveSingleLineCommentStart => {
            let byte = match byte {
                None => return Result::Done,
                Some(byte) => byte,
            };

            if byte == b'\r' ||
               byte == b'\n' {
                tokenizer.state = State::Start;
            }
        },

        State::HaveMultiLineCommentStart => {
            let byte = match byte {
                None => return Result::Done,
                Some(byte) => byte,
            };

            if byte == b'*' {
                tokenizer.state = State::HaveMultiLineCommentEndCharAsterisk;
            }
        },

        State::HaveMultiLineCommentEndCharAsterisk => {
            let byte = match byte {
                None => return Result::Done,
                Some(byte) => byte,
            };

            if byte == b'/' {
                tokenizer.state = State::Start;
            } else {
                tokenizer.state = State::HaveMultiLineCommentStart;
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
            string: String::new(),
        }
    }

    fn feed(&mut self, byte: Option<u8>) -> Result {
        let mut result: Result;

        loop {
            result = fsm_proc(self, byte);
            match result {
                Result::Again => continue,
                _ => break,
            }
        }

        result
    }

    pub fn scan(&mut self, text: &str) {
        let text_buf = text.as_bytes();
        let text_len = text.len();

        for index in 0..text_len {
            self.feed(Some(text_buf[index]));
        }

        self.feed(None);

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
