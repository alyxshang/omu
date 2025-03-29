/*
OMU by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the structure to
/// catch and handle errors.
use super::err::OmuErr;

/// An enum to describe
/// every possible token
/// that can be encountered
/// in OwO Markup source code.
#[derive(Clone)]
pub enum TokenType{
    UserString,
    WhiteSpace,
    NewLine,
    HeadingMarker,
    OpenBracket,
    CloseBracket
}

/// A structure to
/// hold information
/// on a lexed token.
#[derive(Clone)]
pub struct Token{
    pub value: String,
    pub token_type: TokenType,
    pub start: usize,
    pub end: usize
}

/// Implementing generic
/// functions for the "Token"
/// structure.
impl Token {

    /// A function to create
    /// a new instance of the
    /// "Token" structure.
    pub fn new(
        value: &str,
        token_type: &TokenType,
        start: &usize,
        end: &usize
    ) -> Token {
        Token{
            value: value.to_owned(),
            token_type: token_type.clone(),
            start: *start,
            end: *end
        }
    }
}

/// A structure to hold
/// information on OwO
/// Markup source code and
/// the lexed stream of tokens.
pub struct Lexer {
    pub source: String,
    pub stream: Vec<Token>
}

/// Implementing important functions
/// for the "Lexer" structure.
impl Lexer {

    /// A function to create a new
    /// instance of the "Lexer"
    /// structurer.
    pub fn new(
        source: &str,
    ) -> Lexer {
        let stream: Vec<Token> = Vec::new();
        Lexer{
            source: source.to_owned(),
            stream: stream
        }
    }

    /// A function to lex the
    /// supplied source code and
    /// populate the "stream" field
    /// with a stream of tokens.
    pub fn lex(
        mut self
    ) -> Result<(), OmuErr>{
        let mut chars: Vec<char> = self
            .source
            .chars()
            .into_iter()
            .collect::<Vec<char>>();
        for (index,character) in chars
            .clone()
            .into_iter()
            .enumerate()
        {
            if character == '<'{
                if chars[index+1] == '3' && 
                   index != chars.len()
                {
                    self.stream.push(
                        Token::new(
                            "<3",
                            &TokenType::HeadingMarker,
                            &index,
                            &(index+1)
                        )
                    );
                    chars.remove(index);
                    chars.remove(index+1);
                }
                else {
                    let e: String = format!(
                        "Unexpected token encountered at column \"{}\"!",
                        index,
                    );
                    return Err::<(), OmuErr>(
                        OmuErr::new(
                            &e.to_string()
                        )
                    );
                }
            }
            else if character == '\n'{
                self.stream.push(
                    Token::new(
                        &"\n",
                        &TokenType::NewLine,
                        &index,
                        &index
                    )
                );
            }
            else if character == ' '{
                self.stream.push(
                    Token::new(
                        " ",
                        &TokenType::WhiteSpace,
                        &index,
                        &index
                    )
                );
                chars.remove(index);
            }
            else if character == '('{
                self.stream.push(
                    Token::new(
                        "(",
                        &TokenType::OpenBracket,
                        &index,
                        &index
                    )
                );
            }
            else if character == ')'{
                self.stream.push(
                    Token::new(
                        ")",
                        &TokenType::CloseBracket,
                        &index,
                        &index
                    )
                );
            }
            else {
                if character.is_alphanumeric(){
                    self.stream.push(
                        Token::new(
                            &character.to_string(),
                            &TokenType::UserChar,
                            &index,
                            &index
                        )
                    );
                    chars.remove(index);
                }
                else {
                    let e: String = format!(
                        "Unexpected token at column \"{}\"!",
                        &index
                    );
                    return Err::<(), OmuErr>(
                        OmuErr::new(
                            &e.to_string()
                        )
                    );
                }
            }
        }
        Ok(())
    }

}
