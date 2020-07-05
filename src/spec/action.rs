use crate::{LValue, LValueIdent, RValue};

#[derive(Clone)]
pub enum Action {
    Label(String),
    LocalDecl(ActionLocalDecl),
    Export(ActionExport),
    Assignment(ActionAssignment),
    Build(ActionBuild),
    If(Box<ActionIf>),
    Goto(ActionGoto),
    Macro(ActionMacro),
    PCodeOp(ActionPCodeOp),
    Call(ActionCall),
    Return(ActionReturn),
}

#[derive(Clone)]
pub struct ActionLocalDecl {
    pub name: LValueIdent,
    pub val: RValue,
}

#[derive(Clone)]
pub struct ActionExport {
    pub op: RValue,
}

#[derive(Clone)]
pub struct ActionAssignment {
    pub name: LValue,
    pub val: RValue,
}

#[derive(Clone)]
pub struct ActionBuild {
    pub field: String,
}

#[derive(Clone)]
pub struct ActionIf {
    pub cond: RValue,
    pub action: Action,
}

#[derive(Clone)]
pub enum ActionGoto {
    Label(String),
    Address(RValue),
}

#[derive(Clone)]
pub struct ActionMacro {
    pub r#macro: String,
    pub args: Vec<RValue>,
}

#[derive(Clone)]
pub struct ActionPCodeOp {
    pub pcopdeop: String,
    pub args: Vec<RValue>,
}

#[derive(Clone)]
pub struct ActionCall {
    pub address: RValue,
}

#[derive(Clone)]
pub struct ActionReturn {
    pub val: RValue,
}
