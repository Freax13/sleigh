use super::Fix;
use crate::{impl_for_binary_operation, impl_for_unary_operation, spec::constraint::*};

impl Fix<Constraint> for Constraint {
    fn fix(&self) -> Constraint {
        match self {
            Constraint::Ellipsis(inner) => inner.fix(),
            Constraint::And(inner) => inner.fix(),
            Constraint::Or(inner) => inner.fix(),
            Constraint::Semi(inner) => inner.fix(),
            Constraint::Parenthesized(_)
            | Constraint::Comparison(_)
            | Constraint::Exists(_)
            | Constraint::Constructor(_) => self.clone(),
        }
    }

    fn set_left(&self, left: impl FnOnce(Constraint) -> Constraint) -> Constraint {
        match self {
            Constraint::Ellipsis(inner) => inner.set_left(left),
            Constraint::And(inner) => inner.set_left(left),
            Constraint::Or(inner) => inner.set_left(left),
            Constraint::Semi(inner) => inner.set_left(left),
            Constraint::Parenthesized(_)
            | Constraint::Comparison(_)
            | Constraint::Exists(_)
            | Constraint::Constructor(_) => unreachable!(),
        }
    }

    fn set_right(&self, right: impl FnOnce(Constraint) -> Constraint) -> Constraint {
        match self {
            Constraint::Ellipsis(inner) => inner.set_right(right),
            Constraint::And(inner) => inner.set_right(right),
            Constraint::Or(inner) => inner.set_right(right),
            Constraint::Semi(inner) => inner.set_right(right),
            Constraint::Parenthesized(_)
            | Constraint::Comparison(_)
            | Constraint::Exists(_)
            | Constraint::Constructor(_) => unreachable!(),
        }
    }

    fn precedence(&self) -> u8 {
        match self {
            Constraint::Ellipsis(inner) => inner.precedence(),
            Constraint::And(inner) => inner.precedence(),
            Constraint::Or(inner) => inner.precedence(),
            Constraint::Semi(inner) => inner.precedence(),
            Constraint::Parenthesized(_)
            | Constraint::Comparison(_)
            | Constraint::Exists(_)
            | Constraint::Constructor(_) => 0,
        }
    }
}

impl_for_binary_operation!(Constraint, ConstraintAnd, lhs, rhs, 14);
impl_for_binary_operation!(Constraint, ConstraintOr, lhs, rhs, 16);
impl_for_binary_operation!(Constraint, ConstraintSemi, lhs, rhs, 17);

impl_for_unary_operation!(Constraint, ConstraintEllipsis, op, 18, false);

#[cfg(test)]
mod tests {
    use crate::{
        spec::precedence::fix_precedence, Constraint, ConstraintAnd, ConstraintExists, ConstraintOr,
    };

    #[test]
    fn test_precedence() {
        let a: Constraint = Constraint::Exists(ConstraintExists {
            name: "A".to_string(),
        });
        let b: Constraint = Constraint::Exists(ConstraintExists {
            name: "B".to_string(),
        });
        let c: Constraint = Constraint::Exists(ConstraintExists {
            name: "C".to_string(),
        });

        assert_eq!(
            fix_precedence(Constraint::And(Box::new(ConstraintAnd {
                lhs: a.clone(),
                rhs: Constraint::Or(Box::new(ConstraintOr {
                    lhs: b.clone(),
                    rhs: c.clone(),
                }))
            }))),
            Constraint::Or(Box::new(ConstraintOr {
                lhs: Constraint::And(Box::new(ConstraintAnd { lhs: a, rhs: b })),
                rhs: c,
            }))
        );
    }
}
