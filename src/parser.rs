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

#[derive(PartialEq, Debug)]
enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(PartialEq, Debug)]
enum Expression {
    Identifier(String),
    Number(isize),
    BinaryOperation {
        operator: BinaryOperator,
        operand_left: Box<Expression>,
        operand_right: Box<Expression>,
    },
}

#[derive(PartialEq, Debug)]
enum Statement {
    VariableDeclaration {
        identifier: String,
        r#type: Option<String>,
        value: Option<Expression>,
    },
}

#[derive(PartialEq, Debug)]
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

#[cfg(test)]
mod tests {
    use crate::lexer::*;
    use super::*;

    #[macro_export]
    macro_rules! scan_and_parse_program {
        ($text:expr) => {{
            let mut tokenizer = Tokenizer::new();
            let stream: Stream;
            let mut parser: Parser;

            tokenizer.scan($text);
            stream = tokenizer.extract();
            parser = Parser::new(stream);

            parser.parse_program()
        }};
    }

    #[test]
    fn variable_declaration() {
        let mut program: Program;

        program = scan_and_parse_program!("var var_1;");
        assert_eq!(program, Program {
            statements: vec![
                Statement::VariableDeclaration {
                    identifier: String::from("var_1"),
                    r#type: None,
                    value: None,
                },
            ],
        });

        program = scan_and_parse_program!("var var_2 = 47;");
        assert_eq!(program, Program {
            statements: vec![
                Statement::VariableDeclaration {
                    identifier: String::from("var_2"),
                    r#type: None,
                    value: Some(Expression::Number(47)),
                },
            ],
        });

        program = scan_and_parse_program!("var var_3: int;");
        assert_eq!(program, Program {
            statements: vec![
                Statement::VariableDeclaration {
                    identifier: String::from("var_3"),
                    r#type: Some(String::from("int")),
                    value: None,
                },
            ],
        });

        program = scan_and_parse_program!("var var_4: int = 23;");
        assert_eq!(program, Program {
            statements: vec![
                Statement::VariableDeclaration {
                    identifier: String::from("var_4"),
                    r#type: Some(String::from("int")),
                    value: Some(Expression::Number(23)),
                },
            ],
        });

        program = scan_and_parse_program!("var var_5: int = var_1 + var_2;");
        assert_eq!(program, Program {
            statements: vec![
                Statement::VariableDeclaration {
                    identifier: String::from("var_5"),
                    r#type: Some(String::from("int")),
                    value: Some(Expression::BinaryOperation {
                        operator: BinaryOperator::Addition,
                        operand_left: Box::new(
                            Expression::Identifier(String::from("var_1"))),
                        operand_right: Box::new(
                            Expression::Identifier(String::from("var_2"))),
                    }),
                },
            ],
        });

        program = scan_and_parse_program!("var var_6: int = var_3 * var_4 - var_5;");
        assert_eq!(program, Program {
            statements: vec![
                Statement::VariableDeclaration {
                    identifier: String::from("var_6"),
                    r#type: Some(String::from("int")),
                    value: Some(Expression::BinaryOperation {
                        operator: BinaryOperator::Subtraction,
                        operand_left: Box::new(Expression::BinaryOperation {
                            operator: BinaryOperator::Multiplication,
                            operand_left: Box::new(
                                Expression::Identifier(String::from("var_3"))),
                            operand_right: Box::new(
                                Expression::Identifier(String::from("var_4"))),
                        }),
                        operand_right: Box::new(
                            Expression::Identifier(String::from("var_5"))),
                    }),
                },
            ],
        });

        program = scan_and_parse_program!("var var_7: int = var_3 * (var_4 - var_5);");
        assert_eq!(program, Program {
            statements: vec![
                Statement::VariableDeclaration {
                    identifier: String::from("var_7"),
                    r#type: Some(String::from("int")),
                    value: Some(Expression::BinaryOperation {
                        operator: BinaryOperator::Multiplication,
                        operand_left: Box::new(
                            Expression::Identifier(String::from("var_3"))),
                        operand_right: Box::new(Expression::BinaryOperation {
                            operator: BinaryOperator::Subtraction,
                            operand_left: Box::new(
                                Expression::Identifier(String::from("var_4"))),
                            operand_right: Box::new(
                                Expression::Identifier(String::from("var_5"))),
                        }),
                    }),
                },
            ],
        });
    }
}
