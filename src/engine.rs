use std::marker::PhantomData;
use crate::error::RuleEngineError;
use crate::rule::Rule;
use crate::structs::{PriorityOrder};


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
