use super::RValue;
use crate::{Action, ActionGoto, LValue};
use std::collections::HashMap;

fn rename(val: &mut String, args: &HashMap<String, String>) {
    if let Some(new) = args.get(val) {
        *val = new.clone();
    }
}

impl Action {
    pub(in crate::spec) fn rename(&mut self, args: &HashMap<String, String>) {
        match self {
            Action::Label(label) => {
                rename(label, args);
            }
            Action::LocalDecl(inner) => {
                rename(&mut inner.name.field, args);
                inner.val.rename(args);
            }
            Action::Export(inner) => {
                inner.op.rename(args);
            }
            Action::Assignment(inner) => {
                inner.name.rename(args);
                inner.val.rename(args);
            }
            Action::Build(_inner) => {}
            Action::If(inner) => {
                inner.cond.rename(args);
                inner.action.rename(args);
            }
            Action::Goto(ActionGoto::Label(label)) => rename(label, args),
            Action::Goto(ActionGoto::Address(address)) => address.rename(args),
            Action::Macro(inner) => inner.args.iter_mut().for_each(|arg| arg.rename(args)),
            Action::PCodeOp(inner) => inner.args.iter_mut().for_each(|arg| arg.rename(args)),
            Action::Call(inner) => inner.address.rename(args),
            Action::Return(inner) => inner.val.rename(args),
        }
    }
}

impl RValue {
    pub(in crate::spec) fn rename(&mut self, args: &HashMap<String, String>) {
        match self {
            RValue::Add(inner) => {
                inner.lhs.rename(args);
                inner.rhs.rename(args);
            }
            RValue::Sub(inner) => {
                inner.lhs.rename(args);
                inner.rhs.rename(args);
            }
            RValue::Mult(inner) => {
                inner.lhs.rename(args);
                inner.rhs.rename(args);
            }
            RValue::Div(inner) => {
                inner.lhs.rename(args);
                inner.rhs.rename(args);
            }
            RValue::Rem(inner) => {
                inner.lhs.rename(args);
                inner.rhs.rename(args);
            }
            RValue::IntOr(inner) => {
                inner.lhs.rename(args);
                inner.rhs.rename(args);
            }
            RValue::IntAnd(inner) => {
                inner.lhs.rename(args);
                inner.rhs.rename(args);
            }
            RValue::IntXor(inner) => {
                inner.lhs.rename(args);
                inner.rhs.rename(args);
            }
            RValue::BoolOr(inner) => {
                inner.lhs.rename(args);
                inner.rhs.rename(args);
            }
            RValue::BoolAnd(inner) => {
                inner.lhs.rename(args);
                inner.rhs.rename(args);
            }
            RValue::BoolXor(inner) => {
                inner.lhs.rename(args);
                inner.rhs.rename(args);
            }
            RValue::RShift(inner) => {
                inner.lhs.rename(args);
                inner.rhs.rename(args);
            }
            RValue::LShift(inner) => {
                inner.lhs.rename(args);
                inner.rhs.rename(args);
            }
            RValue::Comparison(inner) => {
                inner.lhs.rename(args);
                inner.rhs.rename(args);
            }
            RValue::Not(inner) => {
                inner.op.rename(args);
            }
            RValue::Neg(inner) => {
                inner.op.rename(args);
            }
            RValue::Parenthesized(inner) => {
                inner.op.rename(args);
            }
            RValue::Constant(_) => {}
            RValue::Call(inner) => {
                inner.args.iter_mut().for_each(|arg| arg.rename(args));
            }
            RValue::Ref(inner) => {
                rename(&mut inner.field, args);
            }
            RValue::Deref(inner) => {
                inner.op.rename(args);
            }
            RValue::LValue(lvalue) => {
                lvalue.rename(args);
            }
        }
    }
}

impl LValue {
    fn rename(&mut self, args: &HashMap<String, String>) {
        match self {
            LValue::Ident(ident) => {
                rename(&mut ident.field, args);
            }
            LValue::Ref(r) => {
                r.op.rename(args);
            }
            LValue::Slice(slice) => {
                rename(&mut slice.field, args);
            }
        }
    }
}
