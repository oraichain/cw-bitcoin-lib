pub mod contract;
pub mod error;
pub mod msg;

use cosmwasm_std::{Binary, StdError};
use error::ContractResult;
use libsecp256k1_core::curve::{Affine, ECMultContext, Field, Scalar};
use libsecp256k1_core::util::{TAG_PUBKEY_EVEN, TAG_PUBKEY_ODD};

pub type Pubkey = [u8; 33];
pub type Secret = [u8; 32];

pub fn parse_pubkey(bytes: &Pubkey) -> ContractResult<Affine> {
    let mut x = Field::default();
    if !x.set_b32(arrayref::array_ref!(bytes, 1, 32)) {
        return Err(StdError::generic_err("invalid pubkey").into());
    }
    let mut elem = libsecp256k1_core::curve::Affine::default();
    elem.set_xo_var(&x, bytes[0] == TAG_PUBKEY_ODD);
    Ok(elem)
}

pub fn add_exp_tweak(pubkey: &Pubkey, tweak: &Secret) -> ContractResult<Binary> {
    let mut elem = parse_pubkey(pubkey)?;
    let mut scala = Scalar::default();
    if bool::from(scala.set_b32(&tweak)) {
        return Err(StdError::generic_err("invalid secret").into());
    }

    let ctx = ECMultContext::new_boxed();
    let mut r = libsecp256k1_core::curve::Jacobian::default();
    let a = libsecp256k1_core::curve::Jacobian::from_ge(&elem);
    let one = libsecp256k1_core::curve::Scalar::from_int(1);
    ctx.ecmult(&mut r, &a, &one, &scala);

    elem.set_gej(&r);

    let mut ret = [0u8; 33];

    elem.x.normalize_var();
    elem.y.normalize_var();
    elem.x.fill_b32(arrayref::array_mut_ref!(ret, 1, 32));
    ret[0] = if elem.y.is_odd() {
        TAG_PUBKEY_ODD
    } else {
        TAG_PUBKEY_EVEN
    };
    Ok(Binary::from(ret))
}
