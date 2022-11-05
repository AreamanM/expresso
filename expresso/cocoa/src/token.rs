//! Implementation of data structures that represent expresso's input.

/// A valid token expresso understands.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    /// A valid operator.
    Op(OpKind),
    /// Builtin functions.
    Func(FuncKind),

    /// A valid number represented as a 64-bit floating point value.
    Number(f64),

    /// A left bracket (`(`).
    LParen,
    /// A right bracket (`)`).
    RParen,
}

/// All operators that expresso supports.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpKind {
    /// Addition operator.
    Plus,
    /// Subtraction operator.
    Minus,
    /// Multiplication operator.
    Star,
    /// Division operator.
    Slash,
    /// Euclidian division(remainder) operator.
    Modulo,
    /// Exponentiation operator
    Caret,
    /// Factorial operator.
    Factorial,
}

/// All functions that expresso supports.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FuncKind {
    /// Trignometric sine.
    Sin,
    /// Trignometric cosine.
    Cos,
    /// Trignometric tangent.
    Tan,
    /// Exponential function; `exp(x)` is equivalent to `e^x`.
    Exp,
    /// Natural log.
    Ln,
    /// Log base 10.
    Log,
}

impl OpKind {
    /// Get the binding power of an operator.
    ///
    /// The binding power is used in the pratt parsing algorithm to determine
    /// the precedence of an operator.
    ///
    /// The values themselves are arbitrary, however in order to enforce
    /// predence, an operator `A` with higher precedence than another operator
    /// `B` must have a higher binding power than `A`.
    ///
    /// Operators with the same level of precedence have the same binding power,
    /// since their respective operations can be done in any order,
    /// e.g `2 + 5 - 3` is the same as `5 - 3 + 2` since the order of addition
    /// or subtraction does not matter in this case.
    ///
    /// These binding powers are only applicable for postfix and infix
    /// operators, for unary plus or minus(which are both prefix operators), 15
    /// is added to the regular unary binding power for plus and minus.
    ///
    /// The higher the binding power, the more higher the precedence of the
    /// operator.
    ///
    /// # Examples
    /// ```
    /// use cocoa::token::OpKind;
    ///
    /// assert_eq!(OpKind::Plus.bp(), OpKind::Minus.bp());
    /// assert!(OpKind::Star.bp() > OpKind::Plus.bp());
    /// assert!(OpKind::Modulo.bp() > OpKind::Star.bp());
    /// assert!(OpKind::Caret.bp() > OpKind::Modulo.bp());
    /// assert!(OpKind::Factorial.bp() > OpKind::Modulo.bp());
    /// ```
    pub fn bp(self) -> u8 {
        match self {
            OpKind::Plus | OpKind::Minus => 5,
            OpKind::Star | OpKind::Slash => 10,
            OpKind::Modulo => 15,
            OpKind::Caret => 25,
            OpKind::Factorial => 30,
        }
    }
}

impl FuncKind {
    /// Get the binding power of a function.
    ///
    /// All functions have the same, and the highest binding power.
    ///
    /// Since functions have the highest binding power, an expression like
    /// `sin(2 + 2) - 3` is parsed as `(sin(2 + 2)) - 3` rather than
    /// `sin((2 + 2) - 3)`.
    ///
    /// # Examples
    /// ```
    /// use cocoa::token::{OpKind, FuncKind};
    ///
    /// assert_eq!(FuncKind::Sin.bp(), FuncKind::Ln.bp());
    /// assert!(FuncKind::Sin.bp() > OpKind::Factorial.bp());
    /// ```
    pub fn bp(self) -> u8 {
        35
    }

    /// Evaluate the given function at `input`.
    ///
    /// The result of these functions is entirely dependant on the way floating
    /// point arithmetic is implementedd in rust, hence a floating point
    /// error may appear in calculations.
    ///
    /// # Arguments
    ///
    /// * `input` - The value to evaluate the function at.
    ///
    /// # Examples
    /// ```
    /// use cocoa::token::FuncKind;
    ///
    /// assert_eq!(1.0, FuncKind::Ln.eval(FuncKind::Exp.eval(1.0)));
    /// ```
    pub fn eval(self, input: f64) -> f64 {
        match self {
            FuncKind::Sin => input.sin(),
            FuncKind::Cos => input.cos(),
            FuncKind::Tan => input.tan(),
            FuncKind::Exp => input.exp(),
            FuncKind::Ln => input.ln(),
            FuncKind::Log => input.log10(),
        }
    }
}
