use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::num::NonZero;
use std::str::FromStr;

/// Tree of terms
#[derive(Debug)]
pub enum ExprNode {
    Symbol(char),
    Primed(Box<ExprNode>),
    /// This allows us to evaluate `Or` before `And` or `Primed`
    Bracketed(Box<ExprNode>),
    // the below variants can chain multiple expressions together
    Or(Vec<ExprNode>),
    And(Vec<ExprNode>),
}
impl Display for ExprNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ExprNode::Symbol(c) => format!("{}", c),
                ExprNode::Primed(t) => format!("{}'", t),
                ExprNode::Bracketed(t) => format!("({})", t),
                ExprNode::Or(v) => v.iter().join("+"),
                //                ExprNode::Or(a, b) => format!("{}+{}", a, b),
                //                ExprNode::And(a, b) => format!("{}{}", a, b),
                ExprNode::And(v) => v.iter().join(""),
            }
        )
    }
}
#[derive(Debug)]
pub enum ExprParseErr {
    EmptyBrackets,
    MissingExprBeforePrime,
    MissingExprBeforeOr,
    UnmatchedBrackets {
        /// A positive value means too many `(`, a negative value means too many `)`.
        tally: NonZero<i32>,
    },
    IllegalCharacter(char),
    DoubleOr,
}

impl FromStr for ExprNode {
    type Err = ExprParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //        /// Backwards, as in, the provided character iterator needs to go backwards in order to
        //        /// handle prime targeting properly
        //        fn backwards_parse(
        //            chars: &mut impl Iterator<Item = char>,
        //        ) -> Result<Option<ExprNode>, ExprParseErr> {
        //            match chars.next() {
        //                None | Some(')') => Ok(None),
        //                Some('(') => match backwards_parse(chars)? {
        //                    None => Err(ExprParseErr::EmptyBrackets),
        //                    Some(sub_expr) => Ok(Some(ExprNode::Bracketed(Box::new(sub_expr)))),
        //                },
        //                Some('\'') => match backwards_parse(chars)? {
        //                    None => Err(ExprParseErr::MissingExprBeforePrime),
        //                    Some(sub_expr) => Ok(Some(match sub_expr {
        //                        // these can be naively wrapped
        //                        p @ ExprNode::Symbol(_)
        //                        | p @ ExprNode::Primed(_)
        //                        | p @ ExprNode::Bracketed(_) => ExprNode::Primed(Box::new(p)),
        //                        // these can only work because direction is right to left
        //                        ExprNode::Or(a, b) => ExprNode::Or(Box::new(ExprNode::Primed(a)), b),
        //                        ExprNode::And(a, b) => ExprNode::And(Box::new(ExprNode::Primed(a)), b),
        //                    })),
        //                },
        //                Some('.') => todo!("explicit AND-operator is not supported"),
        //                Some(_) => todo!("rest of the valid characters"),
        //            }
        //        }
        //
        //        match backwards_parse(&mut s.chars().rev()) {
        //            Ok(Some(expr)) => Ok(expr),
        //            Ok(None) => Err(ExprParseErr::EmptyBrackets), // this is dumb
        //            Err(e) => Err(e),
        //        }

        /// # Recursive descent
        /// plan divide parsing into the grammar rules.
        /// The grammar rules look something like this:
        /// ```
        /// Expression ::= Term ( "|" Term )*
        /// Term       ::= Factor ( "&" Factor )*
        /// Factor     ::= "!" Factor | "(" Expression ")" | Literal
        /// Literal    ::= [A-Za-z]
        /// ```
        fn _to_impl() {}
        todo!()
    }
}
