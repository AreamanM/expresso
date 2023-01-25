//! # cocoa
//!
//! cocoa is the library the expresso binary uses to parse
//! arithmetic expressions.
//!
//! It contains an implementation for a lexer and a pratt parser which is used
//! to evaluate mathematical expressions.

pub mod lexer;
pub mod math;
pub mod parser;
pub mod token;
