# rule_kit

**A blazing-fast, composable, and DSL-friendly rule engine kit for Rust.**

> Define your rules. Plug your context (mutable or immutable). Let the engine do the rest.

---

## ✨ Features

- Minimal core: no assumptions, no boilerplate
- Pluggable rules: implement `Rule<T>` for any context (mutable support)
- DSL-friendly: support JSON/YAML/Struct-based rules
- Built for scale: evaluate hundreds of rules with ease
- Supports mutable context during rule application for stateful workflows

---

## 🚀 Quick Start

Define a context (e.g., a struct), implement `Rule<T>` with mutable `apply`, and plug it into `RuleEngine`:

```rust
use rule_kit::{Rule, RuleEngine};
use rule_kit::builder::RuleEngineBuilder;
use rule_kit::utils::PriorityOrder;

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
}
```
---

## 📦 Design Philosophy

`rule_kit` is designed to be:

- **Composable** — add your own rule logic without modifying the engine
- **Extensible** — supports rule metadata, DSL parsing, logging, etc.
- **Performant** — built with scaling in mind (Rayon-friendly)
- **Flexible** — supports mutable context in rules for stateful business logic

You implement the `Rule<T>` trait for your domain, where `apply` can mutate both the rule instance and the context, enabling advanced scenarios like workflow progression, state updates, or side effects.

---

## 📜 License

Licensed under:
- Apache License, Version 2.0 [LICENSE](http://www.apache.org/licenses/LICENSE-2.0.txt)

---

## 🧑‍💻 Author

Created and maintained by [Jerry Maheswara](https://github.com/jerry-maheswara-github)

Feel free to reach out for suggestions, issues, or improvements!

---

## ❤️ Built with Love in Rust

This project is built with ❤️ using **Rust** — a systems programming language that is safe, fast, and concurrent. Rust is the perfect choice for building reliable and efficient applications.

---

## 👋 Contributing

Pull requests, issues, and feedback are welcome!  
If you find this crate useful, give it a ⭐ and share it with others in the Rust community.

---
