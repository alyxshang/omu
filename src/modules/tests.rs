/*
OMU by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Token"
/// structure to use explicit
/// typing.
use super::lexer::Token;

/// Importing the "Lexer"
/// structure to test it.
use super::lexer::Lexer;

/// A function to test the crate's
/// lexer.
#[test]
pub fn test_lexer() -> (){
    let test_code: String = "<3<3 Heading 1\nHello World!";
    let result_stream: Vec<Token> = vec![
        Token::new(
        ),
        Token::new(
        );
        Token::new(
        ),
        Token::new(
        ),
        Token::new(
        )
    ]
    let mut lexer: Lexer = Lexer::new(&test_code);
    match lexer.lex(){
        Ok(stream) => assert_eq!(stream, result_stream),
        Err(e) => eprintln!("{}", &e)
    };
}
