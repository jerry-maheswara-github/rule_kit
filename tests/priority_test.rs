use rule_kit::Rule;

#[derive(Debug)]
pub struct UserContext {
    pub age: u32,
    pub score: u32,
    pub applied: Vec<String>,
}

#[derive(Debug)]
pub struct AgeRule;

#[derive(Debug)]
pub struct ScoreRule;

impl Rule<UserContext> for AgeRule {
    type RuleError = ();

    fn name(&self) -> &str {
        "AgeRule"
    }

    fn priority(&self) -> u32 {
        10
    }

    fn evaluate(&self, ctx: &UserContext) -> Result<bool, Self::RuleError> {
        Ok(ctx.age >= 18)
    }

    fn apply(&mut self, ctx: &mut UserContext) -> Result<(), Self::RuleError> {
        ctx.applied.push("Passed age check".into());
        Ok(())
    }

    fn before_apply(&self, _ctx: &UserContext) {
        println!("About to apply AgeRule");
    }

    fn after_apply(&self, _ctx: &UserContext) {
        println!("Finished applying AgeRule");
    }
}

impl Rule<UserContext> for ScoreRule {
    type RuleError = ();

    fn name(&self) -> &str {
        "ScoreRule"
    }

    fn priority(&self) -> u32 {
        5
    }

    fn evaluate(&self, ctx: &UserContext) -> Result<bool, Self::RuleError> {
        Ok(ctx.score >= 80)
    }

    fn apply(&mut self, ctx: &mut UserContext) -> Result<(), Self::RuleError> {
        ctx.applied.push("Passed score check".into());
        Ok(())
    }

    fn before_apply(&self, _ctx: &UserContext) {
        println!("About to apply ScoreRule");
    }

    fn after_apply(&self, _ctx: &UserContext) {
        println!("Finished applying ScoreRule");
    }
}

#[derive(Debug)]
pub enum UserRule {
    Age(AgeRule),
    Score(ScoreRule),
}

impl Rule<UserContext> for UserRule {
    type RuleError = ();

    fn name(&self) -> &str {
        match self {
            UserRule::Age(_) => "AgeRule",
            UserRule::Score(_) => "ScoreRule",
        }
    }

    fn priority(&self) -> u32 {
        match self {
            UserRule::Age(r) => r.priority(),
            UserRule::Score(r) => r.priority(),
        }
    }

    fn evaluate(&self, ctx: &UserContext) -> Result<bool, Self::RuleError> {
        match self {
            UserRule::Age(r) => r.evaluate(ctx),
            UserRule::Score(r) => r.evaluate(ctx),
        }
    }

    fn apply(&mut self, ctx: &mut UserContext) -> Result<(), Self::RuleError> {
        match self {
            UserRule::Age(r) => r.apply(ctx),
            UserRule::Score(r) => r.apply(ctx),
        }
    }

    fn before_apply(&self, ctx: &UserContext) {
        match self {
            UserRule::Age(r) => r.before_apply(ctx),
            UserRule::Score(r) => r.before_apply(ctx),
        }
    }

    fn after_apply(&self, ctx: &UserContext) {
        match self {
            UserRule::Age(r) => r.after_apply(ctx),
            UserRule::Score(r) => r.after_apply(ctx),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_mutates_context() {
        let mut rules: Vec<UserRule> = vec![
            UserRule::Age(AgeRule),
            UserRule::Score(ScoreRule),
        ];

        rules.sort_by_key(|r| r.priority());

        let mut ctx = UserContext {
            age: 20,
            score: 90,
            applied: vec![],
        };

        for rule in &mut rules {
            if rule.evaluate(&ctx).unwrap() {
                rule.before_apply(&ctx);
                rule.apply(&mut ctx).unwrap();
                rule.after_apply(&ctx);
            }
        }

        assert_eq!(ctx.applied, vec![
            "Passed score check",
            "Passed age check"
        ]);
    }
}
