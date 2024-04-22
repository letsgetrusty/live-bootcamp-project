pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    IncorrectCredentials,
    MissingToken,
    InvalidToken,
    UnexpectedError,
}
