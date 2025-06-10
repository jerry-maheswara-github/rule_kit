use rule_kit::traits::Rule;

#[derive(Debug)]
pub struct UserContext {
    pub age: u32,
    pub score: u32,
}

#[derive(Debug)]
pub struct AgeRule;

#[derive(Debug)]
pub struct ScoreRule;

impl Rule<UserContext> for AgeRule {
    type Output = &'static str;
    type RuleError = ();

    fn evaluate(&self, ctx: &UserContext) -> Result<bool, Self::RuleError> {
        Ok(ctx.age >= 18)
    }

    fn apply(&self, _ctx: &UserContext) -> Result<Self::Output, Self::RuleError> {
        Ok("Passed age check")
    }

    fn priority(&self) -> u32 {
        10
    }
}

impl Rule<UserContext> for ScoreRule {
    type Output = &'static str;
    type RuleError = ();

    fn evaluate(&self, ctx: &UserContext) -> Result<bool, Self::RuleError> {
        Ok(ctx.score >= 80)
    }

    fn apply(&self, _ctx: &UserContext) -> Result<Self::Output, Self::RuleError> {
        Ok("Passed score check")
    }

    fn priority(&self) -> u32 {
        5
    }
}

#[derive(Debug)]
pub enum UserRule {
    AgeRule(AgeRule),
    ScoreRule(ScoreRule),
}

impl Rule<UserContext> for UserRule {
    type Output = &'static str;
    type RuleError = ();

    fn evaluate(&self, ctx: &UserContext) -> Result<bool, Self::RuleError> {
        match self {
            UserRule::AgeRule(rule) => rule.evaluate(ctx),
            UserRule::ScoreRule(rule) => rule.evaluate(ctx),
        }
    }

    fn apply(&self, ctx: &UserContext) -> Result<Self::Output, Self::RuleError> {
        match self {
            UserRule::AgeRule(rule) => rule.apply(ctx),
            UserRule::ScoreRule(rule) => rule.apply(ctx),
        }
    }

    fn priority(&self) -> u32 {
        match self {
            UserRule::AgeRule(rule) => rule.priority(),
            UserRule::ScoreRule(rule) => rule.priority(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rule_kit::builder::RuleEngineBuilder;

    #[test]
    fn test_evaluate_all_with_priority() {
        let rules = vec![
            UserRule::AgeRule(AgeRule),
            UserRule::ScoreRule(ScoreRule),
        ];

        let engine = RuleEngineBuilder::new()
            .with_rules(rules)
            .priority_asc()
            .build();

        let ctx = UserContext { age: 20, score: 90 };
        let results = engine.evaluate_all(&ctx).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0], "Passed score check");
        assert_eq!(results[1], "Passed age check");
    }

    #[test]
    fn test_evaluate_first_short_circuit() {
        let rules = vec![
            UserRule::AgeRule(AgeRule),
            UserRule::ScoreRule(ScoreRule),
        ];

        let engine = RuleEngineBuilder::new()
            .with_rules(rules)
            .priority_asc()
            .build();

        let ctx = UserContext { age: 20, score: 90 };
        let result = engine.evaluate_first(&ctx).unwrap();

        assert_eq!(result, Some("Passed score check"));
    }

    #[test]
    fn test_no_rule_passed() {
        let rules = vec![
            UserRule::AgeRule(AgeRule),
            UserRule::ScoreRule(ScoreRule),
        ];

        let engine = RuleEngineBuilder::new()
            .with_rules(rules)
            .priority_asc()
            .build();

        let ctx = UserContext { age: 10, score: 30 };
        let result = engine.evaluate_all(&ctx).unwrap();

        assert!(result.is_empty());
    }
}
