mod constraint;
mod rvalue;

use super::*;

pub fn fix_precedence_rvalue(value: RValue) -> RValue {
    fix_precedence(value)
}

pub fn fix_precedence_constraint(value: Constraint) -> Constraint {
    fix_precedence(value)
}

fn fix_precedence<T: Fix<T>>(mut value: T) -> T {
    loop {
        let next = value.fix();
        if value != next {
            value = next;
        } else {
            return value;
        }
    }
}

trait Fix<T>: PartialEq {
    fn fix(&self) -> T;
    fn set_left(&self, left: impl FnOnce(T) -> T) -> T;
    fn set_right(&self, right: impl FnOnce(T) -> T) -> T;
    // https://en.cppreference.com/w/cpp/language/operator_precedence
    fn precedence(&self) -> u8;
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_for_binary_operation {
    ($base:ty,$ty:ident,$lhs:ident,$rhs:ident,$p:expr) => {
        impl Fix<$base> for $ty {
            fn fix(&self) -> $base {
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

            fn set_left(&self, left: impl FnOnce($base) -> $base) -> $base {
                let mut cloned = self.clone();
                let left = left(cloned.$lhs.clone());
                cloned.$lhs = left;
                cloned.into()
            }

            fn set_right(&self, right: impl FnOnce($base) -> $base) -> $base {
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

#[doc(hidden)]
#[macro_export]
macro_rules! impl_for_unary_operation {
    ($base:ty,$ty:ident,$inner:ident,$p:expr,$l:expr) => {
        impl Fix<$base> for $ty {
            fn fix(&self) -> $base {
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

            fn set_left(&self, left: impl FnOnce($base) -> $base) -> $base {
                let mut cloned = self.clone();
                let left = left(cloned.$inner.clone());
                cloned.$inner = left;
                cloned.into()
            }

            fn set_right(&self, right: impl FnOnce($base) -> $base) -> $base {
                self.set_left(right)
            }

            fn precedence(&self) -> u8 {
                $p
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_for_terminal {
    ($base:ty,$ty:ident) => {
        impl Fix<$base> for $ty {
            fn fix(&self) -> $base {
                self.clone().into()
            }

            fn set_left(&self, _: impl FnOnce($base) -> $base) -> $base {
                unreachable!()
            }

            fn set_right(&self, _: impl FnOnce($base) -> $base) -> $base {
                unreachable!()
            }

            fn precedence(&self) -> u8 {
                0
            }
        }
    };
}
