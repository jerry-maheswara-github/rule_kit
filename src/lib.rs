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
//! - ✅ Minimal core: no assumptions, no boilerplate
//! - 🔌 Pluggable rules: implement `Rule<T>` for any context
//! - 🧠 DSL-friendly: support JSON/YAML/Struct-based rules
//! - 🧵 Built for scale: evaluate hundreds of rules with ease
//! - ⚙️ Optional parallel evaluation (Rayon support)
//!
//! ---
//!
//! ## 🚀 Quick Start
//!
//! ```rust
//!
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
//! ---
//!
//! ## 🛠 Coming Soon
//!
//! - [ ] JSON-based DSL compiler (`DslRule` → `impl Rule<T>`)
//! - [ ] Built-in operator library (`eq`, `ne`, `lt`, `in`, etc.)
//! - [ ] Serde support for DSL input
//! - [ ] Plugin system for dynamic rules
//!
//! ---
//!
//! ## 📜 License
//!
//! Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.

pub mod engine;
pub mod rule;


pub use rule::Rule;
pub use engine::RuleEngine;