use crate::RValue;

#[derive(Clone, PartialEq)]
pub enum LValue {
    Ident(LValueIdent),
    Slice(LValueSlice),
    Ref(LValueRef),
}

#[derive(Clone, PartialEq)]
pub struct LValueIdent {
    pub field: String,
    pub size: Option<u8>,
}

#[derive(Clone, PartialEq)]
pub struct LValueSlice {
    pub field: String,
    pub offset: u8,
    pub size: u8,
}

#[derive(Clone, PartialEq)]
pub struct LValueRef {
    pub space: Option<String>,
    pub size: Option<u8>,
    pub op: Box<RValue>,
}
