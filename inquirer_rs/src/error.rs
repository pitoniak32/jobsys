use thiserror::Error;

/// Using `thiserror` to create custom Errors
/// using anyhow for error conversion and propagation.
#[derive(Error, Debug)]
pub enum InquisitionError {
    #[error("Retry Failed")]
    FailedRetry,

    #[error("Failed to inquire string from user")]
    String,

    #[error("Failed to inquire i32 from user")]
    I32,

    #[error("Failed to inquire u32 from user")]
    U32,

    #[error("Failed to inquire NaiveDate from user")]
    NaiveDate,

    #[error("Failed to inquire NaiveDate from user")]
    MenuChoiceError(String),
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(true, true)
    }
}
