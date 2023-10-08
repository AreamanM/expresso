//! Functions that expresso uses to perform lexical analysis of it's input to
//! convert it into tokens that it understands.

use std::{iter::Peekable, str::Chars};

use anyhow::{bail, Result};

use crate::token::{FuncKind, OpKind, Token};

/// A lexer that turns an iterator over characters into a vector of `Token`s.
///
/// The lexer only handles ascii alphanumeric and whitespace characters, any
/// unicode glpyhs, including non ascii numbers are treated as unrecognized.
///
/// # Arguments
///
/// * `cs` - A peekable character iterator which will be lexed.
///
/// # Examples
/// ```
/// use cocoa::{token::{Token, OpKind}, lexer::lex};
///
/// let mut input = "2 + 2".chars().peekable();
/// let expected = vec![
///     Token::Number(2.0),
///     Token::Op(OpKind::Plus),
///     Token::Number(2.0)
/// ];
///
/// assert_eq!(expected, lex(&mut input).unwrap());
/// ```
pub fn lex(cs: &mut Peekable<Chars>) -> Result<Vec<Token>> {
    let mut tokens = vec![];

    while let Some(&c) = cs.peek() {
        if c.is_ascii_whitespace() {
            cs.next();
            continue;
        }
        else if c.is_ascii_digit() || c == '.' {
            tokens.push(lex_number(cs)?);
        } else if c.is_ascii_alphabetic() {
            tokens.push(lex_ident(cs)?);
        } else {
            tokens.push(lex_op(c)?);
            cs.next();
        }
    }

    Ok(tokens)
}

fn lex_op(c: char) -> Result<Token> {
    match c {
        '+' => Ok(Token::Op(OpKind::Plus)),
        '-' => Ok(Token::Op(OpKind::Minus)),
        '*' => Ok(Token::Op(OpKind::Star)),
        '/' => Ok(Token::Op(OpKind::Slash)),
        '^' => Ok(Token::Op(OpKind::Caret)),
        '%' => Ok(Token::Op(OpKind::Modulo)),
        '!' => Ok(Token::Op(OpKind::Factorial)),
        '(' => Ok(Token::LParen),
        ')' => Ok(Token::RParen),
        _ => bail!("unrecognized character '{}'", c),
    }
}

fn lex_number(cs: &mut Peekable<Chars>) -> Result<Token> {
    let mut dot = false;
    let mut buf = String::new();

    // insert a 0 in the buffer is the number being parsed is a decimal
    // beginning with a '.'
    //
    // the `parse` function that will parse the buf into an f64 is not happy if
    // the buf begins with a '.' as opposed to '0.'
    if let Some(&c) = cs.peek() {
        if c == '.' {
            buf.push('0');
        }
    }

    while let Some(&c) = cs.peek() {
        if c.is_ascii_digit() || c == '.' {
            if c == '.' {
                if !dot {
                    dot = true;
                } else {
                    bail!("number cannot contain more than one decimal point")
                }
            }
            buf.push(c);
            cs.next();
        } else {
            break;
        }
    }

    Ok(Token::Number(buf.parse()?))
}

// note: a trie is more efficient for the purposes of this function, but the
// current implementation is easier to extend and makes for some nicer error
// messages
fn lex_ident(cs: &mut Peekable<Chars>) -> Result<Token> {
    let mut buf = String::new();

    while let Some(&c) = cs.peek() {
        if c.is_ascii_alphabetic() {
            buf.push(c);
            cs.next();
        } else {
            break;
        }
    }

    match buf.as_str().into() {
        "sin" => Ok(Token::Func(FuncKind::Sin)),
        "cos" => Ok(Token::Func(FuncKind::Cos)),
        "tan" => Ok(Token::Func(FuncKind::Tan)),
        "asin" => Ok(Token::Func(FuncKind::Asin)),
        "acos" => Ok(Token::Func(FuncKind::Acos)),
        "atan" => Ok(Token::Func(FuncKind::Atan)),
        "deg" => Ok(Token::Func(FuncKind::Deg)),
        "rad" => Ok(Token::Func(FuncKind::Rad)),
        "exp" => Ok(Token::Func(FuncKind::Exp)),
        "ln" => Ok(Token::Func(FuncKind::Ln)),
        "log" => Ok(Token::Func(FuncKind::Log)),
        "sqrt" => Ok(Token::Func(FuncKind::Sqrt)),
        // `pi` is treated as a regular floating point number
        "pi" => Ok(Token::Number(std::f64::consts::PI)),
        _ => bail!("unrecognized identifier '{}'", buf),
    }
}
