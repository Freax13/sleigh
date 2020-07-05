use crate::spec::RValue;

#[derive(Clone)]
pub enum LValue {
    Ident(LValueIdent),
    Slice(LValueSlice),
    Ref(LValueRef),
}

#[derive(Clone)]
pub struct LValueIdent {
    pub field: String,
    pub size: Option<u8>,
}

#[derive(Clone)]
pub struct LValueSlice {
    pub field: String,
    pub offset: u8,
    pub size: u8,
}

#[derive(Clone)]
pub struct LValueRef {
    pub space: Option<String>,
    pub size: Option<u8>,
    pub op: Box<RValue>,
}
