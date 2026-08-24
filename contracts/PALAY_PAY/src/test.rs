#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, token, Address, Env};

    fn setup_test() -> (Env, PalayPayContractClient<'static>, Address, Address, token::Client<'static>, token::AdminClient<'static>) {
        let env = Env::default();
        env.mock_all_signatures();

        let contract_id = env.register_contract(None, PalayPayContract);
        let client = PalayPayContractClient::new(&env, &contract_id);

        let buyer = Address::generate(&env);
        
        // Setup mock USDC token
        let token_admin = Address::generate(&env);
        let token_contract_id = env.register_stellar_asset_contract_v2(token_admin.clone());
        let usdc_client = token::Client::new(&env, &token_contract_id.address());
        let usdc_admin_client = token::AdminClient::new(&env, &token_contract_id.address());

        // Mint USDC to buyer balance
        usdc_admin_client.mint(&buyer, &1000_0000000);

        client.initialize(&buyer, &token_contract_id.address());

        (env, client, buyer, token_contract_id.address(), usdc_client, usdc_admin_client)
    }

    #[test]
    fn test_1_happy_path_mvp() {
        let (env, client, buyer, _usdc, usdc_client, _admin) = setup_test();
        let farmer = Address::generate(&env);

        // 1. Buyer creates delivery receipt (250 USDC)
        let receipt_id = client.create_receipt(&farmer, &250_0000000);
        assert_eq!(receipt_id, 1);

        // 2. Farmer claims payout
        client.claim_payout(&receipt_id);

        // Verify farmer received payment
        assert_eq!(usdc_client.balance(&farmer), 250_0000000);
    }

    #[test]
    #[should_panic(expected = "Receipt already claimed")]
    fn test_2_edge_case_double_claim_fails() {
        let (env, client, _buyer, _usdc, _usdc_client, _admin) = setup_test();
        let farmer = Address::generate(&env);

        let receipt_id = client.create_receipt(&farmer, &100_0000000);
        
        // Claim twice to trigger expected panic
        client.claim_payout(&receipt_id);
        client.claim_payout(&receipt_id);
    }

    #[test]
    fn test_3_state_verification() {
        let (env, client, _buyer, _usdc, _usdc_client, _admin) = setup_test();
        let farmer = Address::generate(&env);

        let receipt_id = client.create_receipt(&farmer, &500_0000000);
        let receipt_before = client.get_receipt(&receipt_id);
        assert_eq!(receipt_before.is_claimed, false);

        client.claim_payout(&receipt_id);

        let receipt_after = client.get_receipt(&receipt_id);
        assert_eq!(receipt_after.is_claimed, true);
        assert_eq!(receipt_after.amount_usdc, 500_0000000);
    }

    #[test]
    #[should_panic]
    fn test_4_unauthorized_receipt_creation_fails() {
        let env = Env::default();
        // Skip mock_all_signatures to test authorization failure
        let contract_id = env.register_contract(None, PalayPayContract);
        let client = PalayPayContractClient::new(&env, &contract_id);

        let buyer = Address::generate(&env);
        let non_buyer = Address::generate(&env);
        let farmer = Address::generate(&env);
        let token_admin = Address::generate(&env);
        let token_contract = env.register_stellar_asset_contract_v2(token_admin);

        client.initialize(&buyer, &token_contract.address());

        // Call directly without matching signature
        client.create_receipt(&farmer, &100_0000000);
    }

    #[test]
    fn test_5_multiple_receipts_tracking() {
        let (env, client, _buyer, _usdc, _usdc_client, _admin) = setup_test();
        let farmer1 = Address::generate(&env);
        let farmer2 = Address::generate(&env);

        let id1 = client.create_receipt(&farmer1, &150_0000000);
        let id2 = client.create_receipt(&farmer2, &300_0000000);

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);

        client.claim_payout(&id2);
        assert_eq!(client.get_receipt(&id1).is_claimed, false);
        assert_eq!(client.get_receipt(&id2).is_claimed, true);
    }
}