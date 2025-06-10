use std::marker::PhantomData;
use crate::engine::RuleEngine;
use crate::traits::Rule;
use crate::utils::{PriorityOrder};

/// A builder for constructing a [`RuleEngine`] with a fluent interface.
///
/// The `RuleEngineBuilder` allows you to add rules, set evaluation priority
/// order, and finally build a configured [`RuleEngine`] instance.
///
/// This is useful for situations where configuration is dynamic or more readable
/// than using `RuleEngine::new` directly.
///
/// # Type Parameters
///
/// * `C` - The context type used by the rules.
/// * `R` - A type that implements the [`Rule`] trait for context `C`.
#[derive(Default)]
pub struct RuleEngineBuilder<C, R> {
    /// List of rules to be added to the engine.
    pub rules: Vec<R>,

    /// Evaluation order (ascending or descending priority).
    pub order: PriorityOrder,

    /// Phantom marker to associate the context type `C` with the builder.
    pub marker: PhantomData<C>,
}

impl<C, R> RuleEngineBuilder<C, R>
where
    R: Rule<C>,
{
    /// Creates a new empty builder with default priority order (`Asc`) and no rules.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rule_kit::builder::RuleEngineBuilder;
    /// use rule_kit::traits::Rule;
    /// use rule_kit::utils::PriorityOrder;
    /// use rule_kit::engine::RuleEngine;
    ///
    /// // Example context type
    /// #[derive(Debug)]
    /// struct MyContext {
    ///     value: i32,
    /// }
    ///
    /// // A simple rule that checks if the context value is positive
    /// struct MyRule;
    ///
    /// impl Rule<MyContext> for MyRule {
    ///     type Output = &'static str;
    ///     type RuleError = String;
    ///
    ///     fn evaluate(&self, ctx: &MyContext) -> Result<bool, Self::RuleError> {
    ///         Ok(ctx.value > 0)
    ///     }
    ///
    ///     fn apply(&self, _ctx: &MyContext) -> Result<Self::Output, Self::RuleError> {
    ///         Ok("Value is positive")
    ///     }
    ///
    ///     fn priority(&self) -> u32 {
    ///         1
    ///     }
    /// }
    ///
    /// // Building and using the rule engine
    /// let engine: RuleEngine<MyContext, MyRule> = RuleEngineBuilder::new()
    ///     .with_rules(vec![MyRule])
    ///     .priority_desc()
    ///     .build();
    ///
    /// let ctx = MyContext { value: 42 };
    /// let result = engine.evaluate_all(&ctx).unwrap();
    /// assert_eq!(result, vec!["Value is positive"]);
    /// ```
    ///
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            order: PriorityOrder::default(),
            marker: PhantomData,
        }
    }

    /// Sets the list of rules to be used in the engine.
    ///
    /// # Arguments
    ///
    /// * `rules` - A `Vec` of rules implementing the [`Rule`] trait.
    ///
    /// # Returns
    ///
    /// The builder instance with rules set.
    pub fn with_rules(mut self, rules: Vec<R>) -> Self {
        self.rules = rules;
        self
    }

    /// Sets the evaluation priority order (ascending or descending).
    ///
    /// # Arguments
    ///
    /// * `order` - A [`PriorityOrder`] value.
    ///
    /// # Returns
    ///
    /// The builder instance with priority order set.
    pub fn priority(mut self, order: PriorityOrder) -> Self {
        self.order = order;
        self
    }

    /// Sets the rule evaluation order to descending (highest priority first).
    ///
    /// Equivalent to calling `.priority(PriorityOrder::Desc)`.
    pub fn priority_desc(self) -> Self {
        self.priority(PriorityOrder::Desc)
    }

    /// Sets the rule evaluation order to ascending (lowest priority first).
    ///
    /// Equivalent to calling `.priority(PriorityOrder::Asc)`.
    pub fn priority_asc(self) -> Self {
        self.priority(PriorityOrder::Asc)
    }

    /// Consumes the builder and returns a fully constructed [`RuleEngine`] instance.
    ///
    /// The rules are automatically sorted based on the selected priority order.
    ///
    /// # Returns
    ///
    /// A configured `RuleEngine` with sorted rules and selected evaluation order.
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
