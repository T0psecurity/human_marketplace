use crate::error::ContractError;
use crate::helpers::ExpiryRange;
use crate::msg::SudoMsg;
use crate::state::{ASK_HOOKS, BID_HOOKS, SALE_HOOKS, SUDO_PARAMS};
use cosmwasm_std::{entry_point, Addr, DepsMut, Env, Uint128, Response};

pub struct ParamInfo {
    ask_expiry: Option<ExpiryRange>,
    bid_expiry: Option<ExpiryRange>,
    operators: Option<Vec<String>>,
    min_price: Option<Uint128>,
    listing_fee: Option<Uint128>,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    let api = deps.api;

    match msg {
        SudoMsg::UpdateParams {
            ask_expiry,
            bid_expiry,
            operators,
            min_price,
            listing_fee,
        } => sudo_update_params(
            deps,
            env,
            ParamInfo {
                ask_expiry,
                bid_expiry,
                operators,
                // max_finders_fee_bps,
                min_price,
                listing_fee,
            },
        ),
        SudoMsg::AddOperator { operator } => sudo_add_operator(deps, api.addr_validate(&operator)?),
        SudoMsg::RemoveOperator { operator } => {
            sudo_remove_operator(deps, api.addr_validate(&operator)?)
        }
        SudoMsg::AddSaleHook { hook } => sudo_add_sale_hook(deps, api.addr_validate(&hook)?),
        SudoMsg::AddAskHook { hook } => sudo_add_ask_hook(deps, env, api.addr_validate(&hook)?),
        SudoMsg::AddBidHook { hook } => sudo_add_bid_hook(deps, env, api.addr_validate(&hook)?),
        SudoMsg::RemoveSaleHook { hook } => sudo_remove_sale_hook(deps, api.addr_validate(&hook)?),
        SudoMsg::RemoveAskHook { hook } => sudo_remove_ask_hook(deps, api.addr_validate(&hook)?),
        SudoMsg::RemoveBidHook { hook } => sudo_remove_bid_hook(deps, api.addr_validate(&hook)?),
    }
}

/// Only governance can update contract params
pub fn sudo_update_params(
    deps: DepsMut,
    _env: Env,
    param_info: ParamInfo,
) -> Result<Response, ContractError> {
    let ParamInfo {
        ask_expiry,
        bid_expiry,
        operators: _operators,
        min_price,
        listing_fee,
    } = param_info;
    // if let Some(max_finders_fee_bps) = max_finders_fee_bps {
    //     if max_finders_fee_bps > MAX_FEE_BPS {
    //         return Err(ContractError::InvalidFindersFeeBps(max_finders_fee_bps));
    //     }
    // }

    ask_expiry.as_ref().map(|a| a.validate()).transpose()?;
    bid_expiry.as_ref().map(|b| b.validate()).transpose()?;

    let mut params = SUDO_PARAMS.load(deps.storage)?;

    // params.trading_fee_percent = trading_fee_bps
    //     .map(Decimal::percent)
    //     .unwrap_or(params.trading_fee_percent);

    params.ask_expiry = ask_expiry.unwrap_or(params.ask_expiry);
    params.bid_expiry = bid_expiry.unwrap_or(params.bid_expiry);

    // params.max_finders_fee_percent = max_finders_fee_bps
    //     .map(Decimal::percent)
    //     .unwrap_or(params.max_finders_fee_percent);

    params.min_price = min_price.unwrap_or(params.min_price);

    params.listing_fee = listing_fee.unwrap_or(params.listing_fee);

    SUDO_PARAMS.save(deps.storage, &params)?;

    Ok(Response::new().add_attribute("action", "update_params"))
}

pub fn sudo_add_operator(deps: DepsMut, operator: Addr) -> Result<Response, ContractError> {
    let mut params = SUDO_PARAMS.load(deps.storage)?;
    if !params.operators.iter().any(|o| o == &operator) {
        params.operators.push(operator.clone());
    } else {
        return Err(ContractError::OperatorAlreadyRegistered {});
    }
    SUDO_PARAMS.save(deps.storage, &params)?;
    let res = Response::new()
        .add_attribute("action", "add_operator")
        .add_attribute("operator", operator);
    Ok(res)
}

pub fn sudo_remove_operator(deps: DepsMut, operator: Addr) -> Result<Response, ContractError> {
    let mut params = SUDO_PARAMS.load(deps.storage)?;
    if let Some(i) = params.operators.iter().position(|o| o == &operator) {
        params.operators.remove(i);
    } else {
        return Err(ContractError::OperatorNotRegistered {});
    }
    SUDO_PARAMS.save(deps.storage, &params)?;
    let res = Response::new()
        .add_attribute("action", "remove_operator")
        .add_attribute("operator", operator);
    Ok(res)
}

pub fn sudo_add_sale_hook(deps: DepsMut, hook: Addr) -> Result<Response, ContractError> {
    SALE_HOOKS.add_hook(deps.storage, hook.clone())?;

    let res = Response::new()
        .add_attribute("action", "add_sale_hook")
        .add_attribute("hook", hook);
    Ok(res)
}

pub fn sudo_add_ask_hook(deps: DepsMut, _env: Env, hook: Addr) -> Result<Response, ContractError> {
    ASK_HOOKS.add_hook(deps.storage, hook.clone())?;

    let res = Response::new()
        .add_attribute("action", "add_ask_hook")
        .add_attribute("hook", hook);
    Ok(res)
}

pub fn sudo_add_bid_hook(deps: DepsMut, _env: Env, hook: Addr) -> Result<Response, ContractError> {
    BID_HOOKS.add_hook(deps.storage, hook.clone())?;

    let res = Response::new()
        .add_attribute("action", "add_bid_hook")
        .add_attribute("hook", hook);
    Ok(res)
}

pub fn sudo_remove_sale_hook(deps: DepsMut, hook: Addr) -> Result<Response, ContractError> {
    SALE_HOOKS.remove_hook(deps.storage, hook.clone())?;

    let res = Response::new()
        .add_attribute("action", "remove_sale_hook")
        .add_attribute("hook", hook);
    Ok(res)
}

pub fn sudo_remove_ask_hook(deps: DepsMut, hook: Addr) -> Result<Response, ContractError> {
    ASK_HOOKS.remove_hook(deps.storage, hook.clone())?;

    let res = Response::new()
        .add_attribute("action", "remove_ask_hook")
        .add_attribute("hook", hook);
    Ok(res)
}

pub fn sudo_remove_bid_hook(deps: DepsMut, hook: Addr) -> Result<Response, ContractError> {
    BID_HOOKS.remove_hook(deps.storage, hook.clone())?;

    let res = Response::new()
        .add_attribute("action", "remove_bid_hook")
        .add_attribute("hook", hook);
    Ok(res)
}
