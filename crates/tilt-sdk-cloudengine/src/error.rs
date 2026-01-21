pub type Result<T> = std::result::Result<T, ComputeError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Service {
    OrderService,
    ComputeApi,
    VpcApi,
    PortalApi,
    Global,
}

impl Service {
    pub fn display_name(&self) -> &'static str {
        match self {
            Service::OrderService => "order-service",
            Service::ComputeApi => "compute-api",
            Service::VpcApi => "vpc-api",
            Service::PortalApi => "portal-api",
            Service::Global => "global",
        }
    }
}

impl std::fmt::Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[derive(Debug)]
pub struct ComputeError {
    pub service: Service,
    pub endpoint: Option<String>,
    pub message: String,
    pub request_id: Option<String>,
    pub hints: Option<Vec<String>>,
    pub source: Option<tilt_sdk::SdkError>,
}

impl std::fmt::Display for ComputeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@", self.service)?;
        if let Some(endpoint) = &self.endpoint {
            write!(f, "{}: ", endpoint)?;
        }
        write!(f, "{}", self.message)?;
        Ok(())
    }
}

impl std::error::Error for ComputeError {}

impl ComputeError {
    pub fn new(service: Service, endpoint: Option<&str>, message: String) -> Self {
        Self {
            service,
            endpoint: endpoint.map(|s| s.to_string()),
            message,
            request_id: None,
            hints: None,
            source: None,
        }
    }

    pub fn from_sdk_error(
        error: tilt_sdk::SdkError,
        service: Service,
        endpoint: Option<&str>,
    ) -> Self {
        let (message, request_id, hints) = match &error {
            tilt_sdk::SdkError::Http(http) => (
                format!("{}", http),
                http.request_id.clone(),
                if http.hints.is_empty() {
                    None
                } else {
                    Some(http.hints.clone())
                },
            ),
            _ => (format!("{}", error), None, None),
        };
        Self {
            service,
            endpoint: endpoint.map(|s| s.to_string()),
            message,
            request_id,
            hints,
            source: Some(error),
        }
    }

    pub fn validation(service: Service, endpoint: Option<&str>, message: String) -> Self {
        Self {
            service,
            endpoint: endpoint.map(|s| s.to_string()),
            message,
            request_id: None,
            hints: None,
            source: None,
        }
    }

    pub fn service(&self) -> Service {
        self.service
    }

    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }

    pub fn hints(&self) -> Option<&[String]> {
        self.hints.as_deref()
    }
}
