//! Controlled assurance and deployment-planning revision 2 fixture.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Channel {
    Canary,
    Stable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeploymentPlan {
    pub channel: Channel,
    pub current_identity: String,
    pub prior_identity: String,
    pub rollback_identity: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanError {
    EmptyIdentity,
    IdentityCollision,
}

pub fn plan(
    channel: Channel,
    current_identity: &str,
    prior_identity: &str,
) -> Result<DeploymentPlan, PlanError> {
    if current_identity.is_empty() || prior_identity.is_empty() {
        return Err(PlanError::EmptyIdentity);
    }
    if current_identity == prior_identity {
        return Err(PlanError::IdentityCollision);
    }
    Ok(DeploymentPlan {
        channel,
        current_identity: current_identity.to_owned(),
        prior_identity: prior_identity.to_owned(),
        rollback_identity: prior_identity.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::{Channel, PlanError, plan};

    #[test]
    fn plans_exact_rollback_without_deploying() {
        let plan = plan(Channel::Canary, "artifact-r2", "artifact-r1").unwrap();
        assert_eq!(plan.rollback_identity, "artifact-r1");
        assert_eq!(
            super::plan(Channel::Stable, "same", "same"),
            Err(PlanError::IdentityCollision)
        );
        assert_eq!(
            super::plan(Channel::Stable, "", "prior"),
            Err(PlanError::EmptyIdentity)
        );
    }
}
