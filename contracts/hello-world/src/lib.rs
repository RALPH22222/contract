#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Map, String};

#[contracttype]
pub enum DataKey {
    Pool(String),
    MemberActivePool,
}

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub struct Pool {
    pub contribution_amount: i128,
    pub total_members: u32,
    pub current_cycle: u32,
    pub cycle_contributions: u32,
    pub payout_done_this_cycle: bool,
    pub members: Map<Address, Member>,
    pub token_address: Address,
}

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub struct Member {
    pub sequence: u32,
    pub total_contributed: i128,
}

#[contract]
pub struct PaluwaganContract;

#[contractimpl]
impl PaluwaganContract {
    pub fn create_pool(
        env: Env,
        pool_id: String,
        total_members: u32,
        contribution_amount: i128,
        token_address: Address,
    ) {
        let key = DataKey::Pool(pool_id.clone());
        if env.storage().instance().has(&key) {
            panic!("Pool already exists");
        }
        let pool = Pool {
            contribution_amount,
            total_members,
            current_cycle: 1,
            cycle_contributions: 0,
            payout_done_this_cycle: false,
            members: Map::<Address, Member>::new(&env),
            token_address,
        };
        env.storage().instance().set(&key, &pool);
    }

    pub fn add_member(env: Env, pool_id: String, member: Address, sequence: u32) {
        let key = DataKey::Pool(pool_id.clone());
        let mut pool: Pool = env.storage().instance().get(&key).unwrap_or_else(|| panic!("Pool not found"));

        let mut active_map = env.storage()
            .instance()
            .get(&DataKey::MemberActivePool)
            .unwrap_or(Map::<Address, String>::new(&env));

        if let Some(existing_pool_id) = active_map.get(member.clone()) {
            let old_key = DataKey::Pool(existing_pool_id);
            if let Some(old_pool) = env.storage().instance().get(&old_key) {
                let old_pool: Pool = old_pool;
                if old_pool.current_cycle != 0 {
                    panic!("Member is already in an active pool");
                }
            }
        }

        if sequence < 1 || sequence > pool.total_members {
            panic!("Invalid sequence");
        }
        if pool.members.contains_key(member.clone()) {
            panic!("Member already exists in this pool");
        }

        pool.members.set(member.clone(), Member {
            sequence,
            total_contributed: 0,
        });

        active_map.set(member, pool_id);
        env.storage().instance().set(&DataKey::MemberActivePool, &active_map);
        env.storage().instance().set(&key, &pool);
    }

    pub fn contribute(env: Env, pool_id: String, member: Address) {
        let key = DataKey::Pool(pool_id.clone());
        let mut pool: Pool = env.storage().instance().get(&key).unwrap_or_else(|| panic!("Pool not found"));
        let mut m = pool.members.get(member.clone()).unwrap_or_else(|| panic!("Member not found"));
        m.total_contributed += pool.contribution_amount;
        pool.members.set(member, m);
        pool.cycle_contributions += 1;
        env.storage().instance().set(&key, &pool);
    }

    pub fn payout(env: Env, pool_id: String) {
        let key = DataKey::Pool(pool_id.clone());
        let mut pool: Pool = env.storage().instance().get(&key).unwrap_or_else(|| panic!("Pool not found"));
        if pool.cycle_contributions < pool.total_members {
            panic!("Not all members have contributed");
        }
        if pool.payout_done_this_cycle {
            panic!("Payout already done for this cycle");
        }
        let current_cycle = pool.current_cycle;
        let winner = pool.members.iter()
            .find(|(_, m)| m.sequence == current_cycle)
            .map(|(addr, _)| addr)
            .unwrap_or_else(|| panic!("No winner found for cycle"));

        let pot = pool.contribution_amount * (pool.total_members as i128);

        let token_client = token::TokenClient::new(&env, &pool.token_address);
        token_client.transfer(&env.current_contract_address(), &winner, &pot);

        pool.payout_done_this_cycle = true;
        let next_cycle = current_cycle + 1;
        if next_cycle > pool.total_members {
            pool.current_cycle = 0;
        } else {
            pool.current_cycle = next_cycle;
        }
        pool.cycle_contributions = 0;
        pool.payout_done_this_cycle = false;
        env.storage().instance().set(&key, &pool);
    }

    pub fn get_pool_state(env: Env, pool_id: String) -> PoolState {
        let key = DataKey::Pool(pool_id);
        let pool: Pool = env.storage().instance().get(&key).unwrap_or_else(|| panic!("Pool not found"));
        PoolState {
            contribution_amount: pool.contribution_amount,
            total_members: pool.total_members,
            current_cycle: pool.current_cycle,
            cycle_contributions: pool.cycle_contributions,
            members: pool.members,
        }
    }
}

#[contracttype]
pub struct PoolState {
    pub contribution_amount: i128,
    pub total_members: u32,
    pub current_cycle: u32,
    pub cycle_contributions: u32,
    pub members: Map<Address, Member>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    #[test]
    fn test_pool_lifecycle() {
        let env = Env::default();
        let contract_id = env.register(PaluwaganContract, ());
        let client = PaluwaganContractClient::new(&env, &contract_id);

        let pool_id = String::from_str(&env, "test-pool-uuid");
        let total_members = 3u32;
        let contribution_amount: i128 = 1000;

        let token_address = Address::generate(&env);

        client.create_pool(
            &pool_id,
            &total_members,
            &contribution_amount,
            &token_address,
        );

        let member1 = Address::generate(&env);
        let member2 = Address::generate(&env);
        let member3 = Address::generate(&env);

        client.add_member(&pool_id, &member1, &1u32);
        client.add_member(&pool_id, &member2, &2u32);
        client.add_member(&pool_id, &member3, &3u32);

        assert!(client
            .try_add_member(&pool_id, &Address::generate(&env), &4u32)
            .is_err());

        client.contribute(&pool_id, &member1);
        client.contribute(&pool_id, &member2);
        client.contribute(&pool_id, &member3);

        let state = client.get_pool_state(&pool_id);
        assert_eq!(state.current_cycle, 1);
        assert_eq!(state.cycle_contributions, 3);
    }
}