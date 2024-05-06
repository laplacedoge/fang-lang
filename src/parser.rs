use crate::lexer::{Token, Stream};

#[derive(Debug)]
enum Literal {
    Number(isize),
}

#[derive(Debug)]
enum Expression {
    Literal(Literal),
}

#[derive(Debug)]
enum Statement {
    VariableDeclaration {
        identifier: String,
        _type: Option<String>,
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
        let _type: Option<String>;
        let value: Option<Expression>;

        self.stream.consume();

        identifier = match self.stream.consume() {
            Some(Token::Identifier(id)) => id,
            _ => panic!("Expected identifier!"),
        };

        if self.stream.match_token(Token::VariableTypeIndicator) {
            self.stream.consume();

            _type = match self.stream.consume() {
                Some(Token::Identifier(id)) => Some(id),
                _ => panic!("Expected identifier!"),
            };
        } else {
            match self.stream.consume() {
                Some(Token::Assign) => {},
                _ => panic!("Expected \"=\"!"),
            };

            _type = None;
        }

        if self.stream.match_token(Token::EndOfStatement) {
            self.stream.consume();

            value = None;
        } else {
            value = Some(self.parse_expression());

            match self.stream.consume() {
                Some(Token::EndOfStatement) => {},
                _ => panic!("Expected \";\"!"),
            };
        }

        statement = Statement::VariableDeclaration {
            identifier,
            _type,
            value,
        };

        statement
    }

    fn parse_expression(&mut self) -> Expression {
        let expression: Expression;

        if self.stream.match_token(Token::Number(0)) {
            expression = Expression::Literal(self.parse_literal());
        } else {
            panic!("Expected literal!")
        }

        expression
    }

    fn parse_literal(&mut self) -> Literal {
        let number;

        number = match self.stream.consume() {
            Some(Token::Number(num)) => num,
            _ => panic!("Expected number!"),
        };

        Literal::Number(number)
    }
}
