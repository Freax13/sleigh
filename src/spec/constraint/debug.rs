use super::*;
use std::fmt::{self, Debug, Display};

impl Debug for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constraint::Ellipsis(inner) => write!(f, "{}...", inner.op),
            Constraint::And(inner) => inner.fmt(f),
            Constraint::Or(inner) => inner.fmt(f),
            Constraint::Semi(inner) => inner.fmt(f),
            Constraint::Parenthesized(inner) => write!(f, "({})", inner),
            Constraint::Comparison(inner) => inner.fmt(f),
            Constraint::Exists(inner) => write!(f, "{}", inner),
        }
    }
}

macro_rules! impl_display_for_binary_operation {
    ($ty:ident, $lhs:ident, $rhs:ident, $op:expr) => {
        impl Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({} {} {})", &self.$lhs, $op, &self.$rhs)
            }
        }
    };
}

impl_display_for_binary_operation!(ConstraintAnd, lhs, rhs, "&&");
impl_display_for_binary_operation!(ConstraintOr, lhs, rhs, "||");
impl_display_for_binary_operation!(ConstraintSemi, lhs, rhs, ";");

impl Display for ConstraintComparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}{} {}",
            self.lhs, self.num_type, self.comparison, self.rhs,
        )
    }
}

impl Display for ConstraintRValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstraintRValue::Add(inner) => write!(f, "{} + {}", inner.lhs, inner.rhs),
            ConstraintRValue::Field(inner) => write!(f, "{}", inner),
            ConstraintRValue::Integer(inner) => write!(f, "{}", inner),
        }
    }
}
