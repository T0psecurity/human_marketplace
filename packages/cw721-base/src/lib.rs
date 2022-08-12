mod error;
pub mod helpers;
pub mod msg;
pub mod state;

pub use crate::error::ContractError;
pub use crate::msg::{ExecuteMsg, InstantiateMsg, MintMsg, MinterResponse, QueryMsg, CollectionInfoResponse};
pub use crate::state::{Metadata};

// This is a simple type to let us handle empty extensions
pub type Extension = Option<Metadata>;