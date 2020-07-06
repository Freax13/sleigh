use crate::{ConstraintRValue, Constructor, Endianness, Spec};
use std::{collections::HashMap, iter::repeat, sync::Arc};

#[derive(Clone)]
pub struct State<'s> {
    pub(crate) spec: &'s Spec,
    pub(crate) code: &'s [u8],
    registers: Arc<HashMap<String, Register>>,
}

impl<'s> State<'s> {
    pub fn new(spec: &'s Spec, code: &'s [u8]) -> Self {
        State {
            spec,
            code,
            registers: Arc::new(
                spec.registers
                    .iter()
                    .map(|r| {
                        (
                            r.name.clone(),
                            Register {
                                data: vec![0x6; r.size as usize],
                            },
                        )
                    })
                    .collect(),
            ),
        }
    }

    pub fn match_constructor(&self, table: Option<&str>) -> Option<&Constructor> {
        let table = table.unwrap_or("instruction");
        self.spec
            .constructors
            .iter()
            .filter(|c| c.header.table == table)
            .find(|c| c.matches(self.clone()))
    }

    pub(crate) fn token_len(&self, name: &str) -> Option<usize> {
        self.spec
            .tokens
            .iter()
            .find(|token| token.fields.iter().any(|f| f.name == name))
            .map(|token| token.size as usize / 8)
    }

    pub(crate) fn field_value(&self, name: &str) -> Option<i128> {
        self.spec
            .tokens
            .iter()
            .flat_map(|token| token.fields.iter().zip(repeat(token.size / 8)))
            .find(|(f, _)| f.name == name)
            .map(|(f, size)| (f.range.start, f.range.end, size, self.code))
            .or_else(|| {
                self.spec
                    .contexts
                    .iter()
                    .flat_map(|context| {
                        let register_size = self
                            .spec
                            .registers
                            .iter()
                            .find(|r| r.name == context.register)
                            .map(|r| r.size)
                            .unwrap();
                        let register = self
                            .registers
                            .get(&context.register)
                            .map(|r| &*r.data)
                            .unwrap();
                        context.fields.iter().zip(repeat((register_size, register)))
                    })
                    .find(|(f, _)| f.name == name)
                    .map(|(f, (size, data))| (f.range.start, f.range.end, size, data))
            })
            .and_then(|(start, end, size, data)| {
                if data.len() < size as usize {
                    return None;
                }
                let mask = (1 << (end + 1)) - 1;

                let mut value = 0;
                for (i, b) in data[..size as usize].iter().enumerate() {
                    let offset = if let Endianness::Little = self.spec.endianness {
                        i * 8
                    } else {
                        size as usize - i * 8 - 1
                    };
                    value |= (*b as i128) << offset;
                }

                Some((value & mask) >> start)
            })
    }

    pub(crate) fn eval(&self, rvalue: &ConstraintRValue) -> Option<i128> {
        match rvalue {
            ConstraintRValue::Add(inner) => Some(self.eval(&inner.lhs)? + self.eval(&inner.rhs)?),
            ConstraintRValue::Field(name) => self.field_value(&name),
            ConstraintRValue::Integer(val) => Some(*val),
        }
    }
}

pub struct Register {
    data: Vec<u8>,
}
