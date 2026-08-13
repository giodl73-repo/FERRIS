//! Controlled synthetic identity revision 2 fixture.

const MAX_CREDENTIAL_BYTES: usize = 64;
const MAX_CHALLENGE_BYTES: usize = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Provider {
    Alpha,
    Beta,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntheticResponse {
    pub identity: String,
    pub provider: Provider,
    pub value: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CredentialError {
    EmptyIdentity,
    EmptySecret,
    InvalidFormat,
    TooLong,
}

pub fn respond(
    provider: Provider,
    credential: &str,
    challenge: &str,
) -> Result<SyntheticResponse, CredentialError> {
    if credential.len() > MAX_CREDENTIAL_BYTES || challenge.len() > MAX_CHALLENGE_BYTES {
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
    let seed: u64 = match provider {
        Provider::Alpha => 0x100,
        Provider::Beta => 0x200,
    };
    let value = secret
        .bytes()
        .chain(challenge.bytes())
        .fold(seed, |state, byte| state.wrapping_mul(16777619) ^ u64::from(byte));
    Ok(SyntheticResponse {
        identity: identity.to_owned(),
        provider,
        value,
    })
}

#[cfg(test)]
mod tests {
    use super::{CredentialError, Provider, respond};

    #[test]
    fn selects_distinct_provider_without_exposing_secret() {
        let alpha = respond(Provider::Alpha, "fixture-user:do-not-disclose", "challenge").unwrap();
        let beta = respond(Provider::Beta, "fixture-user:do-not-disclose", "challenge").unwrap();
        assert_ne!(alpha.value, beta.value);
        assert!(!format!("{alpha:?}").contains("do-not-disclose"));
    }

    #[test]
    fn rejects_invalid_credentials() {
        assert_eq!(
            respond(Provider::Alpha, "missing", "x"),
            Err(CredentialError::InvalidFormat)
        );
        assert_eq!(
            respond(Provider::Alpha, ":secret", "x"),
            Err(CredentialError::EmptyIdentity)
        );
        assert_eq!(
            respond(Provider::Alpha, "user:", "x"),
            Err(CredentialError::EmptySecret)
        );
        assert_eq!(
            respond(Provider::Alpha, "user:secret", &"x".repeat(33)),
            Err(CredentialError::TooLong)
        );
    }
}
