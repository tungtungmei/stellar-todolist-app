#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, Vec};

#[contract]
pub struct TodoContract;

#[contractimpl]
impl TodoContract {

    // tambah task
    pub fn add_task(env: Env, title: Symbol) {
        let key = Symbol::short("TASKS");

        let mut tasks: Vec<Symbol> =
            env.storage().instance().get(&key).unwrap_or(Vec::new(&env));

        tasks.push_back(title);

        env.storage().instance().set(&key, &tasks);
    }

    // ambil semua task
    pub fn get_tasks(env: Env) -> Vec<Symbol> {
        env.storage().instance().get(&Symbol::short("TASKS")).unwrap_or(Vec::new(&env))
    }
}