//! Controlled hosted-service revision 1 fixture.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Request {
    Health,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Response {
    pub status: u16,
    pub body: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServiceError {
    Cancelled,
    MalformedRequest,
}

/// Handles one in-process service request.
///
/// ```
/// use ferris_profile_hosted_service::{Request, Response, handle};
///
/// assert_eq!(
///     handle(Request::Health, false).unwrap(),
///     Response { status: 200, body: "healthy" }
/// );
/// ```
pub fn handle(request: Request, cancelled: bool) -> Result<Response, ServiceError> {
    if cancelled {
        return Err(ServiceError::Cancelled);
    }
    match request {
        Request::Health => Ok(Response {
            status: 200,
            body: "healthy",
        }),
        Request::Unknown => Err(ServiceError::MalformedRequest),
    }
}

#[cfg(test)]
mod tests {
    use super::{Request, Response, ServiceError, handle};

    #[test]
    fn serves_health() {
        assert_eq!(
            handle(Request::Health, false),
            Ok(Response {
                status: 200,
                body: "healthy"
            })
        );
    }

    #[test]
    fn rejects_malformed_and_cancelled_requests() {
        assert_eq!(
            handle(Request::Unknown, false),
            Err(ServiceError::MalformedRequest)
        );
        assert_eq!(
            handle(Request::Health, true),
            Err(ServiceError::Cancelled)
        );
    }
}
