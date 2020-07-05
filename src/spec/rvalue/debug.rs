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
        match self {
            RValue::Add(inner) => inner.fmt(f),
            RValue::Sub(inner) => inner.fmt(f),
            RValue::Mult(inner) => inner.fmt(f),
            RValue::Div(inner) => inner.fmt(f),
            RValue::Rem(inner) => inner.fmt(f),
            RValue::IntOr(inner) => inner.fmt(f),
            RValue::IntAnd(inner) => inner.fmt(f),
            RValue::IntXor(inner) => inner.fmt(f),
            RValue::BoolOr(inner) => inner.fmt(f),
            RValue::BoolAnd(inner) => inner.fmt(f),
            RValue::BoolXor(inner) => inner.fmt(f),
            RValue::RShift(inner) => inner.fmt(f),
            RValue::LShift(inner) => inner.fmt(f),
            RValue::Comparison(inner) => inner.fmt(f),
            RValue::Not(inner) => inner.fmt(f),
            RValue::Neg(inner) => inner.fmt(f),
            RValue::Parenthesized(inner) => inner.fmt(f),
            RValue::Constant(inner) => inner.fmt(f),
            RValue::Call(inner) => inner.fmt(f),
            RValue::Ref(inner) => inner.fmt(f),
            RValue::Deref(inner) => inner.fmt(f),
            RValue::LValue(inner) => inner.fmt(f),
        }
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

macro_rules! impl_display_for_binary_operation {
    ($ty:ident, $lhs:ident, $rhs:ident, $op:expr) => {
        impl Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({} {} {})", &self.$lhs, $op, &self.$rhs)
            }
        }
    };
}

macro_rules! impl_display_for_binary_operation_with_prefix {
    ($ty:ident, $lhs:ident, $rhs:ident, $ntp:ident, $op:expr) => {
        impl Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({} {}{} {})", &self.$lhs, &self.$ntp, $op, &self.$rhs)
            }
        }
    };
}

impl_display_for_binary_operation_with_prefix!(RValueAdd, lhs, rhs, num_type_prefix, "+");
impl_display_for_binary_operation_with_prefix!(RValueSub, lhs, rhs, num_type_prefix, "-");
impl_display_for_binary_operation_with_prefix!(RValueMult, lhs, rhs, num_type_prefix, "*");
impl_display_for_binary_operation_with_prefix!(RValueDiv, lhs, rhs, num_type_prefix, "/");
impl_display_for_binary_operation_with_prefix!(RValueRem, lhs, rhs, num_type_prefix, "%");
impl_display_for_binary_operation!(RValueIntOr, lhs, rhs, "|");
impl_display_for_binary_operation!(RValueIntAnd, lhs, rhs, "&");
impl_display_for_binary_operation!(RValueIntXor, lhs, rhs, "^");
impl_display_for_binary_operation!(RValueBoolOr, lhs, rhs, "||");
impl_display_for_binary_operation!(RValueBoolAnd, lhs, rhs, "&&");
impl_display_for_binary_operation!(RValueBoolXor, lhs, rhs, "^^");
impl_display_for_binary_operation_with_prefix!(RValueRShift, lhs, rhs, num_type_prefix, ">>");
impl_display_for_binary_operation!(RValueLShift, lhs, rhs, "<<");

impl Display for RValueComparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({} {}{} {})",
            self.lhs, self.num_type_prefix, self.operator, self.rhs
        )
    }
}

macro_rules! impl_display_for_unary_operation_with_prefix {
    ($ty:ident, $inner:ident, $op:expr) => {
        impl Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({}{})", &self.$inner, $op)
            }
        }
    };
}

impl_display_for_unary_operation_with_prefix!(RValueNot, op, "!");
impl_display_for_unary_operation_with_prefix!(RValueNeg, op, "-");

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
