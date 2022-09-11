use fluxc_lexer::lex;
use fluxc_parser::parse;

#[test]
fn test_parse_fibonacci() {
    let src = include_str!("./fibonacci.flx");
    // lex
    let tokens = lex(src).expect("Lexing failed!");
    // parse
    let ast = parse(src, tokens).unwrap();
}
