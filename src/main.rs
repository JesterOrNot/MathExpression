#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub paren); // synthesized by LALRPOP

fn main() {
    println!("{}", paren::TermParser::new().parse("((((22))))").unwrap() + 10)
}
