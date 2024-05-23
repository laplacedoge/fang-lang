/*

GRAMMAR:

EXPR ::= COMP_OPERAND ("==" COMP_OPERAND | "!=" COMP_OPERAND)*

COMP_OPERAND ::= TERM ("+" TERM | "-" TERM)*

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

    Equal,
    NotEqual,

    Assign,
}

#[derive(PartialEq, Debug)]
enum Expression {
    Identifier(String),
    Number(isize),
    String(String),
    BinaryOperation {
        operator: BinaryOperator,
        operand_left: Box<Expression>,
        operand_right: Box<Expression>,
    },
    FunctionCall {
        callee_name: String,
        arguments: Vec<Expression>,
    }
}

/// Function parameter.
#[derive(PartialEq, Debug)]
struct Parameter {
    name: String,
    r#type: Option<String>,
}

/// Statement, the basic element to form a program.
#[derive(PartialEq, Debug)]
enum Statement {

    /// Variable definition statement.
    /// 
    /// # Examples
    /// ```fang
    /// let value;
    /// let text = "Hello, world!";
    /// let text: String;
    /// let num: usize = 47;
    /// ```
    /// 
    /// # Fields
    /// - `identifier` Identifier of the defined variable.
    /// - `type` Type of the defined variable.
    /// - `value` Initial value of the defined variable.
    VariableDefinition {
        identifier: String,
        r#type: Option<String>,
        value: Option<Expression>,
    },

    /// Function definition statement.
    /// 
    /// # Examples
    /// ```fang
    /// func add_num(a: int, b: int) -> int {
    ///     return a + b;
    /// }
    /// ```
    /// 
    /// # Fields
    /// - `callee_name` Function name.
    /// - `parameters` All parameters.
    /// - `return_type` Type of the return value.
    /// - `statements` All statements inside the function body.
    FunctionDefinition {
        callee_name: String,
        parameters: Vec<Parameter>,
        return_type: Option<String>,
        statements: Vec<Statement>,
    },

    /// Return statement.
    /// 
    /// # Examples
    /// ```fang
    /// return num_1 == num_2;
    /// ```
    /// 
    /// # Fields
    /// - `expression` Returned expression.
    Return {
        expression: Expression,
    },

    /// Expression statement.
    /// 
    /// # Examples
    /// ```fang
    /// name = "Alex Chen";
    /// value = (init + 3) * 4;
    /// ```
    /// 
    /// # Fields
    /// - `expression` Expression.
    Expression {
        expression: Expression,
    },

    /// Block statement.
    /// 
    /// # Examples
    /// ```fang
    /// {
    ///     let value = 33;
    /// }
    /// ```
    /// 
    /// # Fields
    /// - `statements` All statements in this block.
    Block {
        statements: Vec<Statement>,
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
        let statement: Statement;

        statement = match self.stream.peek() {
            Some(Token::LeftCurlyBracket) =>
                self.parse_block_statement(),
            Some(Token::Let) =>
                self.parse_variable_definition_statement(),
            Some(Token::Function) =>
                self.parse_function_definition_statement(),
            Some(Token::Return) =>
                self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        };

        statement
    }

    fn parse_block_statement(
        &mut self
    ) -> Statement {
        let mut statements: Vec<Statement> = Vec::new();
        let statement: Statement;

        self.stream.consume();

        loop {
            match self.stream.peek() {
                None => panic!("Expected statements or \"}}\"!"),
                Some(Token::RightCurlyBracket) => break,
                _ => statements.push(self.parse_statement()),
            }
        }

        match self.stream.consume() {
            Some(Token::RightCurlyBracket) => {},
            _ => panic!("Expected \"}}\"!"),
        }

        statement = Statement::Block {
            statements,
        };

        statement
    }

    fn parse_variable_definition_statement(
        &mut self
    ) -> Statement {
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

        statement = Statement::VariableDefinition {
            identifier,
            r#type,
            value,
        };

        statement
    }

    fn parse_function_definition_statement(
        &mut self
    ) -> Statement {
        let statement: Statement;
        let callee_name: String;
        let parameters: Vec<Parameter>;
        let return_type: Option<String>;
        let statements: Vec<Statement>;

        self.stream.consume();

        callee_name = match self.stream.consume() {
            Some(Token::Identifier(id)) => id,
            _ => panic!("Expected identifier!"),
        };

        parameters = self.parse_function_parameters();

        match self.stream.peek() {
            Some(Token::ReturnTypeIndicator) => {
                self.stream.consume();

                return_type = match self.stream.consume() {
                    Some(Token::Identifier(id)) => Some(id),
                    _ => panic!("Expected identifier!"),
                }
            },
            _ => return_type = None,
        }

        statements = self.parse_function_body();

        statement = Statement::FunctionDefinition {
            callee_name,
            parameters,
            return_type,
            statements,
        };

        statement
    }

    fn parse_function_parameters(
        &mut self
    ) -> Vec<Parameter> {
        let mut parameters: Vec<Parameter> = Vec::new();

        /* Consume `(`. */
        match self.stream.consume() {
            Some(Token::LeftRoundBracket) => {},
            _ => panic!("Expected \"(\"!"),
        };

        loop {
            match self.stream.peek() {
                Some(Token::RightRoundBracket) => break,
                Some(Token::Identifier(_)) =>
                    parameters.push(self.parse_function_parameter()),
                _ => panic!("Expected parameters or \")\"!"),
            }

            match self.stream.peek() {
                Some(Token::Comma) => {
                    self.stream.consume();
                },
                Some(Token::RightRoundBracket) => break,
                _ => panic!("Expected \",\" or \")\"!"),
            }
        }

        /* Consume `)`. */
        match self.stream.consume() {
            Some(Token::RightRoundBracket) => {},
            _ => panic!("Expected \")\"!"),
        };

        parameters
    }

    fn parse_function_parameter(
        &mut self
    ) -> Parameter {
        let parameter: Parameter;
        let name: String;
        let r#type: Option<String>;

        /* Consume parameter name. */
        name = match self.stream.consume() {
            Some(Token::Identifier(id)) => id,
            _ => panic!("Expected identifier!"),
        };

        /* Try to parse parameter type. */
        match self.stream.peek() {
            Some(Token::VariableTypeIndicator) => {
                self.stream.consume();

                r#type = match self.stream.consume() {
                    Some(Token::Identifier(id)) => Some(id),
                    _ => panic!("Expected identifier!"),
                }
            },
            _ => r#type = None,
        }

        parameter = Parameter {
            name,
            r#type,
        };

        parameter
    }

    fn parse_function_body(
        &mut self
    ) -> Vec<Statement> {
        let mut statements: Vec<Statement> = Vec::new();

        /* Consume `{`. */
        match self.stream.consume() {
            Some(Token::LeftCurlyBracket) => {},
            _ => panic!("Expected \"{{\"!"),
        }

        /* Parse all statements. */
        loop {
            match self.stream.peek() {
                None => panic!("Expected statements or \"}}\"!"),
                Some(Token::RightCurlyBracket) => break,
                _ => statements.push(self.parse_statement()),
            }
        }

        /* Consume `}`. */
        match self.stream.consume() {
            Some(Token::RightCurlyBracket) => {},
            _ => panic!("Expected \"}}\"!"),
        }

        statements
    }

    fn parse_return_statement(
        &mut self
    ) -> Statement {
        let statement: Statement;
        let expression: Expression;

        /* Consume `return`. */
        self.stream.consume();

        /* Parse expression. */
        expression = self.parse_expression();

        /* Consume `;`. */
        match self.stream.consume() {
            Some(Token::EndOfStatement) => {},
            _ => panic!("Expected \";\"!"),
        };

        statement = Statement::Return {
            expression,
        };

        statement
    }

    fn parse_expression_statement(&mut self) -> Statement {
        let expression: Expression;

        expression = self.parse_expression();

        match self.stream.consume() {
            Some(Token::EndOfStatement) => {},
            _ => panic!("Expected \";\"!"),
        };

        Statement::Expression {
            expression: expression,
        }
    }

    fn parse_expression(&mut self) -> Expression {
        let mut expression_left: Expression;

        expression_left = self.parse_assignment_operand();

        while let Some(token) = self.stream.peek() {
            match token {
                Token::Assign => {
                    let expression_right: Expression;

                    self.stream.consume();

                    expression_right = self.parse_assignment_operand();

                    expression_left = Expression::BinaryOperation {
                        operator: BinaryOperator::Assign,
                        operand_left: Box::new(expression_left),
                        operand_right: Box::new(expression_right),
                    }
                },
                _ => break,
            }
        }

        expression_left
    }

    /// Parse assignment operand in assignment like `expr_1 = expr_2`.
    fn parse_assignment_operand(&mut self) -> Expression {
        let mut expression_left: Expression;

        expression_left = self.parse_comparison_operand();

        while let Some(token) = self.stream.peek() {
            match token {
                Token::Equal |
                Token::NotEqual => {
                    let operator = match self.stream.consume() {
                        Some(Token::Equal) => BinaryOperator::Equal,
                        Some(Token::NotEqual) => BinaryOperator::NotEqual,
                        _ => panic!(),
                    };
                    let expression_right = self.parse_comparison_operand();

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

    /// Parse comparison operand in comparisons like
    /// `expr_1 == expr_2` or `expr_1 != expr_2`.
    fn parse_comparison_operand(&mut self) -> Expression {
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

        expression = match self.stream.peek() {
            Some(Token::Identifier(_)) =>
                self.parse_identifier_or_function_call(),
            Some(Token::Number(_)) =>
                self.parse_number(),
            Some(Token::String(_)) =>
                self.parse_string(),
            Some(Token::LeftRoundBracket) =>
                self.parse_grouped_expression(),
            _ => panic!("Expected expression!"),
        };

        expression
    }

    fn parse_identifier_or_function_call(
        &mut self
    ) -> Expression {
        let expression: Expression;
        let identifier = match self.stream.consume() {
            Some(Token::Identifier(id)) => id,
            _ => panic!("Expected identifier!"),
        };

        expression = match self.stream.peek() {
            Some(Token::LeftRoundBracket) => {
                let arguments = self.parse_function_call_arguments();

                Expression::FunctionCall {
                    callee_name: identifier,
                    arguments: arguments,
                }
            },
            _ => Expression::Identifier(identifier),
        };

        expression
    }

    fn parse_function_call_arguments(
        &mut self
    ) -> Vec<Expression> {
        let mut arguments: Vec<Expression> = Vec::new();

        /* Consume `(`. */
        self.stream.consume();

        match self.stream.peek() {
            Some(Token::RightRoundBracket) => {

                /* Consume `)`. */
                self.stream.consume();
            },
            _ => {
                let mut expression: Expression;

                loop {
                    expression = self.parse_expression();
                    arguments.push(expression);

                    match self.stream.peek() {
                        Some(Token::Comma) => {

                            /* Consume `,`. */
                            self.stream.consume();

                            continue;
                        },
                        Some(Token::RightRoundBracket) => {

                            /* Consume `)`. */
                            self.stream.consume();

                            break;
                        }
                        _ => panic!("Expected \",\" or \")\"!"),
                    }
                }
            },
        }

        arguments
    }

    fn parse_number(
        &mut self
    ) -> Expression {
        let number = match self.stream.consume() {
            Some(Token::Number(num)) => num,
            _ => panic!("Expected number!"),
        };

        Expression::Number(number)
    }

    fn parse_string(
        &mut self
    ) -> Expression {
        let string = match self.stream.consume() {
            Some(Token::String(str)) => str,
            _ => panic!("Expected string!"),
        };

        Expression::String(string)
    }

    fn parse_grouped_expression(
        &mut self
    ) -> Expression {
        let expression: Expression;

        self.stream.consume();

        expression = self.parse_expression();

        match self.stream.consume() {
            Some(Token::RightRoundBracket) => {},
            _ => panic!("Expected \")\"!"),
        }

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
    fn variable_definition() {
        let mut program: Program;

        program = scan_and_parse_program!("let var_1;");
        assert_eq!(program, Program {
            statements: vec![
                Statement::VariableDefinition {
                    identifier: String::from("var_1"),
                    r#type: None,
                    value: None,
                },
            ],
        });

        program = scan_and_parse_program!("let var_2 = 47;");
        assert_eq!(program, Program {
            statements: vec![
                Statement::VariableDefinition {
                    identifier: String::from("var_2"),
                    r#type: None,
                    value: Some(Expression::Number(47)),
                },
            ],
        });

        program = scan_and_parse_program!("let str_1 = \"Hello, world!\\r\\n\";");
        assert_eq!(program, Program {
            statements: vec![
                Statement::VariableDefinition {
                    identifier: String::from("str_1"),
                    r#type: None,
                    value: Some(Expression::String(
                        String::from("Hello, world!\\r\\n"))),
                },
            ],
        });

        program = scan_and_parse_program!("let var_3: int;");
        assert_eq!(program, Program {
            statements: vec![
                Statement::VariableDefinition {
                    identifier: String::from("var_3"),
                    r#type: Some(String::from("int")),
                    value: None,
                },
            ],
        });

        program = scan_and_parse_program!("let var_4: int = 23;");
        assert_eq!(program, Program {
            statements: vec![
                Statement::VariableDefinition {
                    identifier: String::from("var_4"),
                    r#type: Some(String::from("int")),
                    value: Some(Expression::Number(23)),
                },
            ],
        });

        program = scan_and_parse_program!("let var_5: int = var_1 + var_2;");
        assert_eq!(program, Program {
            statements: vec![
                Statement::VariableDefinition {
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

        program = scan_and_parse_program!("let var_6: int = var_3 * var_4 - var_5;");
        assert_eq!(program, Program {
            statements: vec![
                Statement::VariableDefinition {
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

        program = scan_and_parse_program!("let var_7: int = var_3 * (var_4 - var_5);");
        assert_eq!(program, Program {
            statements: vec![
                Statement::VariableDefinition {
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

    #[test]
    fn expression_assignment() {
        let program: Program;

        program = scan_and_parse_program!("value = (factor + 9) / 17;");
        assert_eq!(program, Program {
            statements: vec![
                Statement::Expression {
                    expression: Expression::BinaryOperation {
                        operator: BinaryOperator::Assign,
                        operand_left: Box::new(Expression::Identifier(
                            String::from("value")
                        )),
                        operand_right: Box::new(Expression::BinaryOperation {
                            operator: BinaryOperator::Division,
                            operand_left: Box::new(Expression::BinaryOperation {
                                operator: BinaryOperator::Addition,
                                operand_left: Box::new(Expression::Identifier(
                                    String::from("factor")
                                )),
                                operand_right: Box::new(Expression::Number(9)),
                            }),
                            operand_right: Box::new(Expression::Number(17)),
                        }),
                    },
                },
            ],
        });
    }

    #[test]
    fn block() {
        let program: Program;

        program = scan_and_parse_program!("let value = 17; { value = 45; { value = 33; } {} }");
        assert_eq!(program, Program {
            statements: vec![
                Statement::VariableDefinition {
                    identifier: String::from("value"),
                    r#type: None,
                    value: Some(Expression::Number(17)),
                },
                Statement::Block {
                    statements: vec![
                        Statement::Expression {
                            expression: Expression::BinaryOperation {
                                operator: BinaryOperator::Assign,
                                operand_left: Box::new(Expression::Identifier(String::from("value"))),
                                operand_right: Box::new(Expression::Number(45)),
                            },
                        },
                        Statement::Block {
                            statements: vec![
                                Statement::Expression {
                                    expression: Expression::BinaryOperation {
                                        operator: BinaryOperator::Assign,
                                        operand_left: Box::new(Expression::Identifier(String::from("value"))),
                                        operand_right: Box::new(Expression::Number(33)),
                                    },
                                },
                            ],
                        },
                        Statement::Block {
                            statements: vec![],
                        },
                    ],
                },
            ],
        });
    }
}
