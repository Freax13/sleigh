use super::*;

macro_rules! impl_constraint_from {
    ($i:ident, $o:ty) => {
        impl From<$o> for Constraint {
            fn from(val: $o) -> Self {
                Constraint::$i(val.into())
            }
        }
    };
}

impl_constraint_from!(Ellipsis, ConstraintEllipsis);
impl_constraint_from!(And, ConstraintAnd);
impl_constraint_from!(Or, ConstraintOr);
impl_constraint_from!(Semi, ConstraintSemi);
impl_constraint_from!(Parenthesized, Box<Constraint>);
impl_constraint_from!(Comparison, ConstraintComparison);
