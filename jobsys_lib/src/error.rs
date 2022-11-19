use thiserror::Error;

/// Using `thiserror` to create custom Errors
/// using anyhow for error conversion and propagation.
#[derive(Error, Debug)]
pub enum JobSysError {
    #[error("NOT IMPLEMENTED")]
    NonImplementedMenuChoice,

    #[error("Failed to save customer data to file")]
    CustomerDataSaveFailed,

    #[error("Failed to load customer data from file")]
    CustomerDataLoadFailed,

    #[error("Failed to create new Customer")]
    FailedToCreateNewCustomer,

    #[error("Failed to create new Vehicle")]
    FailedToCreateNewVehicle,
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(true, true)
    }
}
