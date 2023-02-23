use thiserror::Error;

#[derive(Debug, Error)]
enum NetworkError {
    #[error("connection timed out")]
    TimeOut,
}

#[derive(Debug, Error)]
enum DatabaseError {
    #[error("error querying database")]
    QueryFailure,
}
#[derive(Debug, Error)]
enum ApiError {
    #[error("network error: {0}")]
    Network(#[from] NetworkError),
    #[error("databaseError")]
    Database(#[from] DatabaseError),
}

fn do_stuff() -> Result<(), ApiError> {
    Err(NetworkError::TimeOut)?
}

fn main() {}
