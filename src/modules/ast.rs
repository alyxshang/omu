/*
Valkyrie by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Token"
/// structure because the
/// "Parser" structure 
/// analyzes a stream of
/// tokens.
use super::lexer::Token;

/// Importing the structure
/// to handle and return 
/// errors.
use super::err::OmuErr;

/// An enum to describe
/// every possible type
/// of statement that
/// can be found in
/// OwO Markup.
#[derive(Clone)]
pub enum StatementType{
    UserString,
    Link,
    Heading
}

/// A structure to save
/// information on 
/// parsed statements.
pub struct Statement {
    pub value: String,
    pub degree: Option<usize>,
    pub statement_type: StatementType
}

/// Implementing generic
/// functions for the
/// "Statement" structure.
impl Statement {

    /// Implementing a 
    /// generic function
    /// for creating a new
    /// instance of the
    /// "Statement" structure.
    pub fn new(
        value: &str,
        degree: &Option<usize>,
        statement_type: &StatementType
    ) -> Statement {
        Statement {
            value: value.to_owned(),
            degree: *degree,
            statement_type: statement_type.clone()
        }
    }

}

/// A structure to parse a 
/// token stream and return
/// a stream of parsed statements.
pub struct Parser{
    pub token_stream: Vec<Token>,
    pub statement_stream: Vec<Statement>
}

/// Implementing generic
/// functions for the "Parser"
/// structure.
impl Parser {

    /// Implementing a 
    /// function to create
    /// a new instance of the
    /// "Parser" structure.
    pub fn new(
        token_stream: &Vec<Token>
    ) -> Parser {
        let stmt_stream: Vec<Statement> = Vec::new();
        Parser {
            token_stream: token_stream.to_vec(),
            statement_stream: stmt_stream
        }
    }

    /// Implementing a function to take the
    /// supplied stream of tokens and parse
    /// it into a stream of statements.
    pub fn parse(
        self
    ) -> Result<(), OmuErr>{
       Ok(()) 
    }

}
