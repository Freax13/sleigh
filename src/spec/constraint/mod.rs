mod convert;
mod debug;

use crate::{ComparisonOperator, NumTypePrefix};

#[derive(Clone, PartialEq)]
pub enum Constraint {
    Ellipsis(Box<ConstraintEllipsis>),
    And(Box<ConstraintAnd>),
    Or(Box<ConstraintOr>),
    Semi(Box<ConstraintSemi>),
    Parenthesized(Box<Constraint>),
    Comparison(ConstraintComparison),
    Exists(String),
}

#[derive(Clone, PartialEq)]
pub struct ConstraintEllipsis {
    pub op: Constraint,
}

#[derive(Clone, PartialEq)]
pub struct ConstraintAnd {
    pub lhs: Constraint,
    pub rhs: Constraint,
}

#[derive(Clone, PartialEq)]
pub struct ConstraintOr {
    pub lhs: Constraint,
    pub rhs: Constraint,
}

#[derive(Clone, PartialEq)]
pub struct ConstraintSemi {
    pub lhs: Constraint,
    pub rhs: Constraint,
}

#[derive(Clone, PartialEq)]
pub struct ConstraintComparison {
    pub lhs: ConstraintRValue,
    pub num_type: NumTypePrefix,
    pub comparison: ComparisonOperator,
    pub rhs: ConstraintRValue,
}

#[derive(Clone, PartialEq)]
pub enum ConstraintRValue {
    Add(Box<ConstraintRValueAdd>),
    Field(String),
    Integer(i128),
}

#[derive(Clone, PartialEq)]
pub struct ConstraintRValueAdd {
    pub lhs: ConstraintRValue,
    pub rhs: ConstraintRValue,
}
