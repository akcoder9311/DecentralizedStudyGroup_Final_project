#![cfg(test)]
#![no_std]

extern crate std;

use soroban_sdk::{testutils::EnvExt, Address, Env, IntoVal, vec};
use crate::{StudyGroupContract, Contribution, StudyGroup};

#[test]
fn test_create_group() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StudyGroupContract);

    let creator = Address::random(&env);
    let from = Address::random(&env);

    StudyGroupContract::create_group(
        &env,
        from.clone(),
        creator.clone(),
        "Test Group".into_val(&env),
        "A test group description".into_val(&env),
        vec![&env]
    );

    let group_id = 1u64;
    let stored_group: StudyGroup = env.storage().instance().get(&group_id).unwrap();

    assert_eq!(stored_group.creator, creator);
    assert_eq!(stored_group.title, "Test Group".into_val(&env));
    assert_eq!(stored_group.description, "A test group description".into_val(&env));
    assert_eq!(stored_group.participants.len(), 1);
    assert_eq!(stored_group.participants.get(0).unwrap(), creator);
}

#[test]
fn test_join_group() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StudyGroupContract);

    let creator = Address::random(&env);
    let from = Address::random(&env);
    let new_participant = Address::random(&env);

    StudyGroupContract::create_group(
        &env,
        from.clone(),
        creator.clone(),
        "Test Group".into_val(&env),
        "A test group description".into_val(&env),
        vec![&env]
    );

    let group_id = 1u64;
    StudyGroupContract::join_group(&env, new_participant.clone(), group_id);

    let stored_group: StudyGroup = env.storage().instance().get(&group_id).unwrap();

    assert_eq!(stored_group.participants.len(), 2);
    assert!(stored_group.participants.contains(&creator));
    assert!(stored_group.participants.contains(&new_participant));
}

#[test]
fn test_contribute() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StudyGroupContract);

    let creator = Address::random(&env);
    let from = Address::random(&env);

    StudyGroupContract::create_group(
        &env,
        from.clone(),
        creator.clone(),
        "Test Group".into_val(&env),
        "A test group description".into_val(&env),
        vec![&env]
    );

    let group_id = 1u64;
    let contribution_amount = 100i128;

    StudyGroupContract::contribute(&env, from.clone(), group_id, contribution_amount);

    let stored_group: StudyGroup = env.storage().instance().get(&group_id).unwrap();
    assert_eq!(stored_group.contributions.len(), 1);
    assert_eq!(stored_group.contributions.get(0).unwrap().contributor, from);
    assert_eq!(stored_group.contributions.get(0).unwrap().amount, contribution_amount);
}

#[test]
fn test_view_group() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StudyGroupContract);

    let creator = Address::random(&env);
    let from = Address::random(&env);

    StudyGroupContract::create_group(
        &env,
        from.clone(),
        creator.clone(),
        "Test Group".into_val(&env),
        "A test group description".into_val(&env),
        vec![&env]
    );

    let group_id = 1u64;
    let stored_group: StudyGroup = StudyGroupContract::view_group(&env, group_id);

    assert_eq!(stored_group.creator, creator);
    assert_eq!(stored_group.title, "Test Group".into_val(&env));
    assert_eq!(stored_group.description, "A test group description".into_val(&env));
    assert_eq!(stored_group.participants.len(), 1);
    assert_eq!(stored_group.participants.get(0).unwrap(), creator);
}
