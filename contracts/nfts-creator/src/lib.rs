#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env, Symbol, Map
};

#[contract]
pub struct NFTContract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenMetadata {
    name: Symbol,
    description: Symbol,
    uri: Symbol,
}

#[contracttype]
pub enum DataKey {
    // Admin,
    Tokens,
    Metadata,
    TokenIdCounter,
}


#[contractimpl]
impl NFTContract {
    pub fn initialize(env: Env) {
        env.storage().instance().set(&DataKey::Tokens, &Map::<u32, Address>::new(&env));
        env.storage().instance().set(&DataKey::Metadata, &Map::<u32, TokenMetadata>::new(&env));
        env.storage().instance().set(&DataKey::TokenIdCounter, &0u32);
    }

    pub fn mint(env: Env, to: Address, name: Symbol, description: Symbol, uri: Symbol) -> u32 {
        let mut token_id = env.storage().instance().get::<_, u32>(&DataKey::TokenIdCounter).unwrap();
        token_id += 1;

        let mut tokens: Map<u32, Address> = env.storage().instance().get(&DataKey::Tokens).unwrap();
        tokens.set(token_id, to.clone());
        env.storage().instance().set(&DataKey::Tokens, &tokens);

        let mut metadata: Map<u32, TokenMetadata> = env.storage().instance().get(&DataKey::Metadata).unwrap();
        metadata.set(token_id, TokenMetadata { name: name.clone(), description: description.clone(), uri: uri.clone() });
        env.storage().instance().set(&DataKey::Metadata, &metadata);

        env.storage().instance().set(&DataKey::TokenIdCounter, &token_id);

        env.events().publish((symbol_short!("mint"), to, token_id), name);

        token_id
    }

    pub fn owner_of(env: Env, token_id: u32) -> Option<Address> {
        let tokens: Map<u32, Address> = env.storage().instance().get(&DataKey::Tokens).unwrap();
        tokens.get(token_id)
    }

    pub fn token_metadata(env: Env, token_id: u32) -> Option<TokenMetadata> {
        let metadata: Map<u32, TokenMetadata> = env.storage().instance().get(&DataKey::Metadata).unwrap();
        metadata.get(token_id)
    }

    pub fn total_nfts(env: Env) -> u32 {
        env.storage().instance().get::<_, u32>(&DataKey::TokenIdCounter).unwrap_or(0)
    }
}

mod test;
