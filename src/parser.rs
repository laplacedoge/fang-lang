/*

GRAMMAR:

EXPR ::= TERM ("+" TERM | "-" TERM)*

TERM ::= FACTOR ("*" FACTOR | "/" FACTOR)*

FACTOR ::= "(" EXPR ")"
         | IDENT
         | LITERAL

LITERAL ::= NUMBER

*/

use crate::lexer::{Token, Stream};

#[derive(Debug)]
enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug)]
enum Expression {
    Identifier(String),
    Number(isize),
    BinaryOperation {
        operator: BinaryOperator,
        operand_left: Box<Expression>,
        operand_right: Box<Expression>,
    },
}

#[derive(Debug)]
enum Statement {
    VariableDeclaration {
        identifier: String,
        r#type: Option<String>,
        value: Option<Expression>,
    },
}

#[derive(Debug)]
pub struct Program {
    statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct Parser {
    stream: Stream,
}

impl Parser {
    pub fn new(stream: Stream) -> Parser {
        Parser {
            stream,
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements: Vec<Statement> = Vec::new();

        while self.stream.peek() != Some(&Token::EndOfProgram) {
            let statement = self.parse_statement();

            statements.push(statement);
        }

        Program {
            statements,
        }
    }

    fn parse_statement(&mut self) -> Statement {
        let stream = &mut self.stream;
        let statement: Statement;

        if stream.match_token(Token::Variable) {
            statement = self.parse_variable_declaration();
        } else {
            panic!("");
        }

        statement
    }

    fn parse_variable_declaration(&mut self) -> Statement {
        let statement: Statement;
        let identifier: String;
        let r#type: Option<String>;
        let value: Option<Expression>;

        self.stream.consume();

        identifier = match self.stream.consume() {
            Some(Token::Identifier(id)) => id,
            _ => panic!("Expected identifier!"),
        };

        if self.stream.match_token(Token::VariableTypeIndicator) {
            self.stream.consume();

            r#type = match self.stream.consume() {
                Some(Token::Identifier(id)) => Some(id),
                _ => panic!("Expected identifier!"),
            };
        } else {
            r#type = None;
        }

        if self.stream.match_token(Token::EndOfStatement) {
            self.stream.consume();

            value = None;
        } else {
            match self.stream.consume() {
                Some(Token::Assign) => {},
                _ => panic!("Expected \"=\"!"),
            };

            value = Some(self.parse_expression());

            match self.stream.consume() {
                Some(Token::EndOfStatement) => {},
                _ => panic!("Expected \";\"!"),
            };
        }

        statement = Statement::VariableDeclaration {
            identifier,
            r#type,
            value,
        };

        statement
    }

    fn parse_expression(&mut self) -> Expression {
        let mut expression_left: Expression;

        expression_left = self.parse_term();

        while let Some(token) = self.stream.peek() {
            match token {
                Token::Add |
                Token::Minus => {
                    let operator = match self.stream.consume() {
                        Some(Token::Add) => BinaryOperator::Addition,
                        Some(Token::Minus) => BinaryOperator::Subtraction,
                        _ => panic!(),
                    };
                    let expression_right = self.parse_term();
    
                    expression_left = Expression::BinaryOperation {
                        operator,
                        operand_left: Box::new(expression_left),
                        operand_right: Box::new(expression_right),
                    }
                },
                _ => break,
            }
        }

        expression_left
    }

    fn parse_term(&mut self) -> Expression {
        let mut expression_left: Expression;

        expression_left = self.parse_factor();

        while let Some(token) = self.stream.peek() {
            match token {
                Token::Times |
                Token::Divide => {
                    let operator = match self.stream.consume() {
                        Some(Token::Times) => BinaryOperator::Multiplication,
                        Some(Token::Divide) => BinaryOperator::Division,
                        _ => panic!(),
                    };
                    let expression_right = self.parse_factor();

                    expression_left = Expression::BinaryOperation {
                        operator,
                        operand_left: Box::new(expression_left),
                        operand_right: Box::new(expression_right),
                    }
                },
                _ => break,
            }
        }

        expression_left
    }

    fn parse_factor(&mut self) -> Expression {
        let expression: Expression;

        expression = match self.stream.consume() {
            Some(Token::Identifier(id)) => Expression::Identifier(id),
            Some(Token::Number(num)) => Expression::Number(num),
            Some(Token::LeftRoundBracket) => {
                let expression_inner: Expression;

                expression_inner = self.parse_expression();

                match self.stream.consume() {
                    Some(Token::RightRoundBracket) => {},
                    _ => panic!("Expected \")\"!"),
                }

                expression_inner
            },
            _ => panic!("Expected expression!"),
        };

        expression
    }
}
