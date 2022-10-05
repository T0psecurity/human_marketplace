use crate::msg::ExecuteMsg;
use cosmwasm_std::{to_binary, Addr, Api, StdError, StdResult, WasmMsg, CosmosMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// MarketplaceContract is a wrapper around Addr that provides a lot of helpers
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MarketplaceContract(pub Addr);

impl MarketplaceContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }
}

pub fn map_validate(api: &dyn Api, addresses: &[String]) -> StdResult<Vec<Addr>> {
    let mut validated_addresses = addresses
        .iter()
        .map(|addr| api.addr_validate(addr))
        .collect::<StdResult<Vec<_>>>()?;
    validated_addresses.sort();
    validated_addresses.dedup();
    Ok(validated_addresses)
}

#[derive(Error, Debug, PartialEq)]
pub enum ExpiryRangeError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Invalid expiration range")]
    InvalidExpirationRange {},

    #[error("Expiry min > max")]
    InvalidExpiry {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExpiryRange {
    pub min: u64,
    pub max: u64,
}

impl ExpiryRange {
    pub fn new(min: u64, max: u64) -> Self {
        ExpiryRange { min, max }
    }

    /// Validates if given expires time is within the allowable range
    pub fn is_valid(&self, expires: u64) -> Result<(), ExpiryRangeError> {
        if !(expires > self.min && expires <= self.max) {
            return Err(ExpiryRangeError::InvalidExpirationRange {});
        }

        Ok(())
    }

    pub fn validate(&self) -> Result<(), ExpiryRangeError> {
        if self.min > self.max {
            return Err(ExpiryRangeError::InvalidExpiry {});
        }

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::mock_dependencies;

    #[test]
    fn test_map_validate() {
        let deps = mock_dependencies();
        let adddreses = map_validate(
            &deps.api,
            &[
                "operator1".to_string(),
                "operator2".to_string(),
                "operator3".to_string(),
            ],
        )
        .unwrap();
        assert_eq!(3, adddreses.len());

        let adddreses = map_validate(
            &deps.api,
            &[
                "operator1".to_string(),
                "operator2".to_string(),
                "operator3".to_string(),
                "operator3".to_string(),
                "operator1".to_string(),
            ],
        )
        .unwrap();

        assert_eq!(
            adddreses,
            vec![
                Addr::unchecked("operator1".to_string()),
                Addr::unchecked("operator2".to_string()),
                Addr::unchecked("operator3".to_string()),
            ]
        )
    }
}
