mod core;

pub use self::core::Parser;

#[test]
fn test() {
    use crate::lexer::Lexer;

    let source = "(1 + 2)";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    let mut parser = Parser::new(tokens);
    let expr = parser.parse();
    println!("{:?}", expr);
}
