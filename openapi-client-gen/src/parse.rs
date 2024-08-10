use yaml_rust2::Yaml;

use crate::error::Error;

pub fn string<'a, T>(yaml: &'a Yaml, context: &str) -> Result<T, Error>
where
    T: From<&'a str>,
{
    yaml.as_str()
        .map(|s| T::from(s))
        .ok_or_else(|| Error::StringParse(context.into()))
}

pub fn try_string<T>(yaml: &Yaml, context: &str) -> Result<T, Error>
where
    for<'a> T: TryFrom<&'a str, Error = Error>,
{
    yaml.as_str()
        .ok_or_else(|| Error::StringParse(context.into()))?
        .try_into()
}

pub fn maybe_string(yaml: &Yaml) -> Option<String> {
    yaml.as_str().map(|s| s.into())
}
