use plexo_sdk::errors::sdk::SDKError;
use poem::error::ResponseError;
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlexoAppError {
    #[error("Authorization token not provided")]
    MissingAuthorizationToken,
    #[error("Invalid authorization token")]
    InvalidAuthorizationToken,
    #[error("Email already in use")]
    EmailAlreadyInUse,
    #[error("Password isn't valid")]
    InvalidPassword,
    #[error("Email not found")]
    EmailNotFound,
    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("SDKErrorExtended Error")]
    SDKErrorExtended(#[from] SDKErrorExtended),
    #[error("SDKError error")]
    SDKError(#[from] SDKError),

    #[error("Poem error")]
    NotFoundPoemError(#[from] poem::error::NotFoundError),

    #[error("JSONWebToken error")]
    JSONWebTokenError(#[from] jsonwebtoken::errors::Error),
}

impl ResponseError for PlexoAppError {
    fn status(&self) -> reqwest::StatusCode {
        match self {
            PlexoAppError::MissingAuthorizationToken => reqwest::StatusCode::UNAUTHORIZED,
            PlexoAppError::InvalidAuthorizationToken => reqwest::StatusCode::UNAUTHORIZED,
            PlexoAppError::EmailAlreadyInUse => reqwest::StatusCode::BAD_REQUEST,
            PlexoAppError::InvalidPassword => reqwest::StatusCode::BAD_REQUEST,
            PlexoAppError::EmailNotFound => reqwest::StatusCode::BAD_REQUEST,
            PlexoAppError::EmailAlreadyExists => reqwest::StatusCode::BAD_REQUEST,
            PlexoAppError::SDKError(_) => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            PlexoAppError::NotFoundPoemError(_) => reqwest::StatusCode::NOT_FOUND,
            PlexoAppError::JSONWebTokenError(_) => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            PlexoAppError::SDKErrorExtended(_) => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Error, Debug)]
struct SDKErrorExtended(SDKError);

impl fmt::Display for SDKErrorExtended {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl ResponseError for SDKErrorExtended {
    fn status(&self) -> reqwest::StatusCode {
        match self.0 {
            SDKError::SQLXError(_) => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
