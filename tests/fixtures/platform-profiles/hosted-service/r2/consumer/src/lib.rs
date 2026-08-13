//! Controlled hosted-service revision 2 fixture.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Request {
    Health,
    Readiness,
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
    Unavailable,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Service {
    ready: bool,
}

impl Service {
    pub fn set_ready(&mut self, ready: bool) {
        self.ready = ready;
    }

    /// Handles one in-process service request.
    pub fn handle(
        &self,
        request: Request,
        cancelled: bool,
    ) -> Result<Response, ServiceError> {
        if cancelled {
            return Err(ServiceError::Cancelled);
        }
        match request {
            Request::Health => Ok(Response {
                status: 200,
                body: "healthy",
            }),
            Request::Readiness if self.ready => Ok(Response {
                status: 200,
                body: "ready",
            }),
            Request::Readiness => Err(ServiceError::Unavailable),
            Request::Unknown => Err(ServiceError::MalformedRequest),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Request, Response, Service, ServiceError};

    #[test]
    fn retains_health_and_explicit_readiness_state() {
        let mut service = Service::default();
        assert_eq!(
            service.handle(Request::Health, false),
            Ok(Response {
                status: 200,
                body: "healthy"
            })
        );
        assert_eq!(
            service.handle(Request::Readiness, false),
            Err(ServiceError::Unavailable)
        );
        service.set_ready(true);
        assert_eq!(
            service.handle(Request::Readiness, false),
            Ok(Response {
                status: 200,
                body: "ready"
            })
        );
    }

    #[test]
    fn rejects_malformed_and_cancelled_requests() {
        let service = Service::default();
        assert_eq!(
            service.handle(Request::Unknown, false),
            Err(ServiceError::MalformedRequest)
        );
        assert_eq!(
            service.handle(Request::Health, true),
            Err(ServiceError::Cancelled)
        );
    }
}
