mod lexer;
mod parser;
mod frontend;

use clap::Parser;
use frontend::Frontend;

#[derive(Parser)]
#[command(name = "yuan")]
#[command(version = "1.0.0")]
#[command(about = "The compiler for Fang programming language", long_about = None)]
struct Cli {
    file_path: String,

    #[arg(short, long)]
    output_path: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let frontend = Frontend::new();

    frontend.process_file(&cli.file_path)
}
