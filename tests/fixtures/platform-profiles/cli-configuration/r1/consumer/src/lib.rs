//! Controlled CLI/configuration revision 1 fixture.

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResolveError {
    UnknownArgument(String),
    MissingNameValue,
    EmptyName,
}

/// Resolves `--name`, then the owner environment value, then the default.
///
/// ```
/// use ferris_profile_cli_configuration::resolve_name;
///
/// let args = vec!["--name".to_owned(), "cli".to_owned()];
/// assert_eq!(resolve_name(&args, Some("environment")).unwrap(), "cli");
/// ```
pub fn resolve_name(args: &[String], environment: Option<&str>) -> Result<String, ResolveError> {
    let mut cli_name = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--name" => {
                index += 1;
                let value = args.get(index).ok_or(ResolveError::MissingNameValue)?;
                cli_name = Some(value.as_str());
            }
            unknown => return Err(ResolveError::UnknownArgument(unknown.to_owned())),
        }
        index += 1;
    }

    let name = cli_name.or(environment).unwrap_or("item");
    if name.is_empty() {
        return Err(ResolveError::EmptyName);
    }
    Ok(name.to_owned())
}

#[cfg(test)]
mod tests {
    use super::{ResolveError, resolve_name};

    #[test]
    fn applies_cli_environment_default_precedence() {
        assert_eq!(
            resolve_name(
                &["--name".to_owned(), "cli".to_owned()],
                Some("environment")
            ),
            Ok("cli".to_owned())
        );
        assert_eq!(
            resolve_name(&[], Some("environment")),
            Ok("environment".to_owned())
        );
        assert_eq!(resolve_name(&[], None), Ok("item".to_owned()));
    }

    #[test]
    fn rejects_unknown_missing_and_empty_values() {
        assert_eq!(
            resolve_name(&["--other".to_owned()], None),
            Err(ResolveError::UnknownArgument("--other".to_owned()))
        );
        assert_eq!(
            resolve_name(&["--name".to_owned()], None),
            Err(ResolveError::MissingNameValue)
        );
        assert_eq!(
            resolve_name(&["--name".to_owned(), String::new()], None),
            Err(ResolveError::EmptyName)
        );
    }
}
