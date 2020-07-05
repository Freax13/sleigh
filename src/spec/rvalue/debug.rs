use super::*;
use crate::NumTypePrefix;
use std::fmt::{self, Debug, Display};

impl Debug for RValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for RValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        macro_rules! forward_fmt {
            ($ty:ident) => {
                if let RValue::$ty(inner) = self {
                    return inner.fmt(f);
                }
            };
        }
        forward_fmt!(Add);
        forward_fmt!(Sub);
        forward_fmt!(Mult);
        forward_fmt!(Div);
        forward_fmt!(Rem);
        forward_fmt!(IntOr);
        forward_fmt!(IntAnd);
        forward_fmt!(IntXor);
        forward_fmt!(BoolOr);
        forward_fmt!(BoolAnd);
        forward_fmt!(BoolXor);
        forward_fmt!(RShift);
        forward_fmt!(LShift);
        forward_fmt!(Comparison);
        forward_fmt!(Not);
        forward_fmt!(Neg);
        forward_fmt!(Parenthesized);
        forward_fmt!(Constant);
        forward_fmt!(Call);
        forward_fmt!(Ref);
        forward_fmt!(Deref);
        forward_fmt!(LValue);
        unreachable!();
    }
}

impl Display for NumTypePrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumTypePrefix::Default => Ok(()),
            NumTypePrefix::Signed => write!(f, "s"),
            NumTypePrefix::Float => write!(f, "f"),
        }
    }
}

impl Display for ComparisonOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComparisonOperator::Equal => write!(f, "=="),
            ComparisonOperator::NotEqual => write!(f, "!="),
            ComparisonOperator::Less => write!(f, "<"),
            ComparisonOperator::LessEqual => write!(f, "<="),
            ComparisonOperator::Greater => write!(f, ">"),
            ComparisonOperator::GreaterEqual => write!(f, ">="),
        }
    }
}

macro_rules! impl_debug_for_binary_operation {
    ($ty:ident, $lhs:ident, $rhs:ident, $op:expr) => {
        impl Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({} {} {})", &self.$lhs, $op, &self.$rhs)
            }
        }
    };
}

macro_rules! impl_debug_for_binary_operation_with_prefix {
    ($ty:ident, $lhs:ident, $rhs:ident, $ntp:ident, $op:expr) => {
        impl Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({} {}{} {})", &self.$lhs, &self.$ntp, $op, &self.$rhs)
            }
        }
    };
}

impl_debug_for_binary_operation_with_prefix!(RValueAdd, lhs, rhs, num_type_prefix, "+");
impl_debug_for_binary_operation_with_prefix!(RValueSub, lhs, rhs, num_type_prefix, "-");
impl_debug_for_binary_operation_with_prefix!(RValueMult, lhs, rhs, num_type_prefix, "*");
impl_debug_for_binary_operation_with_prefix!(RValueDiv, lhs, rhs, num_type_prefix, "/");
impl_debug_for_binary_operation_with_prefix!(RValueRem, lhs, rhs, num_type_prefix, "%");
impl_debug_for_binary_operation!(RValueIntOr, lhs, rhs, "|");
impl_debug_for_binary_operation!(RValueIntAnd, lhs, rhs, "&");
impl_debug_for_binary_operation!(RValueIntXor, lhs, rhs, "^");
impl_debug_for_binary_operation!(RValueBoolOr, lhs, rhs, "||");
impl_debug_for_binary_operation!(RValueBoolAnd, lhs, rhs, "&&");
impl_debug_for_binary_operation!(RValueBoolXor, lhs, rhs, "^^");
impl_debug_for_binary_operation_with_prefix!(RValueRShift, lhs, rhs, num_type_prefix, ">>");
impl_debug_for_binary_operation!(RValueLShift, lhs, rhs, "<<");

impl Display for RValueComparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({} {}{} {})",
            self.lhs, self.num_type_prefix, self.operator, self.rhs
        )
    }
}

macro_rules! impl_debug_for_unary_operation_with_prefix {
    ($ty:ident, $inner:ident, $op:expr) => {
        impl Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({}{})", &self.$inner, $op)
            }
        }
    };
}

impl_debug_for_unary_operation_with_prefix!(RValueNot, op, "!");
impl_debug_for_unary_operation_with_prefix!(RValueNeg, op, "-");

impl Display for RValueParenthesized {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", &self.op)
    }
}

impl Display for RValueConstant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.value)?;
        if let Some(size) = self.size {
            write!(f, ":{}", size)
        } else {
            Ok(())
        }
    }
}

impl Display for RValueCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(", self.call)?;
        for (i, arg) in self.args.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", arg)?;
        }
        write!(f, ")")
    }
}

impl Display for RValueRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ":")?;
        if let Some(size) = self.size {
            write!(f, ":{}", size)?;
        }
        write!(f, "{}", self.field)
    }
}

impl Display for RValueDeref {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.op)
    }
}

impl Display for LValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LValue::Ident(inner) => {
                write!(f, "{}", inner.field)?;
                if let Some(size) = inner.size {
                    write!(f, ":{}", size)
                } else {
                    Ok(())
                }
            }
            LValue::Slice(inner) => write!(f, "{}[{}, {}]", inner.field, inner.offset, inner.size),
            LValue::Ref(inner) => {
                write!(f, "*")?;
                if let Some(space) = inner.space.as_ref() {
                    write!(f, "[{}]", space)?;
                }
                if let Some(size) = inner.size {
                    write!(f, ":{}", size)?;
                }
                write!(f, "{}", inner.op)
            }
        }
    }
}
