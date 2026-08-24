#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol, token};

// Data keys used to isolate contract state variables in storage
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Buyer,
    UsdcToken,
    Receipt(u64),
}

// Receipt structure representing physical grain deliveries
#[derive(Clone)]
#[contracttype]
pub struct GrainReceipt {
    pub farmer: Address,
    pub amount_usdc: i128,
    pub is_claimed: bool,
}

const COUNTER_KEY: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct PalayPayContract;

#[contractimpl]
impl PalayPayContract {
    /// Initializes contract with buyer authority and token mint address
    pub fn initialize(env: Env, buyer: Address, usdc_token: Address) {
        if env.storage().instance().has(&DataKey::Buyer) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::Buyer, &buyer);
        env.storage().instance().set(&DataKey::UsdcToken, &usdc_token);
        env.storage().instance().set(&COUNTER_KEY, &0u64);
    }

    /// Buyer creates a grain delivery receipt and locks required funds in contract
    pub fn create_receipt(env: Env, farmer: Address, amount_usdc: i128) -> u64 {
        let buyer: Address = env.storage().instance().get(&DataKey::Buyer).unwrap();
        buyer.require_auth();

        let usdc_token: Address = env.storage().instance().get(&DataKey::UsdcToken).unwrap();
        let client = token::Client::new(&env, &usdc_token);
        
        // Transfer buyer USDC into contract escrow
        client.transfer(&buyer, &env.current_contract_address(), &amount_usdc);

        let mut id: u64 = env.storage().instance().get(&COUNTER_KEY).unwrap_or(0);
        id += 1;

        let receipt = GrainReceipt {
            farmer,
            amount_usdc,
            is_claimed: false,
        };

        env.storage().persistent().set(&DataKey::Receipt(id), &receipt);
        env.storage().instance().set(&COUNTER_KEY, &id);

        id
    }

    /// Farmer claims immediate USDC payment for delivered grain
    pub fn claim_payout(env: Env, receipt_id: u64) {
        let mut receipt: GrainReceipt = env
            .storage()
            .persistent()
            .get(&DataKey::Receipt(receipt_id))
            .expect("Receipt not found");

        receipt.farmer.require_auth();

        if receipt.is_claimed {
            panic!("Receipt already claimed");
        }

        let usdc_token: Address = env.storage().instance().get(&DataKey::UsdcToken).unwrap();
        let client = token::Client::new(&env, &usdc_token);

        // Transfer funds from contract to farmer
        client.transfer(&env.current_contract_address(), &receipt.farmer, &receipt.amount_usdc);

        receipt.is_claimed = true;
        env.storage().persistent().set(&DataKey::Receipt(receipt_id), &receipt);
    }

    /// Helper view function to fetch receipt details
    pub fn get_receipt(env: Env, receipt_id: u64) -> GrainReceipt {
        env.storage()
            .persistent()
            .get(&DataKey::Receipt(receipt_id))
            .expect("Receipt not found")
    }
}