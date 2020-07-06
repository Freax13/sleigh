use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;
use std::{collections::HashMap, fs::read_to_string, path::Path};

#[derive(Parser)]
#[grammar = "../pest/preprocessor.pest"] // relative to src
struct Preprocessor;

pub fn preprocess(dir: impl AsRef<Path>, file: impl AsRef<Path>) -> String {
    let mut context = Context::new();
    context.preprocess_file(dir.as_ref(), file.as_ref());
    context.result
}

#[derive(Default)]
struct Context {
    defines: HashMap<String, String>,
    result: String,
}

impl Context {
    fn new() -> Self {
        Default::default()
    }

    fn preprocess_file(&mut self, dir: &Path, file: impl AsRef<Path>) {
        let file = dir.join(file);

        let raw = read_to_string(file).unwrap();
        let lines = Preprocessor::parse(Rule::file, &raw).unwrap_or_else(|e| panic!("{}", e));
        self.preprocess_items(dir, lines);
    }

    fn preprocess_items(&mut self, dir: &Path, lines: Pairs<Rule>) {
        for line in lines {
            match line.as_rule() {
                Rule::define => {
                    let mut tokens = line.into_inner();
                    let name = tokens.next().unwrap();
                    let value = tokens.next().unwrap().into_inner().next().unwrap();
                    self.defines
                        .insert(name.as_str().to_string(), value.as_str().to_string());
                }
                Rule::include => {
                    let mut tokens = line.into_inner();
                    let file = tokens.next().unwrap().into_inner().next().unwrap();
                    self.preprocess_file(dir, file.as_str());
                }
                Rule::ifdef => {
                    let mut tokens = line.into_inner();
                    let name = tokens.next().unwrap();
                    let idx = if self.defines.contains_key(name.as_str()) {
                        0
                    } else {
                        1
                    };
                    let block = tokens.nth(idx).unwrap();
                    self.preprocess_items(dir, block.into_inner());
                }
                Rule::if_block => {
                    let mut tokens = line.into_inner();
                    loop {
                        let condition = tokens.next().unwrap();

                        let rule = condition.as_rule();

                        let res = {
                            let mut tokens = condition.into_inner();
                            match rule {
                                Rule::ifdef => {
                                    let name = tokens.next().unwrap();
                                    self.defines.contains_key(name.as_str())
                                }
                                Rule::ifndef => {
                                    let name = tokens.next().unwrap();
                                    !self.defines.contains_key(name.as_str())
                                }
                                Rule::if_cond | Rule::elif_block => {
                                    let cond = tokens.next().unwrap();
                                    self.eval_expr(cond)
                                }
                                Rule::else_block => true,
                                Rule::endif => break,
                                r => unreachable!("{:?}", r),
                            }
                        };
                        let block = tokens.next().unwrap();
                        if res {
                            self.preprocess_items(dir, block.into_inner());
                            break;
                        }
                    }
                }
                Rule::sleigh_line => {
                    // to interpolation
                    let mut s = line.as_str();
                    while let Some(pos) = s.find("$(") {
                        self.result += &s[..pos];
                        s = &s[pos + 2..];
                        let end = s.find(')').unwrap();
                        let name = &s[..end];
                        self.result += &self.defines[name];
                        s = &s[end + 1..];
                    }

                    self.result += s;
                }
                Rule::EOI => {}
                rule => {
                    let content = line.as_str();
                    let content = &content[..content.len().min(1000)];
                    unreachable!(
                        "{:?} at {:?}: {}",
                        rule,
                        line.as_span().start_pos().line_col(),
                        content
                    )
                }
            }
        }
    }

    fn eval_expr(&self, cond: Pair<Rule>) -> bool {
        let rule = cond.as_rule();
        let mut tokens = cond.into_inner();
        match rule {
            Rule::or => {
                let op1 = tokens.next().unwrap();
                let op2 = tokens.next().unwrap();
                self.eval_expr(op1) || self.eval_expr(op2)
            }
            Rule::and => {
                let op1 = tokens.next().unwrap();
                let op2 = tokens.next().unwrap();
                self.eval_expr(op1) && self.eval_expr(op2)
            }
            Rule::defined => {
                let name = tokens.next().unwrap();
                self.defines.contains_key(name.as_str())
            }
            Rule::comparison => {
                let name = tokens.next().unwrap();
                let value = tokens.next().unwrap();
                self.defines[name.as_str()] == value.as_str()
            }
            r => unreachable!("{:?}", r),
        }
    }
}
