use std::marker::PhantomData;
use crate::error::RuleEngineError;
use crate::traits::Rule;
use crate::utils::{PriorityOrder};

/// A generic rule engine that evaluates and applies a list of rules based on a given context.
///
/// The `RuleEngine` supports configurable priority ordering (`Asc` or `Desc`)
/// and provides methods to evaluate all matching rules or just the first one.
///
/// # Type Parameters
///
/// * `C` - The context type passed to each rule during evaluation and application.
/// * `R` - A type that implements the [`Rule`] trait for context `C`.
#[derive(Debug)]
pub struct RuleEngine<C, R> {
    /// The list of rules managed by the engine, sorted by priority.
    pub _rules: Vec<R>,

    /// Determines whether the rules are sorted in ascending or descending order.
    pub _order: PriorityOrder,

    /// Phantom marker to associate the context type `C` without storing it.
    pub _marker: PhantomData<C>,
}

impl<C, R> RuleEngine<C, R>
where
    R: Rule<C>,
{
    /// Creates a new `RuleEngine` with a given list of rules and an optional priority order.
    ///
    /// Rules are automatically sorted based on their priority before being stored.
    ///
    /// # Arguments
    ///
    /// * `rules` - A vector of rules to be managed by the engine.
    /// * `order` - Optional priority ordering (`Asc` or `Desc`). Defaults to `Asc` if `None` is provided.
    ///
    pub fn new(mut rules: Vec<R>, order: Option<PriorityOrder>) -> Self {
        let order = order.unwrap_or_default();
        match order {
            PriorityOrder::Asc => {
                rules.sort_by_key(|a| a.priority());
            }
            PriorityOrder::Desc => {
                rules.sort_by_key(|a| std::cmp::Reverse(a.priority()));
            }
        }
        Self {
            _rules: rules,
            _order: order,
            _marker: Default::default(),
        }
    }

    /// Evaluates all rules and applies those that return `true` from [`Rule::evaluate`].
    ///
    /// Returns a list of outputs from the successfully applied rules.
    ///
    /// # Errors
    ///
    /// Returns `RuleEngineError::Evaluation` if rule evaluation fails,
    /// or `RuleEngineError::Application` if applying a rule fails.
    ///
    /// # Returns
    ///
    /// A `Vec` of outputs from rules that evaluated to `true`.
    pub fn evaluate_all(&self, ctx: &C) -> Result<Vec<R::Output>, RuleEngineError<R::RuleError>> {
        let mut results = Vec::new();

        for rule in &self._rules {
            if rule.evaluate(ctx).map_err(RuleEngineError::Evaluation)? {
                let out = rule.apply(ctx).map_err(RuleEngineError::Application)?;
                results.push(out);
            }
        }

        Ok(results)
    }

    /// Evaluates rules in priority order and returns the output of the first rule that applies.
    ///
    /// This method stops evaluating as soon as one rule evaluates to `true` and is successfully applied.
    ///
    /// # Errors
    ///
    /// Returns `RuleEngineError::Evaluation` if rule evaluation fails,
    /// or `RuleEngineError::Application` if applying the rule fails.
    ///
    /// # Returns
    ///
    /// `Ok(Some(output))` if a rule was successfully applied, or `Ok(None)` if no rule matched.
    pub fn evaluate_first(&self, ctx: &C) -> Result<Option<R::Output>, RuleEngineError<R::RuleError>> {
        for rule in &self._rules {
            if rule.evaluate(ctx).map_err(RuleEngineError::Evaluation)? {
                return rule
                    .apply(ctx)
                    .map(Some)
                    .map_err(RuleEngineError::Application);
            }
        }
        Ok(None)
    }
}
