use thiserror::Error;

/// A generic error type representing failures during rule evaluation or application
/// in a rule engine.
///
/// This enum wraps custom errors (`E`) returned by rule logic, and provides
/// clear separation between evaluation and application failure contexts.
///
/// # Type Parameters
///
/// * `E` - The inner error type returned by specific rule implementations.
#[derive(Debug, Error)]
pub enum RuleEngineError<E> {
    /// Indicates that rule evaluation failed.
    ///
    /// Typically returned from the [`/Rule::evaluate`] method.
    #[error("Rule evaluation failed: {0}")]
    Evaluation(E),

    /// Indicates that rule application failed.
    ///
    /// Typically returned from the [`/Rule::apply`] method.
    #[error("Rule application failed: {0}")]
    Application(E),

    /// A fallback error variant for unknown or uncategorized failures.
    #[error("Unknown rule error")]
    Unknown,
}

/// A concrete error type representing possible failures encountered during
/// rule processing, such as I/O, serialization, or logic errors.
///
/// This enum is useful for rules that interact with external data sources
/// or require deserialization of structured data.
#[derive(Debug, Error)]
pub enum RuleError {
    /// I/O-related error.
    ///
    /// This could occur while reading files, streams, or other I/O operations.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Generic evaluation error, typically for domain-specific rule logic issues.
    #[error("Evaluation error: {0}")]
    Eval(String),
}
