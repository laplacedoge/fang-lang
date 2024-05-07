mod lexer;
mod parser;

use lexer::Tokenizer;
use parser::Parser;

const SAMPLE_FANG: &str = "
var var_1;
var var_2 = 47;
var var_3: int;
var var_4: int = 23;
var var_5: int = var_1 + var_2;
var var_6: int = var_3 * var_4 - var_5;
var var_7: int = var_4 + var_5 * var_3;
var var_8: int = var_2 + var_3 - var_4;
var var_9: int = var_2 * var_3 / var_4;
var var_10: int = var_2 + var_3 / var_4 * var_5 - var_6 + var_7;

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
