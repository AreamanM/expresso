//! Functions that convert a stream of tokens that are generated by the lexical
//! analyser into an output.

use std::iter::Peekable;

use anyhow::{bail, Result};

use crate::{
    math::ufactorial,
    token::{Bindable, OpKind, Token},
};

/// A parser which turns an iterator over `Token`s into an output.
///
/// The parser is an implementation of the Pratt parsing algorithm, all
/// operators have a binding power, and the binding power of an operator
/// determines the precedence of the operator.
///
/// E.g. multiplication has a higher precedence than addition, so `2 + 2 * 3`
/// is parsed as `2 + (2 * 3)`
///
/// # Arguments
///
/// * `tokens` - A peekable iterator over some tokens.
/// * `bp` - The minimum binding power the next operator should have in order
///          to be evaluated.
///
/// # Examples
/// ```
/// use cocoa::{token::{Token, OpKind}, parser::parse};
///
/// let mut tokens = vec![
///     Token::Number(2.0),
///     Token::Op(OpKind::Plus),
///     Token::Number(2.0)
/// ].into_iter().peekable();
///
/// // the binding power initially is always 0 so that the first operator
/// // in the expression is not skipped over
/// assert_eq!(4.0, parse(&mut tokens, 0).unwrap());
/// ```
pub fn parse<I: Iterator<Item = Token>>(
    tokens: &mut Peekable<I>,
    bp: u8,
) -> Result<f64> {
    let mut lhs = match tokens.next() {
        Some(t) => match t {
            Token::Number(n) => n,
            Token::Func(f) => {
                // not the best but it gets the job done
                if tokens.next() != Some(Token::LParen) {
                    bail!("expected '(' after token '{:?}'", f)
                }

                let rhs = parse(tokens, f.bp())?;
                f.eval(rhs)
            }
            // unary plus and minus
            Token::Op(o) => match o {
                OpKind::Plus | OpKind::Minus => {
                    // the binding power of unary plus/minus is 15 more than
                    // their infix binding power
                    let rhs = parse(tokens, o.bp() + 15)?;
                    match o {
                        OpKind::Plus => rhs,
                        OpKind::Minus => -rhs,
                        _ => unreachable!(),
                    }
                }
                _ => bail!("unexpected operator token '{:?}'", o),
            },
            Token::LParen => {
                let lhs = parse(tokens, 0)?;

                let next = tokens.next();
                if next != Some(Token::RParen) {
                    bail!("unmatched delimeter '('")
                }

                lhs
            }
            _ => bail!("unexpected token {:?}", t),
        },
        None => bail!("unexpected end of statement"),
    };

    loop {
        let &op = match tokens.peek() {
            Some(Token::Op(o)) => o,
            // an issue with this approach is that expressions such as
            // `(2 + 3)))) * 4` are valid as the extra RParens are simply consumed
            //
            // the ideal solution is a stack to keep track of delimeters
            Some(Token::RParen) => break,
            None => break,
            _ => bail!("unexpected token '{:?}'", tokens.peek()),
        };

        // postfix operators such as factorial need to be handled differently
        match op {
            OpKind::Factorial => {
                if op.bp() <= bp {
                    break;
                }

                tokens.next();

                if lhs.is_sign_negative() {
                    bail!("cannot calculate factorial of negative numbers")
                } else if lhs.fract() != 0.0 {
                    bail!("cannot calculate factorial of non integers")
                } else {
                    // casting is safe since lhs is clearly positive and has no
                    // fractional part if this bit of code is executed
                    lhs = ufactorial(lhs as u64) as f64;
                }

                continue;
            }
            _ => (),
        };

        if op.bp() <= bp {
            break;
        }

        tokens.next();

        let rhs = match op {
            // caret is right associative, 2^3^4 should be parsed as 2^(3^4)
            //
            // the effective binding power of right associative operators is
            // reduced by one so that the loop does not break if the next
            // operator is also the same right associative operator
            OpKind::Caret => parse(tokens, op.bp() - 1)?,
            _ => parse(tokens, op.bp())?,
        };

        lhs = match op {
            OpKind::Plus => lhs + rhs,
            OpKind::Minus => lhs - rhs,
            OpKind::Star => lhs * rhs,
            OpKind::Slash => lhs / rhs,
            OpKind::Modulo => lhs.rem_euclid(rhs),
            OpKind::Caret => lhs.powf(rhs),
            // factorial is handled in the postfix operator implementation
            _ => unreachable!(),
        };
    }

    Ok(lhs)
}
