#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Address, Map};

#[contract]
pub struct CharityDonationTracker;

#[contractimpl]
impl CharityDonationTracker {

    // Store total donations per donor
    pub fn donate(env: Env, donor: Address, amount: i128) {
        donor.require_auth();

        let key = donor.clone();

        let mut donations: Map<Address, i128> =
            env.storage().instance().get(&symbol_short!("DONATIONS"))
            .unwrap_or(Map::new(&env));

        let current = donations.get(key.clone()).unwrap_or(0);
        donations.set(key.clone(), current + amount);

        env.storage().instance().set(&symbol_short!("DONATIONS"), &donations);
    }

    // Get donation amount of a donor
    pub fn get_donation(env: Env, donor: Address) -> i128 {
        let donations: Map<Address, i128> =
            env.storage().instance().get(&symbol_short!("DONATIONS"))
            .unwrap_or(Map::new(&env));

        donations.get(donor).unwrap_or(0)
    }

    // Get total donations (aggregate)
    pub fn get_total(env: Env) -> i128 {
        let donations: Map<Address, i128> =
            env.storage().instance().get(&symbol_short!("DONATIONS"))
            .unwrap_or(Map::new(&env));

        let mut total: i128 = 0;

        for (_, value) in donations.iter() {
            total += value;
        }

        total
    }
}