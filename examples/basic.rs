use rule_kit::{Rule, RuleEngine, RuleEngineBuilder};

#[derive(Debug)]
struct Order {
    pub total: f64,
    pub discount: f64,
}

#[derive(Debug, Clone)]
enum OrderRule {
    DiscountIfHighValue,
}

impl Rule<Order> for OrderRule {
    type RuleError = ();

    fn name(&self) -> &str {
        match self {
            OrderRule::DiscountIfHighValue => "DiscountIfHighValue",
        }
    }

    fn priority(&self) -> u32 {
        1
    }

    fn evaluate(&self, ctx: &Order) -> Result<bool, Self::RuleError> {
        match self {
            OrderRule::DiscountIfHighValue => Ok(ctx.total > 100.0),
        }
    }

    /// Note: `apply` takes `&mut self` and `&mut ctx`, allowing rule and context mutation.
    fn apply(&mut self, ctx: &mut Order) -> Result<(), Self::RuleError> {
        match self {
            OrderRule::DiscountIfHighValue => {
                let discount = ctx.total * 0.10;
                ctx.discount += discount;
                ctx.total -= discount;
                Ok(())
            }
        }
    }

    fn before_apply(&self, ctx: &Order) {
        println!("Checking order total: {}", ctx.total);
    }

    fn after_apply(&self, ctx: &Order) {
        println!("Applied discount, new total discount: {}", ctx.discount);
    }
}

fn main() {
    let mut order = Order {
        total: 150.0,
        discount: 0.0,
    };

    let rules = vec![OrderRule::DiscountIfHighValue];

    // Using RuleEngine directly; pass mutable reference to context
    let mut engine = RuleEngine::new(rules.clone(), None);
    engine.evaluate_all(&mut order).unwrap();
    println!("Discount after RuleEngine: {:.2}", order.discount);

    // Using builder (with priority); also requires mutable context
    let mut order2 = Order {
        total: 150.0,
        discount: 0.0,
    };

    let mut engine_built = RuleEngineBuilder::new()
        .with_rules(rules)
        .priority_asc()
        .build();

    engine_built.evaluate_all(&mut order2).unwrap();
    println!("Discount after RuleEngineBuilder: {:.2}", order2.discount);
    println!("Total after RuleEngineBuilder: {:.2}", order2.total);
}