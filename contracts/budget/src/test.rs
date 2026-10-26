#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, token::StellarAssetClient, Env};

#[test]
fn bloquea_sobregasto() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let usdc = env.register_stellar_asset_contract_v2(admin).address();
    let id = env.register(Budget, ());
    let c = BudgetClient::new(&env, &id);
    let (owner, agent, api) = (Address::generate(&env), Address::generate(&env), Address::generate(&env));
    StellarAssetClient::new(&env, &usdc).mint(&owner, &1_000_0000000);
    token::Client::new(&env, &usdc).approve(&owner, &id, &100_0000000, &1000);
    c.crear_presupuesto(&owner, &agent, &usdc, &100_0000000, &2_592_000);
    assert_eq!(c.gastar(&agent, &api, &60_0000000), 40_0000000);
    assert_eq!(c.try_gastar(&agent, &api, &50_0000000), Err(Ok(Error::TopeExcedido)));
}
