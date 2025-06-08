use std::marker::PhantomData;
use crate::engine::RuleEngine;
use crate::rule::Rule;
use crate::structs::{PriorityOrder};

#[derive(Default)]
pub struct RuleEngineBuilder<C, R> {
    pub rules: Vec<R>,
    pub order: PriorityOrder,
    pub marker: PhantomData<C>,
}


impl<C, R> RuleEngineBuilder<C, R>
where
    R: Rule<C>,
{
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            order: PriorityOrder::default(),
            marker: PhantomData,
        }
    }

    pub fn with_rules(mut self, rules: Vec<R>) -> Self {
        self.rules = rules;
        self
    }

    pub fn priority(mut self, order: PriorityOrder) -> Self {
        self.order = order;
        self
    }

    pub fn priority_desc(self) -> Self {
        self.priority(PriorityOrder::Desc)
    }

    pub fn priority_asc(self) -> Self {
        self.priority(PriorityOrder::Asc)
    }

    pub fn build(mut self) -> RuleEngine<C, R> {
        match self.order {
            PriorityOrder::Asc => {
                self.rules.sort_by_key(|a| a.priority());
            }
            PriorityOrder::Desc => {
                self.rules.sort_by_key(|a| std::cmp::Reverse(a.priority()));
            }
        }

        RuleEngine {
            _rules: self.rules,
            _order: self.order,
            _marker: PhantomData,
        }
    }
}
