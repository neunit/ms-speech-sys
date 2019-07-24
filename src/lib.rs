//! Low level FFI bindings for Microsoft Speech SDK.
//!
pub mod macros;
pub mod properties;
pub mod api;
pub use properties::*;
pub use api::*;

use fmt::{Display, Formatter};
use std::{error::Error as StdError, fmt};

pub type Error = Box<dyn StdError + Send + Sync + 'static>;
pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct ApiError(usize);

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Speech API returned error with code: {}", self.0)
    }
}

impl StdError for ApiError {}

/// (-1) as INVALID HANDLE for initilization or validation.
pub const INVALID_HANDLE: SPXHANDLE = std::usize::MAX as SPXHANDLE;

/// (0:usize) as NULL HANDLE for initilization.
pub const NULL_HANDLE: SPXHANDLE = 0 as SPXHANDLE;

/// Trait for underlying handle of the API.
pub trait Handle<T = SPXHANDLE> {
    fn handle(&self) -> T;
}

/// From hresult to Result.
pub fn from_hr(code: usize) -> Result {
    if code == 0 {
        Ok(())
    } else {
        Err(Box::new(ApiError(code)))
    }
}
