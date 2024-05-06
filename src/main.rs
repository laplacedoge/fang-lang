mod lexer;
mod parser;

use lexer::Tokenizer;
use parser::Parser;

const SAMPLE_FANG: &str = "
var var_1 = 47;
// var var_2: int = 23;
/*
func add_num(a: int, b: int) -> int {
    return a + b;
}

var var_3 = add_num(var_1, var_2);

*/";

fn main() {
    let mut tokenizer = Tokenizer::new();

    tokenizer.scan(SAMPLE_FANG);

    let stream = tokenizer.extract();

    dbg!(&stream);

    let mut parser = Parser::new(stream);

    let program = parser.parse_program();

    dbg!(&program);
}
