#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env, Symbol, Map, Vec,
    token, String
};

#[contract]
pub struct NFTContract;


#[contracttype]
pub enum DataKey {
    Tokens,
    TokenIdCounter,
    Treasury,
    BaseURI,
    CollectionName,
}


#[contractimpl]
impl NFTContract {
    pub fn initialize(env: Env, treasury: Address, base_uri: String, collection_name: Symbol) {
        env.storage().instance().set(&DataKey::Tokens, &Map::<u32, Address>::new(&env));
        env.storage().instance().set(&DataKey::TokenIdCounter, &0u32);
        env.storage().instance().set(&DataKey::Treasury, &treasury);
        env.storage().instance().set(&DataKey::BaseURI, &base_uri);
        env.storage().instance().set(&DataKey::CollectionName, &collection_name);
    }

    pub fn mint_batch(env: Env, to: Address, count: u32) -> Vec<u32> {
        let amount: i128 = (10 * count as i128) * 10_000_000; // 10 XLM per NFT in stroops
        let client = token::Client::new(&env, &env.current_contract_address());
        let treasury: Address = env.storage().instance().get(&DataKey::Treasury).unwrap();

        client.transfer(&to, &treasury, &amount);

        let mut minted_tokens = Vec::new(&env);

        for _ in 0..count {
            let token_id = Self::mint(env.clone(), to.clone());
            minted_tokens.push_back(token_id);
        }

        minted_tokens
    }

    pub fn mint(env: Env, to: Address) -> u32 {
        // let amount: i128 = 10_0000000; // 10 XLM in stroops (1 XLM = 10,000,000 stroops)
        // let client = token::Client::new(&env, &env.current_contract_address());
        // let treasury: Address = env.storage().instance().get(&DataKey::Treasury).unwrap();
        // client.transfer(&to, &treasury, &amount);

        let mut token_id = env.storage().instance().get::<_, u32>(&DataKey::TokenIdCounter).unwrap();
        token_id += 1;

        let mut tokens: Map<u32, Address> = env.storage().instance().get(&DataKey::Tokens).unwrap();
        tokens.set(token_id, to.clone());
        env.storage().instance().set(&DataKey::Tokens, &tokens);

        env.storage().instance().set(&DataKey::TokenIdCounter, &token_id);
        env.events().publish((symbol_short!("mint"), to, token_id), token_id);

        token_id
    }

    pub fn owner_of(env: Env, token_id: u32) -> Option<Address> {
        let tokens: Map<u32, Address> = env.storage().instance().get(&DataKey::Tokens).unwrap();
        tokens.get(token_id)
    }

    pub fn total_nfts(env: Env) -> u32 {
        env.storage().instance().get::<_, u32>(&DataKey::TokenIdCounter).unwrap_or(0)
    }

    pub fn get_base_uri(env: Env) -> Symbol {
        env.storage().instance().get(&DataKey::BaseURI).unwrap()
    }

    pub fn get_collection_name(env: Env) -> Symbol {
        env.storage().instance().get(&DataKey::CollectionName).unwrap()
    }
}

mod test;
