#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Address, Env, Symbol};

#[contract]
pub struct MultiUserIncrementContract;

#[contractimpl]
impl MultiUserIncrementContract {
    fn counter_key(user: &Address) -> (Symbol, Address) {
        (symbol_short!("CTR"), user.clone())
    }

    pub fn increment(env: Env, caller: Address) -> u32 {
        //caller.require_auth();
        let key = Self::counter_key(&caller);
        let mut count: u32 = env.storage().instance().get(&key).unwrap_or(0);
        log!(&env, "User: {}, count: {}", caller, count);
        count += 1;
        env.storage().instance().set(&key, &count);
        env.storage().instance().extend_ttl(50, 100);
        count
    }

    pub fn read(env: Env, user: Address) -> u32 {
        let key = Self::counter_key(&user);
        env.storage().instance().get(&key).unwrap_or(0)
    }
}
