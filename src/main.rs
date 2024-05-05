mod lexer;

use lexer::tokenize;

const SAMPLE_FANG: &str = "
var var_1 = 47;
var var_2 = -23;

func add_num(a: int, b: int) -> int {
    return a + b;
}

var var_3 = add_num(var_1, var_2);

";

fn main() {
    let stream = tokenize(SAMPLE_FANG);

    dbg!(stream);
}
