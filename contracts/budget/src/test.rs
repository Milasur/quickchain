#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger, MockAuth, MockAuthInvoke},
    token::{StellarAssetClient, TokenClient},
    Env, IntoVal,
};

const MES: u64 = 2_592_000; // 30 días
const USDC: i128 = 10_000_000; // 1 USDC con 7 decimales

struct T { env: Env, c: BudgetClient<'static>, usdc: Address, owner: Address, agent: Address, api: Address }

fn preparar() -> T {
    let env = Env::default();
    env.mock_all_auths();
    let usdc = env.register_stellar_asset_contract_v2(Address::generate(&env)).address();
    let id = env.register(Budget, ());
    let c = BudgetClient::new(&env, &id);
    let (owner, agent, api) = (Address::generate(&env), Address::generate(&env), Address::generate(&env));
    StellarAssetClient::new(&env, &usdc).mint(&owner, &(1_000 * USDC));
    TokenClient::new(&env, &usdc).approve(&owner, &id, &(1_000 * USDC), &1_000_000);
    c.crear_presupuesto(&owner, &agent, &usdc, &(100 * USDC), &MES);
    T { env, c, usdc, owner, agent, api }
}

#[test]
fn tope_se_respeta_y_paga() {
    let t = preparar();
    assert_eq!(t.c.gastar(&t.agent, &t.api, &(60 * USDC)), 40 * USDC);
    assert_eq!(TokenClient::new(&t.env, &t.usdc).balance(&t.api), 60 * USDC);
    assert_eq!(t.c.gastar(&t.agent, &t.api, &(40 * USDC)), 0); // justo al tope
}

#[test]
fn sobregasto_rechazado() {
    let t = preparar();
    t.c.gastar(&t.agent, &t.api, &(60 * USDC));
    assert_eq!(t.c.try_gastar(&t.agent, &t.api, &(50 * USDC)), Err(Ok(Error::TopeExcedido)));
    assert_eq!(TokenClient::new(&t.env, &t.usdc).balance(&t.api), 60 * USDC);
}

#[test]
fn periodo_vencido_bloquea_y_renovar_reinicia() {
    let t = preparar();
    t.c.gastar(&t.agent, &t.api, &(100 * USDC));
    t.env.ledger().with_mut(|l| l.timestamp += MES);
    assert_eq!(t.c.try_gastar(&t.agent, &t.api, &USDC), Err(Ok(Error::PeriodoVencido)));
    t.c.renovar(&t.agent, &(50 * USDC));
    let p = t.c.consultar(&t.agent);
    assert_eq!((p.cap, p.gastado), (50 * USDC, 0));
    assert_eq!(t.c.gastar(&t.agent, &t.api, &(10 * USDC)), 40 * USDC);
}

#[test]
fn cancelar_bloquea() {
    let t = preparar();
    t.c.cancelar(&t.agent);
    assert_eq!(t.c.try_gastar(&t.agent, &t.api, &USDC), Err(Ok(Error::Inactivo)));
}

#[test]
fn monto_invalido_y_duplicado() {
    let t = preparar();
    assert_eq!(t.c.try_gastar(&t.agent, &t.api, &0), Err(Ok(Error::MontoInvalido)));
    assert_eq!(t.c.try_crear_presupuesto(&t.owner, &t.agent, &t.usdc, &USDC, &MES), Err(Ok(Error::YaExiste)));
}

#[test]
fn agente_sin_firma_rechazado() {
    let t = preparar();
    let intruso = Address::generate(&t.env);
    // Solo el intruso firma; el contrato exige la firma del agente.
    t.env.set_auths(&[]);
    let r = t.c.mock_auths(&[MockAuth {
        address: &intruso,
        invoke: &MockAuthInvoke { contract: &t.c.address, fn_name: "gastar", args: (&t.agent, &t.api, USDC).into_val(&t.env), sub_invokes: &[] },
    }]).try_gastar(&t.agent, &t.api, &USDC);
    assert!(r.is_err());
}

#[test]
fn solo_dueno_renueva_o_cancela() {
    let t = preparar();
    t.env.set_auths(&[]);
    let r = t.c.mock_auths(&[MockAuth {
        address: &t.agent,
        invoke: &MockAuthInvoke { contract: &t.c.address, fn_name: "cancelar", args: (&t.agent,).into_val(&t.env), sub_invokes: &[] },
    }]).try_cancelar(&t.agent);
    assert!(r.is_err());
}
