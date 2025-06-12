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
/// A generic trait representing a **mutable** rule that may alter the context `C`
/// during application.
///
/// Use this when rules are stateful or when applying a rule involves modifying
/// the underlying data (e.g., business logic updates, workflow progression).
///
/// # Type Parameters
///
/// * `C` - The mutable context on which the rule operates.
///
/// # Associated Types
///
/// * `RuleError` - The error type returned on evaluation or application failure.
pub trait MutableRule<C> {
    /// The error type for the rule's logic.
    type RuleError;

    /// Returns the name or ID of this rule.
    ///
    /// Useful for diagnostics or logging.
    fn name(&self) -> &str;

    /// Returns the rule's priority.
    ///
    /// Higher numbers imply higher priority. Default is `0`.
    fn priority(&self) -> u32 {
        0
    }

    /// Evaluates whether the rule should be applied given the current context.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A reference to the current context.
    ///
    /// # Returns
    ///
    /// * `Ok(true)` if the rule is applicable.
    /// * `Ok(false)` if the rule should be skipped.
    /// * `Err(RuleError)` if evaluation encounters an issue.
    fn evaluate(&self, ctx: &C) -> Result<bool, Self::RuleError>;

    /// Applies the rule to the context, potentially mutating it.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the context.
    ///
    /// # Returns
    ///
    /// * `Ok(())` on successful application.
    /// * `Err(RuleError)` if the rule fails to apply.
    fn apply(&mut self, ctx: &mut C) -> Result<(), Self::RuleError>;

    /// An optional hook invoked before the rule is applied.
    ///
    /// Use this to log or prepare context if needed. Does not affect control flow.
    fn before_apply(&self, _ctx: &C) {}

    /// An optional hook invoked after the rule has been applied.
    ///
    /// Use this to clean up, log results, or trigger downstream effects.
    fn after_apply(&self, _ctx: &C) {}
}
