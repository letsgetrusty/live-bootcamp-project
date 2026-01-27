#[derive(Debug)]
pub enum AuthAPIError {
    MalformedRequest,
    InvalidCredentials,
    IncorrectCredentials,
    UserAlreadyExists,
    UnexpectedError,
    MissingToken,
    InvalidToken,
}
