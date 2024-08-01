use yaml_rust2::Yaml;

use crate::error::ParseError;

pub fn is_keyword(var: &str) -> bool {
    RUST_KEYWORDS.iter().any(|k| k == &var)
}

pub fn string<'a, T>(yaml: &'a Yaml, context: &str) -> Result<T, ParseError>
where
    T: From<&'a str>,
{
    yaml.as_str()
        .map(|s| T::from(s))
        .ok_or_else(|| ParseError::StringParseError(context.into()))
}

pub fn try_string<T>(yaml: &Yaml, context: &str) -> Result<T, ParseError>
where
    for<'a> T: TryFrom<&'a str, Error = ParseError>,
{
    yaml.as_str()
        .ok_or_else(|| ParseError::StringParseError(context.into()))?
        .try_into()
}

pub fn maybe_string(yaml: &Yaml) -> Option<String> {
    yaml.as_str().map(|s| s.into())
}

const RUST_KEYWORDS: [&str; 51] = [
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
];
