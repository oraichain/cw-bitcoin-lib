use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;
use cw_bitcoin_lib::msg::QueryMsg;

//run cargo schema to generate
fn main() {
    write_api! {
        instantiate: Empty,
        query: QueryMsg,
    }
}
