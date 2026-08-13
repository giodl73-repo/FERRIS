//! Controlled assurance and packaging revision 1 fixture.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseArtifact {
    pub package: String,
    pub identity: String,
}

pub fn release_artifact(identity: &str) -> Option<ReleaseArtifact> {
    if identity.is_empty() {
        return None;
    }
    Some(ReleaseArtifact {
        package: "ferris-profile-assurance-packaging-deployment".to_owned(),
        identity: identity.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::release_artifact;

    #[test]
    fn inventories_exact_artifact() {
        let artifact = release_artifact("artifact-r1").unwrap();
        assert_eq!(artifact.identity, "artifact-r1");
        assert!(release_artifact("").is_none());
    }
}
