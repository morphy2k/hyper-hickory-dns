#[derive(Debug)]
pub enum Error {
    HickoryDns(hickory_resolver::error::ResolveError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::HickoryDns(err) => write!(f, "HickoryDNS error: {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::HickoryDns(err) => Some(err),
        }
    }
}

impl From<hickory_resolver::error::ResolveError> for Error {
    fn from(err: hickory_resolver::error::ResolveError) -> Self {
        Self::HickoryDns(err)
    }
}
