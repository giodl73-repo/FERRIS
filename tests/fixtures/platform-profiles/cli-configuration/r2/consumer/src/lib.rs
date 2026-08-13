//! Controlled CLI/configuration revision 2 fixture.

pub const MAX_CONFIG_BYTES: usize = 1024;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    pub cli_name: Option<String>,
    pub config_path: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResolveError {
    UnknownArgument(String),
    MissingNameValue,
    MissingConfigValue,
    EmptyName,
    ConfigTooLarge,
    ConfigNotUtf8,
    MalformedConfig,
}

pub fn parse_arguments(args: &[String]) -> Result<Request, ResolveError> {
    let mut request = Request {
        cli_name: None,
        config_path: None,
    };
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--name" => {
                index += 1;
                request.cli_name = Some(
                    args.get(index)
                        .ok_or(ResolveError::MissingNameValue)?
                        .to_owned(),
                );
            }
            "--config" => {
                index += 1;
                request.config_path = Some(
                    args.get(index)
                        .ok_or(ResolveError::MissingConfigValue)?
                        .to_owned(),
                );
            }
            unknown => return Err(ResolveError::UnknownArgument(unknown.to_owned())),
        }
        index += 1;
    }
    Ok(request)
}

/// Resolves CLI, explicit config bytes, environment, then the default.
///
/// ```
/// use ferris_profile_cli_configuration::{Request, resolve_name};
///
/// let request = Request {
///     cli_name: None,
///     config_path: Some("fixture.conf".to_owned()),
/// };
/// assert_eq!(
///     resolve_name(&request, Some(b"name=config\n"), Some("environment")).unwrap(),
///     "config"
/// );
/// ```
pub fn resolve_name(
    request: &Request,
    config: Option<&[u8]>,
    environment: Option<&str>,
) -> Result<String, ResolveError> {
    let config_name = match config {
        Some(bytes) => Some(parse_config(bytes)?),
        None => None,
    };
    let name = request
        .cli_name
        .as_deref()
        .or(config_name.as_deref())
        .or(environment)
        .unwrap_or("item");
    if name.is_empty() {
        return Err(ResolveError::EmptyName);
    }
    Ok(name.to_owned())
}

fn parse_config(bytes: &[u8]) -> Result<String, ResolveError> {
    if bytes.len() > MAX_CONFIG_BYTES {
        return Err(ResolveError::ConfigTooLarge);
    }
    let text = std::str::from_utf8(bytes).map_err(|_| ResolveError::ConfigNotUtf8)?;
    let line = text.strip_suffix('\n').unwrap_or(text);
    let value = line
        .strip_prefix("name=")
        .filter(|value| !value.contains('\n') && !value.contains('\r'))
        .ok_or(ResolveError::MalformedConfig)?;
    if value.is_empty() {
        return Err(ResolveError::EmptyName);
    }
    Ok(value.to_owned())
}

#[cfg(test)]
mod tests {
    use super::{MAX_CONFIG_BYTES, Request, ResolveError, parse_arguments, resolve_name};

    #[test]
    fn applies_cli_config_environment_default_precedence() {
        let request = parse_arguments(&[
            "--config".to_owned(),
            "fixture.conf".to_owned(),
            "--name".to_owned(),
            "cli".to_owned(),
        ])
        .unwrap();
        assert_eq!(
            resolve_name(&request, Some(b"name=config\n"), Some("environment")),
            Ok("cli".to_owned())
        );

        let request = parse_arguments(&["--config".to_owned(), "fixture.conf".to_owned()]).unwrap();
        assert_eq!(
            resolve_name(&request, Some(b"name=config\n"), Some("environment")),
            Ok("config".to_owned())
        );
        assert_eq!(
            resolve_name(
                &Request {
                    cli_name: None,
                    config_path: None,
                },
                None,
                Some("environment")
            ),
            Ok("environment".to_owned())
        );
    }

    #[test]
    fn rejects_invalid_config_bytes() {
        let request = Request {
            cli_name: None,
            config_path: Some("fixture.conf".to_owned()),
        };
        assert_eq!(
            resolve_name(&request, Some(&vec![b'x'; MAX_CONFIG_BYTES + 1]), None),
            Err(ResolveError::ConfigTooLarge)
        );
        assert_eq!(
            resolve_name(&request, Some(&[0xff]), None),
            Err(ResolveError::ConfigNotUtf8)
        );
        assert_eq!(
            resolve_name(&request, Some(b"other=value\n"), None),
            Err(ResolveError::MalformedConfig)
        );
    }
}
