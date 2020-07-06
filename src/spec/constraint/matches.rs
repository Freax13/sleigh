use super::*;
use crate::State;

impl Constraint {
    pub fn matches(&self, state: State) -> bool {
        match self {
            Constraint::Ellipsis(inner) => inner.matches(state),
            Constraint::And(inner) => inner.matches(state),
            Constraint::Or(inner) => inner.matches(state),
            Constraint::Semi(inner) => inner.matches(state),
            Constraint::Parenthesized(inner) => inner.matches(state),
            Constraint::Comparison(inner) => inner.matches(state),
            Constraint::Exists(inner) => inner.matches(state),
        }
    }

    fn len(&self, state: State) -> Option<usize> {
        match self {
            Constraint::Ellipsis(inner) => todo!(),
            Constraint::And(inner) => inner
                .lhs
                .len(state.clone())
                .or_else(|| inner.rhs.len(state)),
            Constraint::Or(inner) => todo!(),
            Constraint::Semi(inner) => todo!(),
            Constraint::Parenthesized(inner) => todo!(),
            Constraint::Comparison(inner) => inner
                .lhs
                .len(state.clone())
                .or_else(|| inner.rhs.len(state)),
            Constraint::Exists(inner) => inner.len(state),
        }
    }
}

impl ConstraintEllipsis {
    pub fn matches(&self, state: State) -> bool {
        todo!()
    }
}

impl ConstraintAnd {
    pub fn matches(&self, state: State) -> bool {
        self.lhs.matches(state.clone()) && self.rhs.matches(state)
    }
}

impl ConstraintOr {
    pub fn matches(&self, state: State) -> bool {
        self.lhs.matches(state.clone()) || self.rhs.matches(state)
    }
}

impl ConstraintSemi {
    pub fn matches(&self, mut state: State) -> bool {
        if !self.lhs.matches(state.clone()) {
            return false;
        }
        let len = self.lhs.len(state.clone()).unwrap_or_default();
        if state.code.len() < len {
            return false;
        }
        state.code = &state.code[len..];
        self.rhs.matches(state)
    }
}

impl ConstraintComparison {
    pub fn matches(&self, state: State) -> bool {
        if let Some(lhs) = state.eval(&self.lhs) {
            if let Some(rhs) = state.eval(&self.rhs) {
                lhs == rhs
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl ConstraintRValue {
    fn len(&self, state: State) -> Option<usize> {
        match self {
            ConstraintRValue::Add(inner) => inner
                .lhs
                .len(state.clone())
                .or_else(|| inner.rhs.len(state)),
            ConstraintRValue::Field(inner) => state.token_len(inner),
            ConstraintRValue::Integer(_) => None,
        }
    }
}

impl ConstraintExists {
    pub fn matches(&self, state: State) -> bool {
        if state
            .spec
            .constructors
            .iter()
            .any(|c| c.header.table == self.name)
            && self.name != "instruction"
        {
            state.match_constructor(Some(&self.name)).is_some()
        } else {
            true
        }
    }

    fn len(&self, state: State) -> Option<usize> {
        if let Some(c) = state
            .spec
            .constructors
            .iter()
            .find(|c| c.header.table == self.name)
        {
            c.constraint.len(state)
        } else {
            state.token_len(&self.name)
        }
    }
}
