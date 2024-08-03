#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid String {0}")]
    StringParseError(String),
    #[error("Unrecognized ModelType {0}")]
    UnrecognizedModelType(String),
    #[error("Unsupported Method {0}")]
    UnrecognizedRequestPart(String),
    #[error("Unrecognized RequestPart {0}")]
    UnrecognizedRequestPartType(String),
    #[error("Unrecognized RequestPart Type {0}")]
    UnsupportedMethod(String),
    #[error("Unrecognized Tag {0}")]
    UnrecognizedTag(String),
    #[error("Cannot open file {0}")]
    CannotOpenFile(#[from] std::io::Error),
    #[error("Cannot find reference model {0}")]
    MissingModel(String),
    #[error("Cannot write template {0}")]
    WriteError(#[from] askama::Error),
}
