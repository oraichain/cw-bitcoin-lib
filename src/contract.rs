#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use crate::{add_exp_tweak, error::ContractError, msg::QueryMsg};

use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _: MessageInfo,
    _: Empty,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(_: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AddExpTweak { pubkey, secret } => to_json_binary(&add_exp_tweak(
            &pubkey.as_slice().try_into().unwrap(),
            &secret.as_slice().try_into().unwrap(),
        )?),
    }
}

#[cfg(test)]
mod tests {

    use cosmwasm_std::Addr;
    use cosmwasm_std::Binary;
    use cosmwasm_std::Empty;
    use cosmwasm_testing_util::ContractWrapper;
    use cosmwasm_testing_util::MockApp;

    use crate::msg::QueryMsg;

    #[test]
    fn test_init() {
        let mut app = MockApp::new(&[]);
        let code_id = app.upload(Box::new(ContractWrapper::new_with_empty(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        )));

        let addr = app
            .instantiate(
                code_id,
                Addr::unchecked("alice"),
                &Empty {},
                &[],
                "cw-bitcoin-lib",
            )
            .unwrap();

        let pubkey: Binary = app
            .query(
                addr.clone(),
                &QueryMsg::AddExpTweak {
                    pubkey: Binary::from([
                        2, 136, 145, 243, 107, 105, 26, 64, 3, 111, 43, 62, 203, 23, 193, 55, 128,
                        169, 50, 80, 62, 242, 195, 159, 63, 174, 217, 185, 91, 247, 30, 162, 127,
                    ]),
                    secret: Binary::from([
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 1,
                    ]),
                },
            )
            .unwrap();

        println!("pubkey :{:?}", pubkey.as_slice());
    }
}
