use crate::lexer::Tokenizer;
use crate::parser::Parser;
use std::fs::File;
use std::io::Read;

pub struct Frontend {

}

impl Frontend {
    pub fn new() -> Frontend {
        Frontend {

        }
    }

    fn process_string(&self, str: &str) {
        let mut tokenizer = Tokenizer::new();

        tokenizer.scan(str);

        let stream = tokenizer.extract();

        dbg!(&stream);

        let mut parser = Parser::new(stream);

        let program = parser.parse_program();

        dbg!(&program);
    }

    pub fn process_file(&self, path: &String) {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open \"{}\": {}", path, err);
                return;
            },
        };
        let mut buf: Vec<u8> = Vec::new();

        file.read_to_end(&mut buf).unwrap();

        let str = String::from_utf8(buf).unwrap();

        self.process_string(&str);
    }
}
