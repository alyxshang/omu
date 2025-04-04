/*
OMU by Alyx Shang.
Licensed under the FSL v1.
*/

/// Declaring
/// the testing
/// module.
#[cfg(test)]
pub mod tests;

/// Re-exporting the module
/// responsible handling and
/// catching errors.
pub mod err;

/// Exporting the module
/// responsible for parsing
/// OwO Markup source
/// code.
pub mod ast;

/// Exporting the module
/// responsible for lexing
/// OwO Markup source 
/// code.
pub mod lexer;
