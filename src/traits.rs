/// A generic trait that defines a rule which can be evaluated and applied
/// based on a given context `C`.
///
/// This trait is useful in rule-based systems, decision engines, or business logic
/// where individual rules must determine whether they apply and, if so, produce a result.
///
/// # Type Parameters
///
/// * `C` - The context type used for evaluating and applying the rule.
///
/// # Associated Types
///
/// * `Output` - The type returned by the [`Rule::apply`] method when the rule is successfully applied.
/// * `RuleError` - The error type returned by [`Rule::evaluate`] or [`Rule::apply`] if something goes wrong.
pub trait Rule<C> {
    /// The output type produced by this rule when successfully applied.
    type Output;

    /// The error type that may be returned during evaluation or application.
    type RuleError;

    /// Evaluates whether this rule is applicable to the given context.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A reference to the context used for evaluation.
    ///
    /// # Returns
    ///
    /// * `Ok(true)` if the rule applies.
    /// * `Ok(false)` if the rule does not apply.
    /// * `Err(RuleError)` if an error occurs during evaluation.
    fn evaluate(&self, ctx: &C) -> Result<bool, Self::RuleError>;

    /// Applies the rule to the given context and produces an output.
    ///
    /// This method assumes that [`Rule::evaluate`] has returned `Ok(true)`.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A reference to the context used for application.
    ///
    /// # Returns
    ///
    /// * `Ok(Output)` if the rule is successfully applied.
    /// * `Err(RuleError)` if an error occurs during application.
    fn apply(&self, ctx: &C) -> Result<Self::Output, Self::RuleError>;

    /// Returns the priority of this rule.
    ///
    /// This value can be used to determine the order in which multiple rules are
    /// evaluated or applied. Higher values indicate higher priority.
    /// By default, the priority is `0`.
    ///
    /// # Returns
    ///
    /// A `u32` representing the rule's priority.
    fn priority(&self) -> u32 {
        0
    }
}
pub trait MutableRule<C> {
    type RuleError;

    fn name(&self) -> &str;

    fn priority(&self) -> i32 {
        0
    }

    fn evaluate(&self, ctx: &C) -> Result<bool, Self::RuleError>;

    fn apply(&mut self, ctx: &mut C) -> Result<(), Self::RuleError>;

    fn before_apply(&self, _ctx: &C) {}

    fn after_apply(&self, _ctx: &C) {}
}
