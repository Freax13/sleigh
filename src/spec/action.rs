use crate::{LValue, LValueIdent, RValue};

pub enum Action {
    Label(String),
    LocalDecl(ActionLocalDecl),
    Export(ActionExport),
    Assignment(ActionAssignment),
    Build(ActionBuild),
    If(Box<ActionIf>),
    Goto(ActionGoto),
    Macro(ActionMacro),
    Call(ActionCall),
    Return(ActionReturn),
}

pub struct ActionLocalDecl {
    pub name: LValueIdent,
    pub val: RValue,
}

pub struct ActionExport {
    pub op: RValue,
}

pub struct ActionAssignment {
    pub name: LValue,
    pub val: RValue,
}

pub struct ActionBuild {
    pub field: String,
}

pub struct ActionIf {
    pub cond: RValue,
    pub action: Action,
}

pub enum ActionGoto {
    Label(String),
    Address(RValue),
}

pub struct ActionMacro {
    pub r#macro: String,
    pub args: Vec<RValue>,
}

pub struct ActionCall {
    pub address: RValue,
}

pub struct ActionReturn {
    pub val: RValue,
}
