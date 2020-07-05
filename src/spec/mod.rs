mod action;
mod constraint;
mod lvalue;
mod parser;
mod rvalue;

pub use action::*;
pub use constraint::*;
pub use lvalue::*;
pub use rvalue::*;

use parser::SleighParser;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::{mem::replace, ops::Range};

pub struct Spec {
    pub endianness: Endianness,
    pub alignment: u8,
    pub spaces: Vec<Space>,
    pub registers: Vec<Register>,
    pub tokens: Vec<Token>,
    pub contexts: Vec<Context>,
    pub pcodeops: Vec<PCodeOp>,
    pub constructors: Vec<Constructor>,
    pub macros: Vec<Macro>,
}

impl Spec {
    pub fn parse(s: &str) -> Self {
        let mut spec = SleighParser::parse_file(s);
        spec.expand_macros();
        spec
    }

    fn expand_macros(&mut self) {
        for constructor in self.constructors.iter_mut() {
            while let Some(pos) = constructor
                .actions
                .iter()
                .position(|a| matches!(a, Action::Macro(_)))
            {
                let action = constructor.actions.remove(pos);
                let macro_invocation = if let Action::Macro(m) = action {
                    m
                } else {
                    unreachable!()
                };
                for (action, i) in self
                    .macros
                    .iter()
                    .find(|m| m.name == macro_invocation.r#macro)
                    .unwrap()
                    .expand(&macro_invocation.args)
                    .zip(pos..)
                {
                    constructor.actions.insert(i, action);
                }
            }
        }
    }
}

pub enum Endianness {
    Big,
    Little,
}

pub struct Space {
    pub name: String,
    pub ty: SpaceType,
    pub size: u8,
    pub default: bool,
    pub wordsize: u8,
}

pub enum SpaceType {
    Ram,
    Rom,
    Register,
}

pub struct Register {
    pub name: String,
    pub offset: u32,
    pub size: u16,
}

pub struct Context {
    pub register: String,
    pub fields: Vec<ContextField>,
}

pub struct ContextField {
    pub name: String,
    pub range: Range<u16>,
    pub signed: bool,
    pub display: FieldDisplay,
    pub flow: bool,
    pub meaning: FieldMeaning,
}

pub enum FieldDisplay {
    Default,
    Hex,
    Decimal,
}

pub struct Token {
    pub name: String,
    pub size: u16,
    pub fields: Vec<TokenField>,
}

pub struct TokenField {
    pub name: String,
    pub range: Range<u16>,
    pub signed: bool,
    pub display: FieldDisplay,
    pub meaning: FieldMeaning,
}

pub enum FieldMeaning {
    Default,
    Variables(Vec<String>),
    Values(Vec<u128>),
    Names(Vec<String>),
}

pub struct PCodeOp {
    pub name: String,
}

pub struct Constructor {
    pub header: TableHeader,
    pub constraint: Constraint,
    pub calculations: Vec<Calculation>,
    pub actions: Vec<Action>,
}

pub struct TableHeader {
    pub table: String,
    pub mnemonic: String,
}

#[derive(Copy, Clone, PartialEq)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Copy, Clone, PartialEq)]
pub enum NumTypePrefix {
    Default,
    Signed,
    Float,
}

#[derive(Clone)]
pub enum Calculation {
    Assignment(CalculationAssignment),
    GlobalSet(CalculationGlobalSet),
}

#[derive(Clone)]
pub struct CalculationAssignment {
    pub lhs: String,
    pub rhs: RValue,
}

#[derive(Clone)]
pub struct CalculationGlobalSet {
    pub lhs: RValue,
    pub rhs: RValue,
}

pub struct Macro {
    pub name: String,
    pub args: Vec<String>,
    pub actions: Vec<Action>,
}

impl Macro {
    pub fn expand<'s>(&'s self, args: &'s [RValue]) -> impl Iterator<Item = Action> + 's {
        static EXPAND_COUNTER: AtomicU64 = AtomicU64::new(0);
        let cnt = EXPAND_COUNTER.fetch_add(1, Ordering::Relaxed);

        let names = self
            .args
            .iter()
            .map(|s| s.as_str())
            .chain(self.actions.iter().filter_map(|a| match a {
                Action::Label(label) => Some(label.as_str()),
                Action::LocalDecl(local_decl) => Some(local_decl.name.field.as_str()),
                _ => None,
            }));

        let mut renames = HashMap::new();
        for name in names {
            renames.insert(name.to_string(), format!("macro expand {} {}", name, cnt));
        }

        self.args
            .iter()
            .cloned()
            .map(|name| LValueIdent {
                field: name,
                size: None,
            })
            .zip(args.iter().cloned())
            .map(|(name, val)| Action::LocalDecl(ActionLocalDecl { name, val }))
            .chain(self.actions.iter().cloned())
            .map(move |mut a| {
                a.rename(&renames);
                a
            })
    }
}

struct WithBlockContext<'s> {
    table: Option<&'s str>,
    constraint: Constraint,
    calculation_block: Vec<Calculation>,
}

impl WithBlockContext<'_> {
    fn merge(&mut self, other: &Self) {
        if self.table.is_none() {
            self.table = other.table;
        }
        let constraint = replace(&mut self.constraint, Constraint::Exists(String::new()));
        self.constraint = Constraint::And(Box::new(ConstraintAnd {
            lhs: constraint,
            rhs: other.constraint.clone(),
        }));
        self.calculation_block
            .extend_from_slice(&other.calculation_block);
    }
}
