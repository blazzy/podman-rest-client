#[derive(Debug, thiserror::Error)]

pub enum Error {
    #[error("Invalid String {0}")]
    StringParse(String),
    #[error("Unrecognized ModelType {0}")]
    UnrecognizedModelType(String),
    #[error("Could not find model referenced! {0}")]
    MissingModelRef(String),
    #[error("Unsupported Method {0}")]
    UnrecognizedRequestPart(String),
    #[error("Unrecognized RequestPart {0}")]
    UnrecognizedRequestPartType(String),
    #[error("Could not determine common module")]
    CouldNotDetermineCommonModule,
    #[error("Unrecognized RequestPart Type {0}")]
    UnsupportedMethod(String),
    #[error("Unrecognized Tag {0}")]
    UnrecognizedTag(String),
    #[error("Unsupported x-client-default type")]
    UnsupportedXClientDefaultType,
    #[error("Cannot open file {0}")]
    CannotOpenFile(#[from] std::io::Error),
    #[error("Unrecognized Integer Format {0}")]
    UnrecognizedIntegerFormat(String),
    #[error("Cannot write template {0}")]
    Write(#[from] askama::Error),
    #[error("Yaml scan error {0}")]
    YamlScan(#[from] yaml_rust2::ScanError),
    #[error("Rust syntax parsing error {0}")]
    Syn(#[from] syn::Error),
}
