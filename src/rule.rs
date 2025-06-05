pub trait Rule<C> {
    type Output;
    type RuleError;

    fn evaluate(&self, ctx: &C) -> Result<bool, Self::RuleError>;
    fn apply(&self, ctx: &C) -> Result<Self::Output, Self::RuleError>;

    fn priority(&self) -> u32 {
        0
    }
}
