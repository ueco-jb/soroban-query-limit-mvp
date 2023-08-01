#![no_std]

mod test;
mod token;

use soroban_sdk::{contract, contractimpl, contractmeta, Address, BytesN, Env, IntoVal};
use token::create_contract;

// Metadata that is added on to the WASM custom section
contractmeta!(
    key = "Description",
    val = "Constant product AMM with a .3% swap fee"
);

pub trait LiquidityPoolTrait {
    fn deposit(
        e: Env,
        to: Address,
        token_wasm_hash: BytesN<32>,
        token_a: Address,
        token_b: Address,
    );
}

#[contract]
struct LiquidityPool;

#[contractimpl]
impl LiquidityPoolTrait for LiquidityPool {
    fn deposit(
        e: Env,
        to: Address,
        token_wasm_hash: BytesN<32>,
        token_a: Address,
        token_b: Address,
    ) {
        let share_contract = create_contract(&e, token_wasm_hash, &token_a, &token_b);
        let share_token_client = token::Client::new(&e, &share_contract);
        share_token_client.initialize(
            &e.current_contract_address(),
            &7u32,
            &"Pool Share Token".into_val(&e),
            &"POOL".into_val(&e),
        );

        let _ = share_token_client.balance(&to);
        let _ = share_token_client.balance(&to);
        let _ = share_token_client.balance(&to);
        let _ = share_token_client.balance(&to);
        let _ = share_token_client.balance(&to);
        let _ = share_token_client.balance(&to);
        let _ = share_token_client.balance(&to);
        let _ = share_token_client.balance(&to);
    }
}
