use crate::spec::{ComparisonOperator, LValue, NumTypePrefix};

#[derive(Clone)]
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

#[derive(Clone)]
pub struct RValueAdd {
    pub lhs: RValue,
    pub num_type_prefix: NumTypePrefix,
    pub rhs: RValue,
}

#[derive(Clone)]
pub struct RValueSub {
    pub lhs: RValue,
    pub num_type_prefix: NumTypePrefix,
    pub rhs: RValue,
}

#[derive(Clone)]
pub struct RValueMult {
    pub lhs: RValue,
    pub num_type_prefix: NumTypePrefix,
    pub rhs: RValue,
}

#[derive(Clone)]
pub struct RValueDiv {
    pub lhs: RValue,
    pub num_type_prefix: NumTypePrefix,
    pub rhs: RValue,
}

#[derive(Clone)]
pub struct RValueRem {
    pub lhs: RValue,
    pub num_type_prefix: NumTypePrefix,
    pub rhs: RValue,
}

#[derive(Clone)]
pub struct RValueIntOr {
    pub lhs: RValue,
    pub rhs: RValue,
}

#[derive(Clone)]
pub struct RValueIntAnd {
    pub lhs: RValue,
    pub rhs: RValue,
}

#[derive(Clone)]
pub struct RValueIntXor {
    pub lhs: RValue,
    pub rhs: RValue,
}

#[derive(Clone)]
pub struct RValueRShift {
    pub lhs: RValue,
    pub num_type_prefix: NumTypePrefix,
    pub rhs: RValue,
}

#[derive(Clone)]
pub struct RValueLShift {
    pub lhs: RValue,
    pub rhs: RValue,
}

#[derive(Clone)]
pub struct RValueNot {
    pub op: RValue,
}

#[derive(Clone)]
pub struct RValueNeg {
    pub op: RValue,
}

#[derive(Clone)]
pub struct RValueParenthesized {
    pub op: RValue,
}

#[derive(Clone)]
pub struct RValueConstant {
    pub value: i128,
    pub size: Option<u8>,
}

#[derive(Clone)]
pub struct RValueCall {
    pub call: String,
    pub args: Vec<RValue>,
}

#[derive(Clone)]
pub struct RValueRef {
    pub field: String,
    pub size: Option<u8>,
}

#[derive(Clone)]
pub struct RValueDeref {
    pub address: RValue,
}

#[derive(Clone)]
pub struct RValueBoolOr {
    pub lhs: RValue,
    pub rhs: RValue,
}

#[derive(Clone)]
pub struct RValueBoolAnd {
    pub lhs: RValue,
    pub rhs: RValue,
}

#[derive(Clone)]
pub struct RValueBoolXor {
    pub lhs: RValue,
    pub rhs: RValue,
}

#[derive(Clone)]
pub struct RValueComparison {
    pub lhs: RValue,
    pub num_type_prefix: NumTypePrefix,
    pub operator: ComparisonOperator,
    pub rhs: RValue,
}
