#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, Address, Map};

#[contract]
pub struct MintContract;

#[contractimpl]
impl MintContract {
    // Zincir üstü token mint işlemi
    pub fn mint(env: Env, to: Address, amount: i128) -> i128 {
        let key = Symbol::short("balance");
        let mut balances: Map<Address, i128> = env.storage().instance().get(&key).unwrap_or_default();
        let current = balances.get(to.clone()).unwrap_or(0);
        let new_balance = current + amount;
        balances.set(to.clone(), new_balance);
        env.storage().instance().set(&key, &balances);
        new_balance
    }

    // Kullanıcının zincir üzerindeki mevcut token bakiyesini getirir
    pub fn balance(env: Env, user: Address) -> i128 {
        let key = Symbol::short("balance");
        let balances: Map<Address, i128> = env.storage().instance().get(&key).unwrap_or_default();
        balances.get(user).unwrap_or(0)
    }
}