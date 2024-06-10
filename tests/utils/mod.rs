pub mod abi;
pub mod constants;
pub mod setup;

use fuels::{
    programs::call_response::FuelCallResponse,
    types::errors::{transaction::Reason, Error},
};

pub fn validate_error<T> (tx_result: Result<FuelCallResponse<T>, Error>, error: &str) {
    assert!(tx_result.is_err());
    match tx_result.err().unwrap() {
        Error::Transaction(reason) => match reason {
            Reason::Reverted { reason, ..} => {
                assert_eq!(reason.to_string(), error);
            }
            _ => {
                unreachable!("Error should be Reverted");
            }
        },
        _ => {
            unreachable!("Error should be Transaction");
        }
    }
}
