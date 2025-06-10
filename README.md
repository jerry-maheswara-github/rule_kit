# rule_kit

**A blazing-fast, composable, and DSL-friendly rule engine kit for Rust.**

> Define your rules. Plug your context. Let the engine do the rest.

---

## ✨ Features

- Minimal core: no assumptions, no boilerplate
- Pluggable rules: implement `Rule<T>` for any context
- DSL-friendly: support JSON/YAML/Struct-based rules
- Built for scale: evaluate hundreds of rules with ease

---

## 🚀 Quick Start

Define a context (e.g. a struct), implement `Rule<T>`, and plug it into `RuleEngine`:

```code
 use rule_kit::{Rule, RuleEngine};

 #[derive(Debug)]
 struct Order {
    pub total: f64,
 }

 #[derive(Debug, Clone)]
 enum OrderRule {
    DiscountIfHighValue,
 }

 impl Rule<Order> for OrderRule {
    type Output = f64;
    type RuleError = ();

    fn evaluate(&self, ctx: &Order) -> Result<bool, Self::RuleError> {
        match self {
            OrderRule::DiscountIfHighValue => Ok(ctx.total > 100.0),
        }
    }

    fn apply(&self, ctx: &Order) -> Result<Self::Output, Self::RuleError> {
        match self {
            OrderRule::DiscountIfHighValue => Ok(ctx.total * 0.10), // 10% discount
        }
    }

    fn priority(&self) -> u32 {
        1
    }
}

 let order = Order { total: 150.0 };
 let rules = vec![OrderRule::DiscountIfHighValue];
 let engine = RuleEngine::new(rules.clone(), None);

 let result = engine.evaluate_all(&order).unwrap();
 println!("Adjustments: {:?}", result);

// - Using RuleEngineBuilder for more flexibility
 use rule_kit::builder::RuleEngineBuilder;
 use rule_kit::structs::PriorityOrder;
 let engine_built = RuleEngineBuilder::new()
    .with_rules(rules)
    .priority_asc()
    .build();

 println!("Adjustments: {:?}", engine_built.evaluate_all(&order).unwrap());
```
---

## 📦 Crate Design Philosophy

`rule_kit` is designed to be:

- **Composable** — add your own rule logic without modifying the engine
- **Extensible** — supports rule metadata, DSL parsing, logging, etc.
- **Performant** — built with scaling in mind (Rayon-friendly)

You implement the `Rule<T>` trait for your domain, and the engine handles
prioritization, condition evaluation, and output aggregation.

---

## 📜  License

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
