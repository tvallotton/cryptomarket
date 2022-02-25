use crate::api::Response;
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;
#[derive(Debug, Error)]
pub enum Error {
    #[error("ClosedConnectionError.")]
    Closed,

    #[error("TimeoutError: timeout while waiting for a response.")]
    Timeout,

    #[error("RecvError: this error is a bug in the cryptmkt crate. It should not occur.")]
    Error(#[from] tokio::sync::oneshot::error::RecvError),

    #[error("UnexpectedResponse: Unexpected response type.")]
    UnexpectedResponse,

    #[error("APIResponse: {0}.")]
    APIResponse(#[from] crate::api::Error),

    #[error("TokioTungstenite: {0}")]
    TokioTungstenite(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("SerdeError: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("The string \"{0}\" does not represent a supported currency")]
    ParsingCurrency(String),

    #[error("JoinError: {0}")]
    JoinError(#[from] tokio::task::JoinError),

    #[error("ParsingError: {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),

    #[error("InvestingError")]
    Investing,

    #[error("RequestError: {0}")]
    Request(#[from] reqwest::Error),

    #[error("DotenvError: {0}")]
    DotEnv(#[from] dotenv::Error),

    #[error("SendError: {0}")]
    Send(#[from] SendError<crate::api::Response>),
}

impl Error {
    pub fn unexpected(_: Response) -> Self {
        Error::UnexpectedResponse
    }

    pub fn api_respose(&self) -> Option<&crate::api::Error> {
        match self {
            | Self::APIResponse(error) => Some(error),
            | _ => None,
        }
    }
}
