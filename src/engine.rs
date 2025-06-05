use std::marker::PhantomData;
use crate::rule::Rule;

#[derive(Debug, Clone, Copy)]
pub enum PriorityOrder {
    Asc,
    Desc,
}

impl Default for PriorityOrder {
    fn default() -> Self {
        PriorityOrder::Asc
    }
}

#[derive(Debug)]
pub struct RuleEngine<C, R> {
    pub rules: Vec<R>,
    _order: PriorityOrder,
    _marker: PhantomData<C>,
}

impl<C, R> RuleEngine<C, R>
where
    R: Rule<C>,
{
    pub fn new(mut rules: Vec<R>, order: Option<PriorityOrder>) -> Self {
        let order = order.unwrap_or_default();
        match order {
            PriorityOrder::Asc => {
                rules.sort_by(|a, b| a.priority().cmp(&b.priority()));
            }
            PriorityOrder::Desc => {
                rules.sort_by(|a, b| b.priority().cmp(&a.priority()));
            }
        }
        Self {
            rules,
            _order: order,
            _marker: Default::default(),
        }
    }

    pub fn evaluate_all(&self, ctx: &C) -> Result<Vec<R::Output>, R::RuleError> {
        let mut results = Vec::new();

        for rule in &self.rules {
            if rule.evaluate(ctx)? {
                results.push(rule.apply(ctx)?);
            }
        }

        Ok(results)
    }

    pub fn evaluate_first(&self, ctx: &C) -> Result<Option<R::Output>, R::RuleError> {
        for rule in &self.rules {
            if rule.evaluate(ctx)? {
                return Ok(Some(rule.apply(ctx)?));
            }
        }
        Ok(None)
    }

}
