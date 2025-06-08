use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuleEngineError<E> {
    #[error("Rule evaluation failed: {0}")]
    Evaluation(E),

    #[error("Rule application failed: {0}")]
    Application(E),

    #[error("Unknown rule error")]
    Unknown,
}


#[derive(Debug, Error)]
pub enum RuleError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML deserialization error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("JSON deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Evaluation error: {0}")]
    Eval(String),
}
