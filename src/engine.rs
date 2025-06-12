use std::marker::PhantomData;
use crate::error::RuleEngineError;
use crate::PriorityOrder;
use crate::Rule;

/// A rule engine that evaluates and applies mutable rules over a mutable context.
///
/// Suitable for workflows or business rules where rules **mutate** the context directly.
///
/// # Type Parameters
/// * `C` - The mutable context type.
/// * `R` - A rule type that implements `Rule<C>`.
#[derive(Debug)]
pub struct RuleEngine<C, R> {
    pub _rules: Vec<R>,
    pub _order: PriorityOrder,
    pub _marker: PhantomData<C>,
}

impl<C, R> RuleEngine<C, R>
where
    R: Rule<C>,
{
    /// Constructs a new mutable rule engine with ordered rules.
    pub fn new(mut rules: Vec<R>, order: Option<PriorityOrder>) -> Self {
        let order = order.unwrap_or_default();

        match order {
            PriorityOrder::Asc => rules.sort_by_key(|r| r.priority()),
            PriorityOrder::Desc => rules.sort_by_key(|r| std::cmp::Reverse(r.priority())),
        }

        Self {
            _rules: rules,
            _order: order,
            _marker: Default::default(),
        }
    }

    /// Evaluates all rules and applies those that match the context.
    ///
    /// Each rule can mutate the context directly.
    ///
    /// # Errors
    /// * `RuleEngineError::Evaluation` - If a rule fails during evaluation.
    /// * `RuleEngineError::Application` - If a rule fails during application.
    pub fn evaluate_all(
        &mut self,
        ctx: &mut C,
    ) -> Result<(), RuleEngineError<R::RuleError>> {
        for rule in &mut self._rules {
            if rule.evaluate(ctx).map_err(RuleEngineError::Evaluation)? {
                rule.before_apply(ctx);
                rule.apply(ctx).map_err(RuleEngineError::Application)?;
                rule.after_apply(ctx);
            }
        }
        Ok(())
    }

    /// Evaluates and applies only the first rule that matches.
    ///
    /// Stops processing after the first rule is successfully applied.
    pub fn evaluate_first(
        &mut self,
        ctx: &mut C,
    ) -> Result<bool, RuleEngineError<R::RuleError>> {
        for rule in &mut self._rules {
            if rule.evaluate(ctx).map_err(RuleEngineError::Evaluation)? {
                rule.before_apply(ctx);
                rule.apply(ctx).map_err(RuleEngineError::Application)?;
                rule.after_apply(ctx);
                return Ok(true);
            }
        }
        Ok(false)
    }
}
