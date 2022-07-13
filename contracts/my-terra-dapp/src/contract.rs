#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, StateResponse};
use crate::state::{FieldState, GameState, State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:my-terra-dapp";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        fields: [FieldState::Empty; 9],
        game_state: GameState::InProgress,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("fields", "empty*9")
        .add_attribute("game_state", "inprogress"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Play { field_num } => try_play(deps, field_num),
        ExecuteMsg::Reset {} => try_reset(deps, info),
    }
}

pub fn try_play(deps: DepsMut, field_num: u8) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if state.game_state != GameState::InProgress {
            return Err(ContractError::CustomError {
                val: String::from("The current match has finished."),
            });
        }

        // TODO: keep track of who is X and who is O and place the proper symbol
        state.fields[field_num as usize] = match state.fields[field_num as usize] {
            FieldState::Empty | FieldState::O => FieldState::X,
            FieldState::X => FieldState::O,
        };

        // check for win condition
        let root_field = state.fields[0];
        if root_field != FieldState::Empty
            && ((root_field == state.fields[1] && root_field == state.fields[2])
                || (root_field == state.fields[4] && root_field == state.fields[8])
                || (root_field == state.fields[3] && root_field == state.fields[6]))
        {
            state.game_state = match root_field {
                FieldState::Empty => unreachable!(),
                FieldState::X => GameState::XWon,
                FieldState::O => GameState::OWon,
            };
        }
        let root_field = state.fields[1];
        if root_field != FieldState::Empty
            && root_field == state.fields[4]
            && root_field == state.fields[7]
        {
            state.game_state = match root_field {
                FieldState::Empty => unreachable!(),
                FieldState::X => GameState::XWon,
                FieldState::O => GameState::OWon,
            };
        }
        let root_field = state.fields[2];
        if root_field != FieldState::Empty
            && ((root_field == state.fields[4] && root_field == state.fields[6])
                || (root_field == state.fields[5] && root_field == state.fields[8]))
        {
            state.game_state = match root_field {
                FieldState::Empty => unreachable!(),
                FieldState::X => GameState::XWon,
                FieldState::O => GameState::OWon,
            };
        }
        let root_field = state.fields[3];
        if root_field != FieldState::Empty
            && root_field == state.fields[4]
            && root_field == state.fields[5]
        {
            state.game_state = match root_field {
                FieldState::Empty => unreachable!(),
                FieldState::X => GameState::XWon,
                FieldState::O => GameState::OWon,
            };
        }
        let root_field = state.fields[6];
        if root_field != FieldState::Empty
            && root_field == state.fields[7]
            && root_field == state.fields[8]
        {
            state.game_state = match root_field {
                FieldState::Empty => unreachable!(),
                FieldState::X => GameState::XWon,
                FieldState::O => GameState::OWon,
            };
        }

        // check for draw condition
        if state.fields.iter().all(|f| *f != FieldState::Empty) {
            state.game_state = GameState::Draw;
        }

        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_play"))
}

pub fn try_reset(deps: DepsMut, _info: MessageInfo) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.fields = [FieldState::Empty; 9];
        state.game_state = GameState::InProgress;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "reset"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetState {} => to_binary(&query_state(deps)?),
    }
}

fn query_state(deps: Deps) -> StdResult<StateResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(StateResponse {
        fields: state.fields,
        game_state: state.game_state,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetFields {}).unwrap();
        let value: FieldsResponse = from_binary(&res).unwrap();
        assert!(value.fields.iter().all(|f| *f == FieldState::Empty));
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetFields {}).unwrap();
        let value: FieldsResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset {};
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset {};
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetFields {}).unwrap();
        let value: FieldsResponse = from_binary(&res).unwrap();
        assert!(value.fields.iter().all(|f| *f == FieldState::Empty));
    }
}
