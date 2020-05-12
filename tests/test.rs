#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub arithmetic);

#[test]
fn multiplication() {
    let parser = arithmetic::TermParser::new();
    assert_eq!(parser.parse("(3*3)").unwrap(), 9.0);
    assert_eq!(parser.parse("(-3*-3)").unwrap(), 9.0);
    assert_eq!(parser.parse("(3*-3)").unwrap(), -9.0);
    assert_eq!(parser.parse("(-3*3)").unwrap(), -9.0);
}

#[test]
fn addition() {
    let parser = arithmetic::TermParser::new();
    assert_eq!(parser.parse("(3+3)").unwrap(), 6.0);
    assert_eq!(parser.parse("(-3+-3)").unwrap(), -6.0);
    assert_eq!(parser.parse("(3+-3)").unwrap(), 0.0);
    assert_eq!(parser.parse("(-3+3)").unwrap(), 0.0);
}

#[test]
fn division() {
    let parser = arithmetic::TermParser::new();
    assert_eq!(parser.parse("(6/2)").unwrap(), 3.0);
    assert_eq!(parser.parse("(6/-2)").unwrap(), -3.0);
    assert_eq!(parser.parse("(-6/-2)").unwrap(), 3.0);
    assert_eq!(parser.parse("(-6/2)").unwrap(), -3.0);
    #[should_panic]
    assert_eq!(parser.parse("(6/0)").unwrap(), 0.0);
}

#[test]
fn subtraction() {
    let parser = arithmetic::TermParser::new();
    assert_eq!(parser.parse("(-3--3)").unwrap(), 0.0);
    assert_eq!(parser.parse("(3--3)").unwrap(), 6.0);
    assert_eq!(parser.parse("(-3-3)").unwrap(), 6.0);
    assert_eq!(parser.parse("(3-3)").unwrap(), 0.0);
}
