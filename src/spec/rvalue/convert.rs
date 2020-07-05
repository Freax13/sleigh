use super::*;

macro_rules! impl_rvalue_from {
    ($i:ident, $o:ident) => {
        impl From<$o> for RValue {
            fn from(val: $o) -> Self {
                RValue::$i(val.into())
            }
        }
    };
}

impl_rvalue_from!(Add, RValueAdd);
impl_rvalue_from!(Sub, RValueSub);
impl_rvalue_from!(Mult, RValueMult);
impl_rvalue_from!(Div, RValueDiv);
impl_rvalue_from!(Rem, RValueRem);
impl_rvalue_from!(IntOr, RValueIntOr);
impl_rvalue_from!(IntAnd, RValueIntAnd);
impl_rvalue_from!(IntXor, RValueIntXor);
impl_rvalue_from!(BoolOr, RValueBoolOr);
impl_rvalue_from!(BoolAnd, RValueBoolAnd);
impl_rvalue_from!(BoolXor, RValueBoolXor);
impl_rvalue_from!(RShift, RValueRShift);
impl_rvalue_from!(LShift, RValueLShift);
impl_rvalue_from!(Comparison, RValueComparison);
impl_rvalue_from!(Not, RValueNot);
impl_rvalue_from!(Neg, RValueNeg);
impl_rvalue_from!(Parenthesized, RValueParenthesized);
impl_rvalue_from!(Constant, RValueConstant);
impl_rvalue_from!(Call, RValueCall);
impl_rvalue_from!(Ref, RValueRef);
impl_rvalue_from!(Deref, RValueDeref);
impl_rvalue_from!(LValue, LValue);
