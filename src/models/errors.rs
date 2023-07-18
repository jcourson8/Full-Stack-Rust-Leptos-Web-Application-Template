use http::status::StatusCode;
use thiserror::Error;
// use leptos::ServerFnError;

// #[derive(Debug, Error)]
// pub enum GeneralError {
//     #[error(transparent)]
//     UserError(#[from] UserError),
//     #[error("Database Pool Unavailable")]
//     DatabasePoolUnavailable,
//     #[error(transparent)]
//     TodoAppError(#[from] TodoAppError),
//     #[error(transparent)]
//     ServerFnError(#[from] MyLeptosError), 
// }

// // a hack to get around the fact that leptos::ServerFnError doesn't satisfy the AsDynError<'_> and `leptos::ServerFnError: StdError`
// pub struct MyLeptosError(leptos::ServerFnError);

// impl std::fmt::Display for MyLeptosError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

// impl std::fmt::Debug for MyLeptosError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{:?}", self.0)
//     }
// }

// impl std::error::Error for MyLeptosError {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         None  // adjust according to your needs
//     }
// }


#[derive(Debug, Clone, Error)]
pub enum TodoAppError {
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
}

impl TodoAppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            TodoAppError::NotFound => StatusCode::NOT_FOUND,
            TodoAppError::InternalServerError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum UserError {
    #[error("User not found")]
    NotFound,
    #[error("Database error")]
    DatabaseError,
    #[error("Internal server error")]
    InternalServerError,
}

impl UserError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            UserError::NotFound => StatusCode::NOT_FOUND,
            UserError::DatabaseError => StatusCode::BAD_REQUEST,
            UserError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum PoolError {
    #[error("Connection Error")]
    ConnectionError,

}

impl PoolError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            PoolError::ConnectionError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}