//! # rule_kit
//!
//! **A blazing-fast, composable, and DSL-friendly rule engine kit for Rust.**
//!
//! > Define your rules. Plug your context. Let the engine do the rest.
//!
//! ---
//!
//! ## ✨ Features
//!
//! - Minimal core: no assumptions, no boilerplate
//! - Pluggable rules: implement `Rule<T>` for any context
//! - DSL-friendly: support JSON/YAML/Struct-based rules
//! - Optional parallel evaluation (Rayon support)
//! - Built for scale: evaluate hundreds of rules with ease
//!
//! ---
//!
//! ## 🚀 Quick Start
//!
//! Define a context (e.g. a struct), implement `Rule<T>`, and plug it into `RuleEngine`:
//!
//! ```rust
//! use rule_kit::{Rule, RuleEngine};
//!
//! #[derive(Debug)]
//! struct Order {
//!     pub total: f64,
//! }
//!
//! #[derive(Debug)]
//! enum OrderRule {
//!     DiscountIfHighValue,
//! }
//!
//! impl Rule<Order> for OrderRule {
//!     type Output = f64;
//!     type RuleError = ();
//!
//!     fn evaluate(&self, ctx: &Order) -> Result<bool, Self::RuleError> {
//!         match self {
//!             OrderRule::DiscountIfHighValue => Ok(ctx.total > 100.0),
//!         }
//!     }
//!
//!     fn apply(&self, ctx: &Order) -> Result<Self::Output, Self::RuleError> {
//!         match self {
//!             OrderRule::DiscountIfHighValue => Ok(ctx.total * 0.10), // 10% discount
//!         }
//!     }
//!
//!     fn priority(&self) -> u32 {
//!         1
//!     }
//! }
//!
//!  let order = Order { total: 150.0 };
//!  let rules = vec![OrderRule::DiscountIfHighValue];
//!  let engine = RuleEngine::new(rules, None);
//!
//!  let result = engine.evaluate_all(&order).unwrap();
//!  println!("Adjustments: {:?}", result);
//! ```
//!
//! ---
//!
//! ## 📦 Crate Design Philosophy
//!
//! `rule_kit` is designed to be:
//!
//! - **Composable** — add your own rule logic without modifying the engine
//! - **Extensible** — supports rule metadata, DSL parsing, logging, etc.
//! - **Performant** — built with scaling in mind (Rayon-friendly)
//!
//! You implement the `Rule<T>` trait for your domain, and the engine handles
//! prioritization, condition evaluation, and output aggregation.
//!
//! ---
//!
//! ## 📚 Module Overview
//!
//! - `engine` — Core `RuleEngine` implementation
//! - `rule` — The `Rule` trait that powers your domain logic
//! - `error` — Unified error type for rule evaluation and application
//! - `builder` — Optional `RuleEngineBuilder` for fluent-style setup
//! - `structs` — Common helper types (e.g. priority enum)
//!
//! ---
//!
//! ## 🛠 Coming Soon
//!
//! - [ ] JSON/YAML DSL compiler (`DslRule` → `impl Rule<T>`)
//! - [ ] Built-in operator library (`eq`, `ne`, `lt`, `in`, etc.)
//! - [ ] Serde support for DSL + dynamic rule config
//! - [ ] Plugin system for runtime-loaded rules
//!
//! ---
//!
//! ## 📜 License
//!
//! Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
//!
//! This means you are free to use, modify, and redistribute under either license.

pub mod engine;
pub mod rule;
pub mod error;
pub mod builder;
pub mod structs;

// Public re-exports
pub use rule::Rule;
pub use engine::RuleEngine;
pub use structs::PriorityOrder;
pub use error::RuleEngineError;
