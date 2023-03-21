use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError,
    StdResult,
};
use cw_storage_plus::Item;

pub const NUMBER: Item<u64> = Item::new("number");

#[cw_serde]
pub enum ExecuteMsg {
    /// Increment the number by 1
    Increment {},

    /// Attempt to increment the number by 1, but intentionally fail by the end.
    ///
    /// Used to test that state changes effected by failed submessages will not
    /// be committed.
    IncrementButFail {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Query the current number stored in the contract
    #[returns(NumberResponse)]
    Number {},
}

#[cw_serde]
pub struct NumberResponse {
    pub number: u64,
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    NUMBER.save(deps.storage, &0)?;

    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Increment {} => {
            NUMBER.update(deps.storage, |number| -> StdResult<_> {
                Ok(number + 1)
            })?;

            Ok(Response::new())
        },
        ExecuteMsg::IncrementButFail {} => {
            NUMBER.update(deps.storage, |number| -> StdResult<_> {
                Ok(number + 1)
            })?;

            Err(StdError::generic_err("intentional error instructed by user"))
        },
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Number {} => {
            let number = NUMBER.load(deps.storage)?;
            to_binary(&NumberResponse {
                number,
            })
        },
    }
}
