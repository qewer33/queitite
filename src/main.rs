use clap::Parser as ClapParser;
use std::path::PathBuf;

use crate::{evaluator::Evaluator, lexer::Lexer, parser::Parser, reporter::Reporter, src::Src};

pub mod evaluator;
pub mod lexer;
pub mod parser;
pub mod reporter;
pub mod src;

#[derive(ClapParser, Debug)]
#[command(
    name = "queitite",
    about = "queitite interpreter",
    version = "0.0.1",
    author = "qewer33"
)]
struct Args {
    /// Program file to run
    file: PathBuf,

    /// Dump token stream and exit
    #[arg(long, conflicts_with_all = ["dump_ast", "verbose"])]
    dump_tokens: bool,

    /// Dump AST and exit
    #[arg(long, conflicts_with_all = ["dump_tokens", "verbose"])]
    dump_ast: bool,

    /// Dump tokens and AST, then execute
    #[arg(long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    // 1) Read source
    let mut src = Src::new(args.file);

    // 2) Lex
    let mut lexer = Lexer::new(src.text.clone());
    src.tokens = Some(lexer.tokenize());

    if args.dump_tokens || args.verbose {
        println!("== TOKENS ==");
        dbg!(&src.tokens);
        if args.dump_tokens {
            return;
        }
    }

    // 3) Parse
    let mut parser = Parser::new(&src);
    let parser_out = parser.parse();
    src.ast = match parser_out.ast {
        Some(s) => Some(s),
        None => {
            // Exit on parse error
            Reporter::error(
                format!("parser exited with {} errors", parser_out.error_count).as_str(),
            );
            std::process::exit(1);
        }
    };

    if args.dump_ast || args.verbose {
        println!("== AST ==");
        dbg!(&src.ast);
        if args.dump_ast {
            return;
        }
    }

    // 4) Execute
    let mut evaluator = Evaluator::new(&src);
    evaluator.eval();
}
