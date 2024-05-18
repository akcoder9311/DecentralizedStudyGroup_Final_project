#![cfg(test)]
#![no_std]

extern crate std;

use soroban_sdk::{testutils::Address as TestAdress, vec, Env, IntoVal, Vec};
use crate::{StudyGroupContract, Contribution, StudyGroup};

#[test]
fn test_create_group() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StudyGroupContract);

    let creator = TestAdress::generate(&env);
    let from = TestAdress::generate(&env);

    StudyGroupContract::create_group(
        env,
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

    let creator = TestAdress::generate(&env);
    let from = TestAdress::generate(&env);
    let new_participant = TestAdress::generate(&env);

    StudyGroupContract::create_group(
        env,
        from.clone(),
        creator.clone(),
        "Test Group".into_val(&env),
        "A test group description".into_val(&env),
        vec![&env]
    );

    let group_id = 1u64;
    StudyGroupContract::join_group(env, new_participant.clone(), group_id);

    let stored_group: StudyGroup = env.storage().instance().get(&group_id).unwrap();

    assert_eq!(stored_group.participants.len(), 2);
    assert!(stored_group.participants.contains(&creator));
    assert!(stored_group.participants.contains(&new_participant));
}

#[test]
fn test_contribute() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StudyGroupContract);

    let creator = TestAdress::generate(&env);
    let from = TestAdress::generate(&env);

    StudyGroupContract::create_group(
        env,
        from.clone(),
        creator.clone(),
        "Test Group".into_val(&env),
        "A test group description".into_val(&env),
        vec![&env]
    );

    let group_id = 1u64;
    let contribution_amount = 100i128;

    StudyGroupContract::contribute(env, from.clone(), group_id, contribution_amount);

    let stored_group: StudyGroup = env.storage().instance().get(&group_id).unwrap();
    assert_eq!(stored_group.contributions.len(), 1);
    assert_eq!(stored_group.contributions.get(0).unwrap().contributor, from);
    assert_eq!(stored_group.contributions.get(0).unwrap().amount, contribution_amount);
}

#[test]
fn test_view_group() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StudyGroupContract);

    let creator = TestAdress::generate(&env);
    let from = TestAdress::generate(&env);

    StudyGroupContract::create_group(
        env,
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


#[test]

pub fn test_leave_group(){
    let env = Env::default();
        let group_id: u64 = 1;

        // Create some test addresses
        let alice = TestAdress::generate(&env);
        let bob = TestAdress::generate(&env);
        let charlie = TestAdress::generate(&env);

        // Create a study group and add it to the storage
        let mut participants = Vec::new(&env);
        participants.push_back(alice.clone());
        participants.push_back(bob.clone());
        participants.push_back(charlie.clone());

        let group = StudyGroup { participants  };
        env.storage().instance().set(&group_id, &group);

        // Alice leaves the group
        leave_group(env.clone(), alice.clone(), &group_id);

        // Retrieve the updated group from storage
        let updated_group: StudyGroup = env.storage().instance().get(&group_id).unwrap();

        // Check that Alice is no longer a participant
        assert_eq!(updated_group.participants.len(), 2);
        // println!("{}",updated_group.participant.len());
        assert!(updated_group.participants.contains(&bob));
        assert!(updated_group.participants.contains(&charlie));
        assert!(updated_group.participants.contains(&alice));
}