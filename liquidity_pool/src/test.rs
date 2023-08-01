#![cfg(test)]
extern crate std;

use crate::{token, LiquidityPoolClient};

use soroban_sdk::{contractimport, testutils::Address as _, Address, BytesN, Env};

fn create_token_contract<'a>(e: &Env, admin: &Address) -> token::Client<'a> {
    token::Client::new(e, &e.register_stellar_asset_contract(admin.clone()))
}

fn install_token_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm"
    );
    e.deployer().upload_contract_wasm(WASM)
}

#[test]
fn test() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::random(&e);
    let to = Address::random(&e);

    let token_a = create_token_contract(&e, &admin);
    let token_b = create_token_contract(&e, &admin);

    let liqpool = LiquidityPoolClient::new(&e, &e.register_contract(None, crate::LiquidityPool {}));
    liqpool.deposit(
        &to,
        &install_token_wasm(&e),
        &token_a.address,
        &token_b.address,
    );
}
