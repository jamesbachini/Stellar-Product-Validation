#![cfg(test)]
use soroban_sdk::{testutils::Address as _, Address, Env};
use crate::{MultiUserIncrementContract, MultiUserIncrementContractClient};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, MultiUserIncrementContract);
    let client = MultiUserIncrementContractClient::new(&env, &contract_id);
    let user1 = Address::random(&env);
    let user2 = Address::random(&env);
    // User1
    assert_eq!(client.increment(&user1), 1);
    assert_eq!(client.increment(&user1), 2);
    // User2
    assert_eq!(client.increment(&user2), 1);
    assert_eq!(client.increment(&user2), 2);
    assert_eq!(client.increment(&user2), 3);
    // Verify
    assert_eq!(client.read(&user1), 2);
    assert_eq!(client.read(&user2), 3);
}
