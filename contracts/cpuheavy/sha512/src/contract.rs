use cosmwasm_std::{
    Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdResult, Storage,
};

use sha2::{Digest, Sha512};

use crate::msg::{HandleMsg, InitMsg, QueryMsg};

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
        HandleMsg::Hash { input } => try_hash(deps, env, input),
    }
}

pub fn try_hash<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    input: String,
) -> StdResult<HandleResponse> {
    let mut hasher = Sha512::new();
    hasher.update(input);
    let _result = hasher.finalize();
    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    _deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
    }
}

