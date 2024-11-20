//! # Example Algebra BNF
//! expressions are terms that can be OR'd together:
//! <expression> ::= <term> ( "+" <term> )
//! terms are factors that can be AND'd together:
//! <term>       ::= <factor> ( <factor> )
//! factors are primed factors, bracketed expressions or literals:
//! <factor>     ::= <factor> "'" | "(" <expression> ")" | Literal
//! literals are just symbols:
//! <literal>    ::= [A-Za-z]
//!
//! # Design
//! Appearance of variants is similar to the order of BNF above.
//! Variants in boxes form a tree.
use std::{
	fmt::{Display, Formatter},
	str::FromStr
};

use itertools::Itertools;
use winnow::combinator::todo;

enum Expr {
	And(Box<Expr>, Box<Expr>),
	Or(Box<Expr>, Box<Expr>),
	Primed(Box<Expr>),
	Parenthesised(Box<Expr>),
	Literal(char)
}

impl Display for Expr {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			Expr::Or(a, b) => format!("{}+{}", a, b),
			Expr::And(a, b) => format!("{}{}", a, b),
			Expr::Parenthesised(t) => format!("({})", t),
			Expr::Primed(t) => format!("{}'", t),
			Expr::Literal(c) => format!("{}", c)
		})
	}
}

/// Accepted syntax is similar to a stack based language.
impl FromStr for Expr {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> { 
        s.chars().try_fold(None, |prev: Option<>, c| {
            Some(match prev {
                None => {
                    match c {
                        '+' => Err("Early `+`".to_string()),
                        '\'' => Err("Early `'`".to_string()),
                        _ => Err("Unsupported symbol".to_string())
                    }
                }
                Some(prev) => {
                    todo!()
                }
            })
        }).ok_or("No expression to parse.".to_string()).flatten()
	}
}
