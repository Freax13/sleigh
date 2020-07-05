use crate::spec::{ComparisonOperator, NumTypePrefix};

#[derive(Clone)]
pub enum Constraint {
    Ellipsis(Box<Constraint>),
    And(Box<ConstraintAnd>),
    Or(Box<ConstraintOr>),
    Semi(Box<ConstraintSemi>),
    Parenthesized(Box<Constraint>),
    Comparison(ConstraintComparison),
    Exists(String),
}

#[derive(Clone)]
pub struct ConstraintAnd {
    pub lhs: Constraint,
    pub rhs: Constraint,
}

#[derive(Clone)]
pub struct ConstraintOr {
    pub lhs: Constraint,
    pub rhs: Constraint,
}

#[derive(Clone)]
pub struct ConstraintSemi {
    pub lhs: Constraint,
    pub rhs: Constraint,
}

#[derive(Clone)]
pub struct ConstraintComparison {
    pub lhs: String,
    pub num_type: NumTypePrefix,
    pub comparison: ComparisonOperator,
    pub rhs: ConstraintRValue,
}

#[derive(Clone)]
pub enum ConstraintRValue {
    Add(Box<ConstraintRValueAdd>),
    Field(String),
    Integer(i128),
}

#[derive(Clone)]
pub struct ConstraintRValueAdd {
    pub lhs: ConstraintRValue,
    pub rhs: ConstraintRValue,
}
