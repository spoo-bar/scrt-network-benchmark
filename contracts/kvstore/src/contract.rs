use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdResult, Storage,
};
use std::str;

use crate::msg::{Response, HandleMsg, InitMsg, QueryMsg};

pub fn init<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Store { key, value } => try_store(deps, env, key, value),
    }
}

pub fn try_store<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    key: String,
    value: String,
) -> StdResult<HandleResponse> {
    deps.storage
        .set(key.as_bytes(), value.as_bytes());

    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Get { key } => to_binary(&query_count(deps, key)?),
    }
}

fn query_count<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>, key: String) -> StdResult<Response> {
    let byte_value = deps.storage.get(key.as_bytes()).unwrap(); 
    let string_value = str::from_utf8(byte_value.as_slice()).unwrap();
    let response = Response {
        value: string_value.to_string(),
    };
    Ok(response)
}
