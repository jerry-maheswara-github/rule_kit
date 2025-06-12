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
pub trait Rule<C> {
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
