#[macro_use]
extern crate lalrpop_util;
use synterm::{syntax_highlight_gen, Color, CommandLineTool};
lalrpop_mod!(pub arithmetic);

struct App;

impl CommandLineTool for App {
    fn evaluator_function(line: &String) -> String {
        arithmetic::TermParser::new()
            .parse(&format!("({})", line))
            .unwrap()
            .to_string()
    }
    fn syntax_highlight(string: &str) {
        syntax_highlight_gen!(
            TheLexer,
            parser,
            (Operator, Color::Blue, r"\+|-|/|\*\*?|\^"),
            (Paren, Color::Magenta, r"\(|\)"),
            (Number, Color::Yellow, r"[\+-]?[1-9][0-9]*(?:\.[0-9]*)?"),
            (NoHighlight, Color::White, r"[a-zA-Z_$]+")
        );
        parser(TheLexer::lexer(string));
    }
}

fn main() {
    App.start();
}
