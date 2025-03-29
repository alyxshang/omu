/*
OMU by Alyx Shang.
Licensed under the FSL v1.
*/

/// Declaring the
/// "modules" directory
/// as a module.
pub mod modules;

/// Re-exporting the module
/// responsible handling and
/// catching errors.
pub use modules::err::*;

/// Re-exporting the module
/// responsible for parsing
/// OwO Markup source
/// code.
pub use modules::ast::*;

/// Re-exporting the module
/// responsible for lexing
/// OwO Markup source 
/// code.
pub use modules::lexer::*;
