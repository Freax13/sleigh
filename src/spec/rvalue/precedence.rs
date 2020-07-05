use super::*;

pub fn fix_precedence(mut value: RValue) -> RValue {
    loop {
        let next = value.fix();
        if value != next {
            value = next;
        } else {
            return value;
        }
    }
}

trait Fix: PartialEq {
    fn fix(&self) -> RValue;
    fn set_left(&self, left: impl FnOnce(RValue) -> RValue) -> RValue;
    fn set_right(&self, right: impl FnOnce(RValue) -> RValue) -> RValue;
    // https://en.cppreference.com/w/cpp/language/operator_precedence
    fn precedence(&self) -> u8;
}

impl Fix for RValue {
    fn fix(&self) -> RValue {
        macro_rules! forward_fix {
            ($ty:ident) => {
                if let RValue::$ty(inner) = self {
                    return inner.fix();
                }
            };
        }
        forward_fix!(Add);
        forward_fix!(Sub);
        forward_fix!(Mult);
        forward_fix!(Div);
        forward_fix!(Rem);
        forward_fix!(IntOr);
        forward_fix!(IntAnd);
        forward_fix!(IntXor);
        forward_fix!(BoolOr);
        forward_fix!(BoolAnd);
        forward_fix!(BoolXor);
        forward_fix!(RShift);
        forward_fix!(LShift);
        forward_fix!(Comparison);
        forward_fix!(Not);
        forward_fix!(Neg);
        forward_fix!(Parenthesized);
        forward_fix!(Constant);
        forward_fix!(Call);
        forward_fix!(Ref);
        forward_fix!(Deref);
        forward_fix!(LValue);
        unreachable!();
    }

    fn set_left(&self, left: impl FnOnce(RValue) -> RValue) -> RValue {
        macro_rules! forward_set_left {
            ($ty:ident) => {
                if let RValue::$ty(inner) = self {
                    return inner.set_left(left);
                }
            };
        }
        forward_set_left!(Add);
        forward_set_left!(Sub);
        forward_set_left!(Mult);
        forward_set_left!(Div);
        forward_set_left!(Rem);
        forward_set_left!(IntOr);
        forward_set_left!(IntAnd);
        forward_set_left!(IntXor);
        forward_set_left!(BoolOr);
        forward_set_left!(BoolAnd);
        forward_set_left!(BoolXor);
        forward_set_left!(RShift);
        forward_set_left!(LShift);
        forward_set_left!(Comparison);
        forward_set_left!(Not);
        forward_set_left!(Neg);
        forward_set_left!(Parenthesized);
        forward_set_left!(Constant);
        forward_set_left!(Call);
        forward_set_left!(Ref);
        forward_set_left!(Deref);
        forward_set_left!(LValue);
        unreachable!();
    }

    fn set_right(&self, right: impl FnOnce(RValue) -> RValue) -> RValue {
        macro_rules! forward_set_right {
            ($ty:ident) => {
                if let RValue::$ty(inner) = self {
                    return inner.set_right(right);
                }
            };
        }
        forward_set_right!(Add);
        forward_set_right!(Sub);
        forward_set_right!(Mult);
        forward_set_right!(Div);
        forward_set_right!(Rem);
        forward_set_right!(IntOr);
        forward_set_right!(IntAnd);
        forward_set_right!(IntXor);
        forward_set_right!(BoolOr);
        forward_set_right!(BoolAnd);
        forward_set_right!(BoolXor);
        forward_set_right!(RShift);
        forward_set_right!(LShift);
        forward_set_right!(Comparison);
        forward_set_right!(Not);
        forward_set_right!(Neg);
        forward_set_right!(Parenthesized);
        forward_set_right!(Constant);
        forward_set_right!(Call);
        forward_set_right!(Ref);
        forward_set_right!(Deref);
        forward_set_right!(LValue);
        unreachable!();
    }

    fn precedence(&self) -> u8 {
        macro_rules! forward_precedence {
            ($ty:ident) => {
                if let RValue::$ty(inner) = self {
                    return inner.precedence();
                }
            };
        }
        forward_precedence!(Add);
        forward_precedence!(Sub);
        forward_precedence!(Mult);
        forward_precedence!(Div);
        forward_precedence!(Rem);
        forward_precedence!(IntOr);
        forward_precedence!(IntAnd);
        forward_precedence!(IntXor);
        forward_precedence!(BoolOr);
        forward_precedence!(BoolAnd);
        forward_precedence!(BoolXor);
        forward_precedence!(RShift);
        forward_precedence!(LShift);
        forward_precedence!(Comparison);
        forward_precedence!(Not);
        forward_precedence!(Neg);
        forward_precedence!(Parenthesized);
        forward_precedence!(Constant);
        forward_precedence!(Call);
        forward_precedence!(Ref);
        forward_precedence!(Deref);
        forward_precedence!(LValue);
        unreachable!();
    }
}

macro_rules! impl_for_binary_operation {
    ($ty:ident,$lhs:ident,$rhs:ident,$p:expr) => {
        impl Fix for $ty {
            fn fix(&self) -> RValue {
                if $p < self.$lhs.precedence() {
                    self.$lhs.set_right(|right| {
                        let mut cloned = self.clone();
                        cloned.$lhs = right;
                        cloned.into()
                    })
                } else if $p < self.$rhs.precedence() {
                    self.$rhs.set_left(|left| {
                        let mut cloned = self.clone();
                        cloned.$rhs = left;
                        cloned.into()
                    })
                } else {
                    let mut cloned = self.clone();
                    cloned.$lhs = cloned.$lhs.fix();
                    cloned.$rhs = cloned.$rhs.fix();
                    cloned.into()
                }
            }

            fn set_left(&self, left: impl FnOnce(RValue) -> RValue) -> RValue {
                let mut cloned = self.clone();
                let left = left(cloned.$lhs.clone());
                cloned.$lhs = left;
                cloned.into()
            }

            fn set_right(&self, right: impl FnOnce(RValue) -> RValue) -> RValue {
                let mut cloned = self.clone();
                let right = right(cloned.$rhs.clone());
                cloned.$rhs = right;
                cloned.into()
            }

            fn precedence(&self) -> u8 {
                $p
            }
        }
    };
}

impl_for_binary_operation!(RValueMult, lhs, rhs, 5);
impl_for_binary_operation!(RValueDiv, lhs, rhs, 5);
impl_for_binary_operation!(RValueRem, lhs, rhs, 5);
impl_for_binary_operation!(RValueAdd, lhs, rhs, 6);
impl_for_binary_operation!(RValueSub, lhs, rhs, 6);
impl_for_binary_operation!(RValueRShift, lhs, rhs, 6);
impl_for_binary_operation!(RValueLShift, lhs, rhs, 6);
impl_for_binary_operation!(RValueComparison, lhs, rhs, 9);
impl_for_binary_operation!(RValueIntAnd, lhs, rhs, 11);
impl_for_binary_operation!(RValueIntXor, lhs, rhs, 12);
impl_for_binary_operation!(RValueIntOr, lhs, rhs, 13);
impl_for_binary_operation!(RValueBoolAnd, lhs, rhs, 14);
impl_for_binary_operation!(RValueBoolXor, lhs, rhs, 15);
impl_for_binary_operation!(RValueBoolOr, lhs, rhs, 16);

macro_rules! impl_for_unary_operation {
    ($ty:ident,$inner:ident,$p:expr,$l:expr) => {
        impl Fix for $ty {
            fn fix(&self) -> RValue {
                let inner = &self.$inner;
                if $p < inner.precedence() {
                    let update = |new| {
                        let mut cloned = self.clone();
                        cloned.$inner = new;
                        cloned.into()
                    };
                    if $l {
                        inner.set_right(update)
                    } else {
                        inner.set_left(update)
                    }
                } else {
                    let mut cloned = self.clone();
                    cloned.$inner = inner.fix();
                    cloned.into()
                }
            }

            fn set_left(&self, left: impl FnOnce(RValue) -> RValue) -> RValue {
                let mut cloned = self.clone();
                let left = left(cloned.$inner.clone());
                cloned.$inner = left;
                cloned.into()
            }

            fn set_right(&self, right: impl FnOnce(RValue) -> RValue) -> RValue {
                self.set_left(right)
            }

            fn precedence(&self) -> u8 {
                $p
            }
        }
    };
}

impl_for_unary_operation!(RValueNot, op, 3, false);
impl_for_unary_operation!(RValueNeg, op, 3, false);
impl_for_unary_operation!(RValueDeref, op, 3, false);

macro_rules! impl_for_terminal {
    ($ty:ident,$r:ident) => {
        impl Fix for $ty {
            fn fix(&self) -> RValue {
                self.clone().into()
            }

            fn set_left(&self, _: impl FnOnce(RValue) -> RValue) -> RValue {
                unreachable!()
            }

            fn set_right(&self, _: impl FnOnce(RValue) -> RValue) -> RValue {
                unreachable!()
            }

            fn precedence(&self) -> u8 {
                0
            }
        }
    };
}

impl_for_terminal!(RValueParenthesized, Parenthesized);
impl_for_terminal!(RValueCall, Call);
impl_for_terminal!(RValueConstant, Constant);
impl_for_terminal!(RValueRef, Ref);
impl_for_terminal!(LValue, LValue);

#[cfg(test)]
mod tests {
    use super::fix_precedence;
    use crate::{RValue, RValueConstant, RValueIntAnd, RValueIntOr};

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
