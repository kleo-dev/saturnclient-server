pub type Result<T = Response> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Response {
    Pong,
    Success,
    Player {
        cloak: String,
        uuid: String,
        cloaks: Vec<String>,
    },
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Response::Pong => write!(f, "Pong"),
            Response::Success => write!(f, "Success"),
            Response::Player {
                cloak,
                uuid,
                cloaks,
            } => write!(f, "@cloak={cloak}@uuid={uuid}@cloaks={}", cloaks.join("$")),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    InvalidRequest,
    InvalidMethod,
    InvalidParameter,
    ParameterNotFound,
    InvalidSession,
    InvalidHandshake,
    DatabaseError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidRequest => write!(f, "Invalid request"),
            Error::InvalidMethod => write!(f, "Invalid method"),
            Error::InvalidParameter => write!(f, "Invalid parameter"),
            Error::ParameterNotFound => write!(f, "Parameter not found"),
            Error::InvalidSession => write!(f, "Invalid session"),
            Error::InvalidHandshake => write!(f, "Invalid handshake"),
            Error::DatabaseError => write!(f, "Database error"),
        }
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(e: mongodb::error::Error) -> Self {
        println!("{e}");
        Error::DatabaseError
    }
}

impl std::error::Error for Error {}
