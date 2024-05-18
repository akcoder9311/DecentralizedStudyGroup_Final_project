#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, vec, Address, Env, String, Vec };
// use soroban_sdk::*;

mod test;

#[derive(Clone)]
#[contracttype]
pub enum StorageKey {
    GroupID,
}

#[derive(Clone)]
#[contracttype]
pub struct StudyGroup {
    creator: Address,
    title: String,
    description: String,
    participants: Vec<Address>,
    contributions: Vec<Contribution>,
}

#[derive(Clone)]
#[contracttype]
pub struct Contribution {
    contributor: Address,
    amount: i128,
}

#[contract]
pub struct StudyGroupContract;

#[contractimpl]
impl StudyGroupContract {
    pub fn create_group(
        env: Env,
        from: Address,
        creator: Address,
        title: String,
        description: String,
        initial_contributors: Vec<Contribution>
    ) {
        // Ensure that the transaction sender is authorized to create groups
        from.require_auth();

        // Generate a new unique group ID
        let group_id =
            env.storage().instance().get::<_, u64>(&StorageKey::GroupID).unwrap_or(0) + 1;

        let initial_participants: Vec<Address> = vec![&env, creator.clone()];
        // Create the study group
        let group = StudyGroup {
            creator,
            title,
            description,
            participants: initial_participants,
            contributions: initial_contributors,
        };

        // Store the group in the contract storage
        env.storage().instance().set(&StorageKey::GroupID, &group_id);
        env.storage().instance().set(&group_id, &group);
    }

    pub fn join_group(env: Env, from: Address, group_id: u64) {
        // Ensure that the transaction sender is authorized to join groups
        from.require_auth();

        // Retrieve the group from the contract storage
        let mut group: StudyGroup = env.storage().instance().get(&group_id).unwrap();

        // Add the participant to the group
        let participant = from;
        if !group.participants.contains(&participant) {
            group.participants.push_back(participant);
        }

        // Update the group in the contract storage
        env.storage().instance().set(&group_id, &group);
    }

    pub fn contribute(env: Env, from: Address, group_id: u64, amount: i128) {
        // Ensure that the transaction sender is authorized to contribute
        from.require_auth();

        // Retrieve the group from the contract storage
        let mut group: StudyGroup = env.storage().instance().get(&group_id).unwrap();

        // Add the contribution to the group
        let contribution = Contribution {
            contributor: from,
            amount,
        };
        group.contributions.push_back(contribution);

        // Update the group in the contract storage
        env.storage().instance().set(&group_id, &group);
    }

    pub fn distribute_rewards(env: Env, from: Address, group_id: u64) {
        // Ensure that the transaction sender is authorized to distribute rewards
        from.require_auth();

        // Retrieve the group from the contract storage
        let group: StudyGroup = env.storage().instance().get(&group_id).unwrap();

        // Calculate total contributions
        let total_contributions: i128 = group.contributions
            .iter()
            .map(|c| c.amount)
            .sum();

        // Distribute rewards to participants based on their contributions
        for contribution in group.contributions {
            let reward_amount: i128 = (contribution.amount * 10) / total_contributions; // Arbitrary reward calculation
            token::Client
                ::new(&env, &env.current_contract_address())
                .transfer(
                    &env.current_contract_address(),
                    &contribution.contributor,
                    &reward_amount
                );
        }
    }


    //view the present group
    pub fn view_group(env:Env, group_id:u64)->StudyGroup{
        env.storage().instance().get(&group_id).unwrap()
    }


    // leave group from the 
    pub fn leave_graoup(env:Env, from:Address, group_id:u64 ){
          from.require_auth();

         let mut group : StudyGroup  = env.storage().instance().get(&group_id).unwrap();

        let mut new_participant:Vec<Address> = Vec::new(&env);
         for i in group.participants {
               if i != from {
                   new_participant.push_back(i);
               }
         }


        group.participants = new_participant;  

        env.storage().instance().set(&group_id ,&group);
   }

}

