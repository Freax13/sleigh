mod convert;
mod debug;
mod precedence;

pub use precedence::fix_precedence;

use crate::{ComparisonOperator, LValue, NumTypePrefix};

#[derive(Clone, PartialEq)]
pub enum RValue {
    Add(Box<RValueAdd>),
    Sub(Box<RValueSub>),
    Mult(Box<RValueMult>),
    Div(Box<RValueDiv>),
    Rem(Box<RValueRem>),
    IntOr(Box<RValueIntOr>),
    IntAnd(Box<RValueIntAnd>),
    IntXor(Box<RValueIntXor>),
    BoolOr(Box<RValueBoolOr>),
    BoolAnd(Box<RValueBoolAnd>),
    BoolXor(Box<RValueBoolXor>),
    RShift(Box<RValueRShift>),
    LShift(Box<RValueLShift>),
    Comparison(Box<RValueComparison>),
    Not(Box<RValueNot>),
    Neg(Box<RValueNeg>),
    Parenthesized(Box<RValueParenthesized>),
    Constant(RValueConstant),
    Call(RValueCall),
    Ref(RValueRef),
    Deref(Box<RValueDeref>),
    LValue(LValue),
}

#[derive(Clone, PartialEq)]
pub struct RValueAdd {
    pub lhs: RValue,
    pub num_type_prefix: NumTypePrefix,
    pub rhs: RValue,
}

#[derive(Clone, PartialEq)]
pub struct RValueSub {
    pub lhs: RValue,
    pub num_type_prefix: NumTypePrefix,
    pub rhs: RValue,
}

#[derive(Clone, PartialEq)]
pub struct RValueMult {
    pub lhs: RValue,
    pub num_type_prefix: NumTypePrefix,
    pub rhs: RValue,
}

#[derive(Clone, PartialEq)]
pub struct RValueDiv {
    pub lhs: RValue,
    pub num_type_prefix: NumTypePrefix,
    pub rhs: RValue,
}

#[derive(Clone, PartialEq)]
pub struct RValueRem {
    pub lhs: RValue,
    pub num_type_prefix: NumTypePrefix,
    pub rhs: RValue,
}

#[derive(Clone, PartialEq)]
pub struct RValueIntOr {
    pub lhs: RValue,
    pub rhs: RValue,
}

#[derive(Clone, PartialEq)]
pub struct RValueIntAnd {
    pub lhs: RValue,
    pub rhs: RValue,
}

#[derive(Clone, PartialEq)]
pub struct RValueIntXor {
    pub lhs: RValue,
    pub rhs: RValue,
}

#[derive(Clone, PartialEq)]
pub struct RValueRShift {
    pub lhs: RValue,
    pub num_type_prefix: NumTypePrefix,
    pub rhs: RValue,
}

#[derive(Clone, PartialEq)]
pub struct RValueLShift {
    pub lhs: RValue,
    pub rhs: RValue,
}

#[derive(Clone, PartialEq)]
pub struct RValueNot {
    pub op: RValue,
}

#[derive(Clone, PartialEq)]
pub struct RValueNeg {
    pub op: RValue,
}

#[derive(Clone, PartialEq)]
pub struct RValueParenthesized {
    pub op: RValue,
}

#[derive(Copy, Clone, PartialEq)]
pub struct RValueConstant {
    pub value: i128,
    pub size: Option<u8>,
}

#[derive(Clone, PartialEq)]
pub struct RValueCall {
    pub call: String,
    pub args: Vec<RValue>,
}

#[derive(Clone, PartialEq)]
pub struct RValueRef {
    pub field: String,
    pub size: Option<u8>,
}

#[derive(Clone, PartialEq)]
pub struct RValueDeref {
    pub op: RValue,
}

#[derive(Clone, PartialEq)]
pub struct RValueBoolOr {
    pub lhs: RValue,
    pub rhs: RValue,
}

#[derive(Clone, PartialEq)]
pub struct RValueBoolAnd {
    pub lhs: RValue,
    pub rhs: RValue,
}

#[derive(Clone, PartialEq)]
pub struct RValueBoolXor {
    pub lhs: RValue,
    pub rhs: RValue,
}

#[derive(Clone, PartialEq)]
pub struct RValueComparison {
    pub lhs: RValue,
    pub num_type_prefix: NumTypePrefix,
    pub operator: ComparisonOperator,
    pub rhs: RValue,
}
