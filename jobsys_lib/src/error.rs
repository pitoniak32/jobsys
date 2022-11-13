use thiserror::Error;

/// Using `thiserror` to create custom Errors
/// using anyhow for error conversion and propagation.
#[derive(Error, Debug)]
pub enum JobSysError {
  #[error("NOT IMPLEMENTED")]
  NonImplementedMenuChoice,
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(true, true)
    }
}
