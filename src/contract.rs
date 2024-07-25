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

    use bitcoin::util::bip32::ChildNumber;
    use cosmwasm_std::Addr;
    use cosmwasm_std::Binary;
    use cosmwasm_std::Empty;
    use cosmwasm_testing_util::ContractWrapper;
    use cosmwasm_testing_util::MockApp;

    use crate::msg::QueryMsg;
    use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey};
    use bitcoin::{secp256k1, Network};

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

        let secp = secp256k1::Secp256k1::new();
        let network = Network::Bitcoin;
        let xpriv = ExtendedPrivKey::new_master(network, &[0]).unwrap();

        let xpub = ExtendedPubKey::from_priv(&secp, &xpriv);
        let pubkey_bytes = Binary::from(&xpub.public_key.serialize());
        let child = ChildNumber::from_normal_idx(1).unwrap();

        let (sk, _) = xpub.ckd_pub_tweak(child).unwrap();
        let secret_bytes = Binary::from(sk.secret_bytes());

        let pubkey = xpub.derive_pub(&secp, &[child]).unwrap().public_key;

        let pubkey_bytes: Binary = app
            .query(
                addr.clone(),
                &QueryMsg::AddExpTweak {
                    pubkey: pubkey_bytes,
                    secret: secret_bytes,
                },
            )
            .unwrap();

        let pubkey_ret = secp256k1::PublicKey::from_slice(&pubkey_bytes).unwrap();

        assert_eq!(pubkey_ret, pubkey);
    }
}
