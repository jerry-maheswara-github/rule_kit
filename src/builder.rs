use std::marker::PhantomData;
use crate::RuleEngine;
use crate::PriorityOrder;
use crate::Rule;

/// A builder for constructing a [`RuleEngine`] with a fluent interface.
///
/// This is useful when configuring rules dynamically or chaining method calls is more readable.
///
/// # Type Parameters
///
/// * `C` - The mutable context type used by the rules.
/// * `R` - A type that implements the [`Rule`] trait for context `C`.
#[derive(Default)]
pub struct RuleEngineBuilder<C, R> {
    /// Rules to be added into the rule engine.
    pub rules: Vec<R>,

    /// Evaluation order (ascending or descending).
    pub order: PriorityOrder,

    /// Marker to track the context type.
    pub marker: PhantomData<C>,
}

impl<C, R> RuleEngineBuilder<C, R>
where
    R: Rule<C>,
{
    /// Creates a new, empty builder with default ascending priority.
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            order: PriorityOrder::default(),
            marker: PhantomData,
        }
    }

    /// Sets the full list of rules.
    pub fn with_rules(mut self, rules: Vec<R>) -> Self {
        self.rules = rules;
        self
    }

    /// Adds a single rule to the existing list.
    pub fn add_rule(mut self, rule: R) -> Self {
        self.rules.push(rule);
        self
    }

    /// Sets evaluation priority order explicitly.
    pub fn priority(mut self, order: PriorityOrder) -> Self {
        self.order = order;
        self
    }

    /// Sets priority to descending (highest first).
    pub fn priority_desc(self) -> Self {
        self.priority(PriorityOrder::Desc)
    }

    /// Sets priority to ascending (lowest first).
    pub fn priority_asc(self) -> Self {
        self.priority(PriorityOrder::Asc)
    }

    /// Builds the final [`RuleEngine`] with sorted rules.
    pub fn build(mut self) -> RuleEngine<C, R> {
        match self.order {
            PriorityOrder::Asc => self.rules.sort_by_key(|r| r.priority()),
            PriorityOrder::Desc => self.rules.sort_by_key(|r| std::cmp::Reverse(r.priority())),
        }

        RuleEngine {
            _rules: self.rules,
            _order: self.order,
            _marker: PhantomData,
        }
    }
}
