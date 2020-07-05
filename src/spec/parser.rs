use crate::spec::*;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;
use std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
    ops::Range,
};

#[derive(Parser, Default)]
#[grammar = "../pest/sleigh.pest"] // relative to src
pub struct SleighParser {
    endianness: Option<Endianness>,
    alignment: u8,
    spaces: Vec<Space>,
    registers: Vec<Register>,
    tokens: Vec<Token>,
    contexts: Vec<Context>,
    pcodeops: Vec<PCodeOp>,
    constructors: Vec<Constructor>,
    macros: Vec<Macro>,
}

impl SleighParser {
    pub fn parse_file(s: &str) -> Spec {
        let stmts: Pairs<Rule> =
            SleighParser::parse(Rule::file, &s).unwrap_or_else(|e| panic!("{}", e));
        let mut parser: SleighParser = Default::default();
        parser.alignment = 1;
        parser.parse_stmts(stmts, None);
        parser.finish()
    }

    fn parse_stmts<'p>(
        &mut self,
        stmts: impl Iterator<Item = Pair<'p, Rule>>,
        with_context: Option<&WithBlockContext>,
    ) {
        for stmt in stmts {
            let stmt: Pair<Rule> = stmt;

            let rule = stmt.as_rule();
            let content = stmt.as_str();
            let span = stmt.as_span();

            let tokens = stmt.into_inner();
            match rule {
                Rule::define_endianness => self.define_endianness(tokens),
                Rule::define_alignment => self.define_alignment(tokens),
                Rule::define_space => self.define_space(tokens),
                Rule::define_register => self.define_register(tokens),
                Rule::define_token => self.define_token(tokens),
                Rule::define_context => self.define_context(tokens),
                Rule::define_pcodeop => self.define_pcodeop(tokens),
                Rule::attach_variables => self.attach_variables(tokens),
                Rule::attach_values => self.attach_values(tokens),
                Rule::attach_names => self.attach_names(tokens),
                Rule::stmt_macro => self.stmt_macro(tokens),
                Rule::with_block => self.with_block(tokens, with_context),
                Rule::constructor => self.constructor(tokens, with_context),
                Rule::EOI => {}
                rule => {
                    let content = &content[..content.len().min(1000)];
                    unreachable!(
                        "{:?} at {:?}: {}",
                        rule,
                        span.start_pos().line_col(),
                        content
                    )
                }
            }
        }
    }

    fn define_endianness(&mut self, mut tokens: Pairs<Rule>) {
        self.endianness = Some(match tokens.next().unwrap().as_rule() {
            Rule::little_endian => Endianness::Little,
            Rule::big_endian => Endianness::Big,
            _ => unreachable!(),
        });
    }

    fn define_alignment(&mut self, mut tokens: Pairs<Rule>) {
        self.alignment = Self::parse_integer(tokens.next().unwrap());
    }

    fn define_space(&mut self, mut tokens: Pairs<Rule>) {
        let name = tokens.next().unwrap().as_str().to_string();
        let mut ty = None;
        let mut size = None;
        let mut default = false;
        let mut wordsize = 1;

        tokens.for_each(|token| match token.as_rule() {
            Rule::space_attribute_type => {
                ty = Some(match token.into_inner().next().unwrap().as_str() {
                    "ram_space" => SpaceType::Ram,
                    "rom_space" => SpaceType::Rom,
                    "register_space" => SpaceType::Register,
                    _ => unreachable!(),
                });
            }
            Rule::space_attribute_size => {
                size = Some(Self::parse_integer(token.into_inner().next().unwrap()))
            }
            Rule::space_attribute_default => {
                default = true;
            }
            Rule::space_attribute_wordsize => {
                wordsize = Self::parse_integer(token.into_inner().next().unwrap());
            }
            _ => unreachable!(),
        });

        self.spaces.push(Space {
            name,
            ty: ty.unwrap(),
            size: size.unwrap(),
            default,
            wordsize,
        });
    }

    fn define_register(&mut self, mut tokens: Pairs<Rule>) {
        let mut offset = Self::parse_integer(tokens.next().unwrap());
        let size = Self::parse_integer(tokens.next().unwrap());

        for name in Self::parse_string_list(tokens.next().unwrap()) {
            self.registers.push(Register {
                name: name.to_string(),
                offset,
                size,
            });
            offset += size as u32;
        }
    }

    fn define_token(&mut self, mut tokens: Pairs<Rule>) {
        let name = tokens.next().unwrap().as_str().to_string();
        let size = Self::parse_integer(tokens.next().unwrap());
        let fields = tokens
            .map(|token| {
                let mut tokens = token.into_inner();
                let name = tokens.next().unwrap().as_str().to_string();
                let start = Self::parse_integer(tokens.next().unwrap());
                let end = Self::parse_integer(tokens.next().unwrap());

                let mut field = TokenField {
                    name,
                    range: Range { start, end },
                    signed: false,
                    display: FieldDisplay::Default,
                    meaning: FieldMeaning::Default,
                };

                for attribute in tokens {
                    match attribute.as_str() {
                        "signed" => field.signed = true,
                        "dec" => field.display = FieldDisplay::Decimal,
                        "hex" => field.display = FieldDisplay::Hex,
                        _ => unreachable!(),
                    }
                }

                field
            })
            .collect();
        self.tokens.push(Token { name, size, fields })
    }

    fn define_context(&mut self, mut tokens: Pairs<Rule>) {
        let register = tokens.next().unwrap().as_str().to_string();
        let fields = tokens
            .map(|token| {
                let mut tokens = token.into_inner();
                let name = tokens.next().unwrap().as_str().to_string();
                let start = Self::parse_integer(tokens.next().unwrap());
                let end = Self::parse_integer(tokens.next().unwrap());

                let mut field = ContextField {
                    name,
                    range: Range { start, end },
                    signed: false,
                    display: FieldDisplay::Default,
                    flow: true,
                    meaning: FieldMeaning::Default,
                };

                for attribute in tokens {
                    match attribute.as_str() {
                        "signed" => field.signed = true,
                        "dec" => field.display = FieldDisplay::Decimal,
                        "hex" => field.display = FieldDisplay::Hex,
                        "noflow" => field.flow = false,
                        _ => unreachable!(),
                    }
                }

                field
            })
            .collect();
        self.contexts.push(Context { register, fields })
    }

    fn define_pcodeop(&mut self, mut tokens: Pairs<Rule>) {
        let name = tokens.next().unwrap().as_str().to_string();
        self.pcodeops.push(PCodeOp { name })
    }

    fn attach_variables(&mut self, mut tokens: Pairs<Rule>) {
        let fields = Self::parse_string_list(tokens.next().unwrap());
        let variable = Self::parse_string_list(tokens.next().unwrap());

        for field in fields {
            let variables = variable.clone().map(str::to_string).collect();
            self.set_meaning(field, FieldMeaning::Variables(variables));
        }
    }

    fn attach_values(&mut self, mut tokens: Pairs<Rule>) {
        let fields = Self::parse_string_list(tokens.next().unwrap());
        let values = Self::parse_integer_list(tokens.next().unwrap());

        for field in fields {
            let values = values.clone().collect();
            self.set_meaning(field, FieldMeaning::Values(values));
        }
    }

    fn attach_names(&mut self, mut tokens: Pairs<Rule>) {
        let fields = Self::parse_string_list(tokens.next().unwrap());
        let names = Self::parse_string_list(tokens.next().unwrap());

        for field in fields {
            let names = names.clone().map(str::to_string).collect();
            self.set_meaning(field, FieldMeaning::Names(names));
        }
    }

    fn stmt_macro(&mut self, mut tokens: Pairs<Rule>) {
        let name = tokens.next().unwrap().as_str().to_string();

        let mut args = Vec::new();
        let mut token = tokens.next().unwrap();
        while let Rule::ident = token.as_rule() {
            args.push(token.as_str().to_string());
            token = tokens.next().unwrap();
        }

        let actions = Self::parse_action_block(token);

        self.macros.push(Macro {
            name,
            args,
            actions,
        })
    }

    fn with_block(&mut self, mut tokens: Pairs<Rule>, with_context: Option<&WithBlockContext>) {
        let mut token = tokens.next().unwrap();
        let mut table = None;
        if let Rule::ident = token.as_rule() {
            table = Some(token.as_str());
            token = tokens.next().unwrap();
        }
        let constraint = Self::parse_constraint(token);

        if let Some(token) = tokens.next() {
            let rule = token.as_rule();
            let mut token = Some(token);

            let mut calculation_block = Vec::new();
            if let Rule::calculation_block = rule {
                calculation_block = Self::parse_calculation_block(token.take().unwrap());
            }

            let mut context = WithBlockContext {
                table,
                constraint,
                calculation_block,
            };
            if let Some(other) = with_context {
                context.merge(other);
            }

            if let Some(token) = token {
                self.parse_stmts(Some(token).into_iter(), Some(&context));
            }

            self.parse_stmts(tokens, Some(&context));
        }
    }

    fn constructor(&mut self, mut tokens: Pairs<Rule>, with_context: Option<&WithBlockContext>) {
        let mut header = Self::parse_table_header(tokens.next().unwrap());
        let mut constraint = Self::parse_constraint(tokens.next().unwrap());

        let mut token = tokens.next().unwrap();
        let mut calculations = with_context
            .map(|ctx| ctx.calculation_block.clone())
            .unwrap_or_default();

        if let Rule::calculation_block = token.as_rule() {
            calculations.extend(Self::parse_calculation_block(token));
            token = tokens.next().unwrap();
        }

        let actions = Self::parse_action_block(token);

        if let Some(ctx) = with_context {
            if let Some(table) = ctx.table {
                debug_assert_eq!(header.table, "");
                header.table = table.to_string();
            }
            constraint = Constraint::And(Box::new(ConstraintAnd {
                lhs: constraint,
                rhs: ctx.constraint.clone(),
            }));
        }

        self.constructors.push(Constructor {
            header,
            constraint,
            calculations,
            actions,
        });
    }

    fn set_meaning(&mut self, field: &str, meaning: FieldMeaning) {
        if let Some(field) = self
            .tokens
            .iter_mut()
            .flat_map(|token| token.fields.iter_mut())
            .find(|f| f.name == field)
        {
            field.meaning = meaning;
        } else if let Some(field) = self
            .contexts
            .iter_mut()
            .flat_map(|token| token.fields.iter_mut())
            .find(|f| f.name == field)
        {
            field.meaning = meaning;
        }
    }

    fn parse_table_header(token: Pair<Rule>) -> TableHeader {
        let mut tokens = token.into_inner();

        let mut token = tokens.next().unwrap();
        let table = if let Rule::ident = token.as_rule() {
            let table = token.as_str().to_string();
            token = tokens.next().unwrap();
            table
        } else {
            "instruction".to_string()
        };

        let mnemonic = token.as_str().to_string();

        TableHeader { table, mnemonic }
    }

    fn parse_constraint(token: Pair<Rule>) -> Constraint {
        let rule = token.as_rule();
        let mut tokens = token.into_inner();

        match rule {
            Rule::constraint => Self::parse_constraint(tokens.next().unwrap()),
            Rule::constraint_and => {
                let mut constraint = Self::parse_constraint(tokens.next().unwrap());
                for rhs in tokens.map(Self::parse_constraint) {
                    constraint = Constraint::And(Box::new(ConstraintAnd {
                        lhs: constraint,
                        rhs,
                    }));
                }
                constraint
            }
            Rule::constraint_or => {
                let mut constraint = Self::parse_constraint(tokens.next().unwrap());
                for rhs in tokens.map(Self::parse_constraint) {
                    constraint = Constraint::Or(Box::new(ConstraintOr {
                        lhs: constraint,
                        rhs,
                    }));
                }
                constraint
            }
            Rule::constraint_semi => {
                let mut constraint = Self::parse_constraint(tokens.next().unwrap());
                for rhs in tokens.map(Self::parse_constraint) {
                    constraint = Constraint::Semi(Box::new(ConstraintSemi {
                        lhs: constraint,
                        rhs,
                    }));
                }
                constraint
            }
            Rule::basic_constraint_comparison => {
                let lhs = tokens.next().unwrap().as_str().to_string();

                let mut token = tokens.next().unwrap();
                let mut num_type = NumTypePrefix::Default;
                if let Rule::num_type_prefix = token.as_rule() {
                    num_type = Self::parse_num_type(token);
                    token = tokens.next().unwrap();
                }
                let comparison = Self::parse_comparison_operator(token);
                let rhs = Self::parse_constraint_rvalue(tokens.next().unwrap());
                Constraint::Comparison(ConstraintComparison {
                    lhs,
                    num_type,
                    comparison,
                    rhs,
                })
            }
            Rule::basic_constraint_exists => {
                let ident = tokens.next().unwrap().as_str().to_string();
                Constraint::Exists(ident)
            }
            Rule::basic_constraint_parenthesized => {
                let constraint = Self::parse_constraint(tokens.next().unwrap());
                Constraint::Parenthesized(Box::new(constraint))
            }
            r => unreachable!("{:?}", r),
        }
    }

    fn parse_constraint_rvalue(token: Pair<Rule>) -> ConstraintRValue {
        let rule = token.as_rule();

        if let Rule::ident = rule {
            return ConstraintRValue::Field(token.as_str().to_string());
        }

        let mut tokens = token.into_inner();
        match rule {
            Rule::constraint_rvalue => Self::parse_constraint_rvalue(tokens.next().unwrap()),
            Rule::constraint_rvalue_int_add => {
                let mut rvalue = Self::parse_constraint_rvalue(tokens.next().unwrap());
                for rhs in tokens.map(Self::parse_constraint_rvalue) {
                    rvalue =
                        ConstraintRValue::Add(Box::new(ConstraintRValueAdd { lhs: rvalue, rhs }));
                }
                rvalue
            }
            Rule::signed_integer => {
                ConstraintRValue::Integer(Self::parse_signed_integer(tokens.next().unwrap()))
            }
            r => unreachable!("{:?}", r),
        }
    }

    fn parse_comparison_operator(token: Pair<Rule>) -> ComparisonOperator {
        debug_assert!(
            token.as_rule() == Rule::constraint_comparison_operator
                || token.as_rule() == Rule::comparison_operator
        );
        match token.as_str() {
            "=" | "==" => ComparisonOperator::Equal,
            "!=" => ComparisonOperator::NotEqual,
            "<" => ComparisonOperator::Less,
            "<=" => ComparisonOperator::LessEqual,
            ">" => ComparisonOperator::Greater,
            ">=" => ComparisonOperator::GreaterEqual,
            r => unreachable!("{}", r),
        }
    }

    fn parse_calculation_block(token: Pair<Rule>) -> Vec<Calculation> {
        debug_assert_eq!(token.as_rule(), Rule::calculation_block);
        token.into_inner().map(Self::parse_calculation).collect()
    }

    fn parse_calculation(token: Pair<Rule>) -> Calculation {
        let rule = token.as_rule();
        let mut tokens = token.into_inner();

        match rule {
            Rule::calculation_assignment => {
                let lhs = tokens.next().unwrap().as_str().to_string();
                let rhs = Self::parse_rvalue(tokens.next().unwrap());
                Calculation::Assignment(CalculationAssignment { lhs, rhs })
            }
            Rule::calculation_globalset => {
                let lhs = Self::parse_rvalue(tokens.next().unwrap());
                let rhs = Self::parse_rvalue(tokens.next().unwrap());
                Calculation::GlobalSet(CalculationGlobalSet { lhs, rhs })
            }
            r => unreachable!("{:?}", r),
        }
    }

    fn parse_action_block(token: Pair<Rule>) -> Vec<Action> {
        debug_assert_eq!(token.as_rule(), Rule::action_block);
        token.into_inner().map(Self::parse_action).collect()
    }

    fn parse_action(token: Pair<Rule>) -> Action {
        let rule = token.as_rule();

        if let Rule::label = rule {
            let label = Self::parse_label(token);
            return Action::Label(label);
        }

        let mut tokens = token.into_inner();

        match rule {
            Rule::action_export => Action::Export(ActionExport {
                op: Self::parse_rvalue(tokens.next().unwrap()),
            }),
            Rule::action_local_decl => {
                let name = Self::parse_lvalue_ident(tokens.next().unwrap().into_inner());
                let val = Self::parse_rvalue(tokens.next().unwrap());
                Action::LocalDecl(ActionLocalDecl { name, val })
            }
            Rule::action_assignment => {
                let name = Self::parse_lvalue(tokens.next().unwrap());
                let val = Self::parse_rvalue(tokens.next().unwrap());
                Action::Assignment(ActionAssignment { name, val })
            }
            Rule::action_build => {
                let field = tokens.next().unwrap().as_str().to_string();
                Action::Build(ActionBuild { field })
            }
            Rule::action_if => {
                let cond = Self::parse_rvalue(tokens.next().unwrap());
                let action = Self::parse_action(tokens.next().unwrap());
                Action::If(Box::new(ActionIf { cond, action }))
            }
            Rule::action_goto => {
                let token = tokens.next().unwrap();
                match token.as_rule() {
                    Rule::label => {
                        let label = Self::parse_label(token);
                        Action::Goto(ActionGoto::Label(label))
                    }
                    _ => {
                        let address = Self::parse_rvalue(token);
                        Action::Goto(ActionGoto::Address(address))
                    }
                }
            }
            Rule::action_macro => {
                let r#macro = tokens.next().unwrap().as_str().to_string();
                let args = tokens.map(Self::parse_rvalue).collect();
                Action::Macro(ActionMacro { r#macro, args })
            }
            Rule::action_call => {
                let address = Self::parse_rvalue(tokens.next().unwrap());
                Action::Call(ActionCall { address })
            }
            Rule::action_return => {
                let val = Self::parse_rvalue(tokens.next().unwrap());
                Action::Return(ActionReturn { val })
            }
            r => unreachable!("{:?}", r),
        }
    }

    fn parse_rvalue(token: Pair<Rule>) -> RValue {
        let rule = token.as_rule();

        if let Rule::lvalue = rule {
            return RValue::LValue(Self::parse_lvalue(token));
        }

        let mut tokens = token.into_inner();

        if let Rule::rvalue = rule {
            return Self::parse_rvalue(tokens.next().unwrap());
        }

        macro_rules! binary_operator_with_prefix {
            ($rule:ident, $en:ident, $ty:ident) => {
                if let Rule::$rule = rule {
                    let mut value = Self::parse_rvalue(tokens.next().unwrap());
                    while let Some(mut token) = tokens.next() {
                        let mut num_type_prefix = NumTypePrefix::Default;
                        if let Rule::num_type_prefix = token.as_rule() {
                            num_type_prefix = Self::parse_num_type(token);
                            token = tokens.next().unwrap();
                        }
                        let rhs = Self::parse_rvalue(token);
                        value = RValue::$en(Box::new($ty {
                            lhs: value,
                            num_type_prefix,
                            rhs,
                        }));
                    }
                    return value;
                }
            };
        }
        macro_rules! binary_operator {
            ($rule:ident, $en:ident, $ty:ident) => {
                if let Rule::$rule = rule {
                    let mut value = Self::parse_rvalue(tokens.next().unwrap());
                    for token in tokens {
                        let rhs = Self::parse_rvalue(token);
                        value = RValue::$en(Box::new($ty { lhs: value, rhs }));
                    }
                    return value;
                }
            };
        }

        binary_operator_with_prefix!(rvalue_add, Add, RValueAdd);
        binary_operator_with_prefix!(rvalue_sub, Sub, RValueSub);
        binary_operator_with_prefix!(rvalue_mult, Mult, RValueMult);
        binary_operator_with_prefix!(rvalue_div, Div, RValueDiv);
        binary_operator_with_prefix!(rvalue_rem, Rem, RValueRem);
        binary_operator!(rvalue_int_or, IntOr, RValueIntOr);
        binary_operator!(rvalue_int_and, IntAnd, RValueIntAnd);
        binary_operator!(rvalue_int_xor, IntXor, RValueIntXor);
        binary_operator!(rvalue_bool_or, BoolOr, RValueBoolOr);
        binary_operator!(rvalue_bool_and, BoolAnd, RValueBoolAnd);
        binary_operator!(rvalue_bool_xor, BoolXor, RValueBoolXor);
        binary_operator_with_prefix!(rvalue_rshift, RShift, RValueRShift);
        binary_operator!(rvalue_lshift, LShift, RValueLShift);

        match rule {
            Rule::rvalue_bool_comparison => {
                let mut value = Self::parse_rvalue(tokens.next().unwrap());
                if let Some(mut token) = tokens.next() {
                    let mut num_type_prefix = NumTypePrefix::Default;
                    if let Rule::num_type_prefix = token.as_rule() {
                        num_type_prefix = Self::parse_num_type(token);
                        token = tokens.next().unwrap();
                    }
                    let operator = Self::parse_comparison_operator(token);
                    let rhs = Self::parse_rvalue(tokens.next().unwrap());
                    value = RValue::Comparison(Box::new(RValueComparison {
                        lhs: value,
                        num_type_prefix,
                        operator,
                        rhs,
                    }))
                }
                value
            }
            Rule::rvalue_not => {
                let token = tokens.next().unwrap();
                if let Rule::not_operator = token.as_rule() {
                    RValue::Not(Box::new(RValueNot {
                        op: Self::parse_rvalue(tokens.next().unwrap()),
                    }))
                } else {
                    Self::parse_rvalue(token)
                }
            }
            Rule::rvalue_neg => {
                let token = tokens.next().unwrap();
                if let Rule::neg_operator = token.as_rule() {
                    RValue::Neg(Box::new(RValueNeg {
                        op: Self::parse_rvalue(tokens.next().unwrap()),
                    }))
                } else {
                    Self::parse_rvalue(token)
                }
            }
            Rule::rvalue_basic_int => {
                let value = Self::parse_signed_integer(tokens.next().unwrap());
                let size = tokens.next().map(Self::parse_integer);
                RValue::Constant(RValueConstant { value, size })
            }
            Rule::rvalue_basic_call => {
                let call = tokens.next().unwrap().as_str().to_string();
                let args = tokens.map(Self::parse_rvalue).collect();
                RValue::Call(RValueCall { call, args })
            }
            Rule::rvalue_basic_parenthesized => {
                let op = Self::parse_rvalue(tokens.next().unwrap());
                RValue::Parenthesized(Box::new(RValueParenthesized { op }))
            }
            Rule::rvalue_basic_ref => {
                let mut token = tokens.next().unwrap();
                let mut size = None;
                if let Rule::integer = token.as_rule() {
                    size = Some(Self::parse_integer(token));
                    token = tokens.next().unwrap();
                }
                let field = token.as_str().to_string();
                RValue::Ref(RValueRef { field, size })
            }
            Rule::rvalue_basic_deref => {
                let address = Self::parse_rvalue(tokens.next().unwrap());
                RValue::Deref(Box::new(RValueDeref { address }))
            }
            r => unreachable!("{:?}", r),
        }
    }

    fn parse_lvalue(token: Pair<Rule>) -> LValue {
        let rule = token.as_rule();
        let mut tokens = token.into_inner();

        match rule {
            Rule::lvalue => Self::parse_lvalue(tokens.next().unwrap()),
            Rule::lvalue_ident => LValue::Ident(Self::parse_lvalue_ident(tokens)),
            Rule::lvalue_ref => {
                let mut token = tokens.next().unwrap();
                let mut space = None;
                if let Rule::ident = token.as_rule() {
                    space = Some(token.as_str().to_string());
                    token = tokens.next().unwrap();
                }
                let mut size = None;
                if let Rule::integer = token.as_rule() {
                    size = Some(Self::parse_integer(token));
                    token = tokens.next().unwrap();
                }
                let op = Self::parse_rvalue(token);
                LValue::Ref(LValueRef {
                    space,
                    size,
                    op: Box::new(op),
                })
            }
            Rule::lvalue_slice => {
                let field = tokens.next().unwrap().as_str().to_string();
                let offset = Self::parse_integer(tokens.next().unwrap());
                let size = Self::parse_integer(tokens.next().unwrap());
                LValue::Slice(LValueSlice {
                    field,
                    offset,
                    size,
                })
            }
            r => unreachable!("{:?}", r),
        }
    }

    fn parse_lvalue_ident(mut tokens: Pairs<Rule>) -> LValueIdent {
        let field = tokens.next().unwrap().as_str().to_string();
        let size = tokens.next().map(Self::parse_integer);
        LValueIdent { field, size }
    }

    fn parse_label(token: Pair<Rule>) -> String {
        debug_assert_eq!(token.as_rule(), Rule::label);
        token.into_inner().as_str().to_string()
    }

    fn parse_num_type(token: Pair<Rule>) -> NumTypePrefix {
        debug_assert_eq!(token.as_rule(), Rule::num_type_prefix);
        match token.as_str() {
            "f" => NumTypePrefix::Float,
            "s" => NumTypePrefix::Signed,
            _ => unreachable!(),
        }
    }

    fn parse_integer_list<I: TryFrom<u128> + 'static>(
        token: Pair<Rule>,
    ) -> impl Iterator<Item = I> + Clone + '_
    where
        I::Error: Debug,
    {
        debug_assert_eq!(token.as_rule(), Rule::integer_list);
        token.into_inner().map(Self::parse_integer)
    }

    fn parse_integer<I: TryFrom<u128>>(token: Pair<Rule>) -> I
    where
        I::Error: Debug,
    {
        let val = match token.as_rule() {
            Rule::integer => Self::parse_integer(token.into_inner().next().unwrap()),
            Rule::integer_decimal => token.as_str().parse().unwrap(),
            Rule::integer_binary => u128::from_str_radix(&token.as_str()[2..], 2).unwrap(),
            Rule::integer_hexadecimal => u128::from_str_radix(&token.as_str()[2..], 16).unwrap(),
            r => unreachable!("{:?}", r),
        };
        I::try_from(val).unwrap()
    }

    fn parse_signed_integer<I: TryFrom<i128>>(token: Pair<Rule>) -> I
    where
        I::Error: Debug,
    {
        if let Rule::signed_integer = token.as_rule() {
            return Self::parse_signed_integer(token.into_inner().next().unwrap());
        }

        let mut tokens = token.into_inner();
        let mut token = tokens.next().unwrap();

        let mut factor = 1;
        if let Rule::sign = token.as_rule() {
            factor = match token.as_str() {
                "+" => 1,
                "-" => -1,
                _ => unreachable!(),
            };
            token = tokens.next().unwrap();
        }

        let value: u128 = Self::parse_integer(token);
        let value: i128 = value.try_into().unwrap();
        I::try_from(value * factor).unwrap()
    }

    fn parse_string_list(token: Pair<Rule>) -> impl Iterator<Item = &str> + Clone + '_ {
        debug_assert_eq!(token.as_rule(), Rule::string_list);
        token.into_inner().map(Self::parse_ident_or_string)
    }

    fn parse_ident_or_string(token: Pair<Rule>) -> &str {
        let token = token.into_inner().next().unwrap();
        match token.as_rule() {
            Rule::ident => token.as_str(),
            Rule::string => Self::parse_string(token),
            _ => unreachable!(),
        }
    }

    fn parse_string(token: Pair<Rule>) -> &str {
        debug_assert_eq!(token.as_rule(), Rule::string);
        token.into_inner().next().unwrap().as_str()
    }

    fn finish(self) -> Spec {
        Spec {
            endianness: self.endianness.unwrap(),
            alignment: self.alignment,
            spaces: self.spaces,
            registers: self.registers,
            tokens: self.tokens,
            contexts: self.contexts,
            pcodeops: self.pcodeops,
            constructors: self.constructors,
            macros: self.macros,
        }
    }
}
