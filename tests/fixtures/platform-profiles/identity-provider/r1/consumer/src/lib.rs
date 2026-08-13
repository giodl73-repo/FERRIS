//! Controlled synthetic identity revision 1 fixture.

const MAX_CREDENTIAL_BYTES: usize = 64;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Identity {
    pub name: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CredentialError {
    EmptyIdentity,
    EmptySecret,
    InvalidFormat,
    TooLong,
}

pub fn identify(credential: &str) -> Result<Identity, CredentialError> {
    if credential.len() > MAX_CREDENTIAL_BYTES {
        return Err(CredentialError::TooLong);
    }
    let (identity, secret) = credential
        .split_once(':')
        .ok_or(CredentialError::InvalidFormat)?;
    if identity.is_empty() {
        return Err(CredentialError::EmptyIdentity);
    }
    if secret.is_empty() {
        return Err(CredentialError::EmptySecret);
    }
    Ok(Identity {
        name: identity.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::{CredentialError, Identity, identify};

    #[test]
    fn exposes_identity_without_secret() {
        let value = identify("fixture-user:do-not-disclose").unwrap();
        assert_eq!(value, Identity { name: "fixture-user".to_owned() });
        assert!(!format!("{value:?}").contains("do-not-disclose"));
    }

    #[test]
    fn rejects_invalid_credentials() {
        assert_eq!(identify("missing"), Err(CredentialError::InvalidFormat));
        assert_eq!(identify(":secret"), Err(CredentialError::EmptyIdentity));
        assert_eq!(identify("user:"), Err(CredentialError::EmptySecret));
        assert_eq!(identify(&"x".repeat(65)), Err(CredentialError::TooLong));
    }
}
