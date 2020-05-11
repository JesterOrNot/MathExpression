#[macro_use]
extern crate lalrpop_util;
use synterm::{syntax_highlight_gen, Color, CommandLineTool};

lalrpop_mod!(pub arithmetic);

struct App;

fn main() {
    App.start();
}

impl CommandLineTool for App {
    fn evaluator_function(line: &String) -> String {
        match arithmetic::TermParser::new().parse(&format!("({})", line)) {
            Ok(n) => n.to_string(),
            Err(n) => format!("Error: {}!", n).to_string()
        }
    }
    fn syntax_highlight(string: &str) {
        syntax_highlight_gen!(
            TheLexer,
            parser,
            (Operator, Color::Blue, r"\+|-|/|\*\*?|\^"),
            (Paren, Color::Magenta, r"\(|\)"),
            (Number, Color::Yellow, r"[1-9][0-9]*(?:\.[0-9]*)?"),
            (Function, Color::Cyan, r"log|sqrt|abs")
        );
        parser(TheLexer::lexer(string));
    }
}
