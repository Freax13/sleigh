use super::Fix;
use crate::spec::{lvalue::*, rvalue::*};
use crate::{impl_for_binary_operation, impl_for_terminal, impl_for_unary_operation};

impl Fix<RValue> for RValue {
    fn fix(&self) -> RValue {
        match self {
            RValue::Add(inner) => inner.fix(),
            RValue::Sub(inner) => inner.fix(),
            RValue::Mult(inner) => inner.fix(),
            RValue::Div(inner) => inner.fix(),
            RValue::Rem(inner) => inner.fix(),
            RValue::IntOr(inner) => inner.fix(),
            RValue::IntAnd(inner) => inner.fix(),
            RValue::IntXor(inner) => inner.fix(),
            RValue::BoolOr(inner) => inner.fix(),
            RValue::BoolAnd(inner) => inner.fix(),
            RValue::BoolXor(inner) => inner.fix(),
            RValue::RShift(inner) => inner.fix(),
            RValue::LShift(inner) => inner.fix(),
            RValue::Comparison(inner) => inner.fix(),
            RValue::Not(inner) => inner.fix(),
            RValue::Neg(inner) => inner.fix(),
            RValue::Parenthesized(inner) => inner.fix(),
            RValue::Constant(inner) => inner.fix(),
            RValue::Call(inner) => inner.fix(),
            RValue::Ref(inner) => inner.fix(),
            RValue::Deref(inner) => inner.fix(),
            RValue::LValue(inner) => inner.fix(),
        }
    }

    fn set_left(&self, left: impl FnOnce(RValue) -> RValue) -> RValue {
        match self {
            RValue::Add(inner) => inner.set_left(left),
            RValue::Sub(inner) => inner.set_left(left),
            RValue::Mult(inner) => inner.set_left(left),
            RValue::Div(inner) => inner.set_left(left),
            RValue::Rem(inner) => inner.set_left(left),
            RValue::IntOr(inner) => inner.set_left(left),
            RValue::IntAnd(inner) => inner.set_left(left),
            RValue::IntXor(inner) => inner.set_left(left),
            RValue::BoolOr(inner) => inner.set_left(left),
            RValue::BoolAnd(inner) => inner.set_left(left),
            RValue::BoolXor(inner) => inner.set_left(left),
            RValue::RShift(inner) => inner.set_left(left),
            RValue::LShift(inner) => inner.set_left(left),
            RValue::Comparison(inner) => inner.set_left(left),
            RValue::Not(inner) => inner.set_left(left),
            RValue::Neg(inner) => inner.set_left(left),
            RValue::Parenthesized(inner) => inner.set_left(left),
            RValue::Constant(inner) => inner.set_left(left),
            RValue::Call(inner) => inner.set_left(left),
            RValue::Ref(inner) => inner.set_left(left),
            RValue::Deref(inner) => inner.set_left(left),
            RValue::LValue(inner) => inner.set_left(left),
        }
    }

    fn set_right(&self, right: impl FnOnce(RValue) -> RValue) -> RValue {
        match self {
            RValue::Add(inner) => inner.set_right(right),
            RValue::Sub(inner) => inner.set_right(right),
            RValue::Mult(inner) => inner.set_right(right),
            RValue::Div(inner) => inner.set_right(right),
            RValue::Rem(inner) => inner.set_right(right),
            RValue::IntOr(inner) => inner.set_right(right),
            RValue::IntAnd(inner) => inner.set_right(right),
            RValue::IntXor(inner) => inner.set_right(right),
            RValue::BoolOr(inner) => inner.set_right(right),
            RValue::BoolAnd(inner) => inner.set_right(right),
            RValue::BoolXor(inner) => inner.set_right(right),
            RValue::RShift(inner) => inner.set_right(right),
            RValue::LShift(inner) => inner.set_right(right),
            RValue::Comparison(inner) => inner.set_right(right),
            RValue::Not(inner) => inner.set_right(right),
            RValue::Neg(inner) => inner.set_right(right),
            RValue::Parenthesized(inner) => inner.set_right(right),
            RValue::Constant(inner) => inner.set_right(right),
            RValue::Call(inner) => inner.set_right(right),
            RValue::Ref(inner) => inner.set_right(right),
            RValue::Deref(inner) => inner.set_right(right),
            RValue::LValue(inner) => inner.set_right(right),
        }
    }

    fn precedence(&self) -> u8 {
        match self {
            RValue::Add(inner) => inner.precedence(),
            RValue::Sub(inner) => inner.precedence(),
            RValue::Mult(inner) => inner.precedence(),
            RValue::Div(inner) => inner.precedence(),
            RValue::Rem(inner) => inner.precedence(),
            RValue::IntOr(inner) => inner.precedence(),
            RValue::IntAnd(inner) => inner.precedence(),
            RValue::IntXor(inner) => inner.precedence(),
            RValue::BoolOr(inner) => inner.precedence(),
            RValue::BoolAnd(inner) => inner.precedence(),
            RValue::BoolXor(inner) => inner.precedence(),
            RValue::RShift(inner) => inner.precedence(),
            RValue::LShift(inner) => inner.precedence(),
            RValue::Comparison(inner) => inner.precedence(),
            RValue::Not(inner) => inner.precedence(),
            RValue::Neg(inner) => inner.precedence(),
            RValue::Parenthesized(inner) => inner.precedence(),
            RValue::Constant(inner) => inner.precedence(),
            RValue::Call(inner) => inner.precedence(),
            RValue::Ref(inner) => inner.precedence(),
            RValue::Deref(inner) => inner.precedence(),
            RValue::LValue(inner) => inner.precedence(),
        }
    }
}

impl_for_binary_operation!(RValue, RValueMult, lhs, rhs, 5);
impl_for_binary_operation!(RValue, RValueDiv, lhs, rhs, 5);
impl_for_binary_operation!(RValue, RValueRem, lhs, rhs, 5);
impl_for_binary_operation!(RValue, RValueAdd, lhs, rhs, 6);
impl_for_binary_operation!(RValue, RValueSub, lhs, rhs, 6);
impl_for_binary_operation!(RValue, RValueRShift, lhs, rhs, 6);
impl_for_binary_operation!(RValue, RValueLShift, lhs, rhs, 6);
impl_for_binary_operation!(RValue, RValueComparison, lhs, rhs, 9);
impl_for_binary_operation!(RValue, RValueIntAnd, lhs, rhs, 11);
impl_for_binary_operation!(RValue, RValueIntXor, lhs, rhs, 12);
impl_for_binary_operation!(RValue, RValueIntOr, lhs, rhs, 13);
impl_for_binary_operation!(RValue, RValueBoolAnd, lhs, rhs, 14);
impl_for_binary_operation!(RValue, RValueBoolXor, lhs, rhs, 15);
impl_for_binary_operation!(RValue, RValueBoolOr, lhs, rhs, 16);

impl_for_unary_operation!(RValue, RValueNot, op, 3, false);
impl_for_unary_operation!(RValue, RValueNeg, op, 3, false);
impl_for_unary_operation!(RValue, RValueDeref, op, 3, false);

impl_for_terminal!(RValue, RValueParenthesized);
impl_for_terminal!(RValue, RValueCall);
impl_for_terminal!(RValue, RValueConstant);
impl_for_terminal!(RValue, RValueRef);
impl_for_terminal!(RValue, LValue);

#[cfg(test)]
mod tests {
    use crate::{
        spec::precedence::fix_precedence, RValue, RValueConstant, RValueIntAnd, RValueIntOr,
    };

    #[test]
    fn test_precedence() {
        const A: RValueConstant = RValueConstant {
            value: 1,
            size: None,
        };
        const B: RValueConstant = RValueConstant {
            value: 2,
            size: None,
        };
        const C: RValueConstant = RValueConstant {
            value: 3,
            size: None,
        };

        assert_eq!(
            fix_precedence(RValue::IntAnd(Box::new(RValueIntAnd {
                lhs: RValue::Constant(A),
                rhs: RValue::IntOr(Box::new(RValueIntOr {
                    lhs: RValue::Constant(B),
                    rhs: RValue::Constant(C)
                }))
            }))),
            RValue::IntOr(Box::new(RValueIntOr {
                lhs: RValue::IntAnd(Box::new(RValueIntAnd {
                    lhs: RValue::Constant(A),
                    rhs: RValue::Constant(B)
                })),
                rhs: RValue::Constant(C),
            }))
        );
    }
}
