#![no_std]
//! Presupuesto mensual en USDC para un agente de IA.
//! El dueño firma UNA vez un tope por periodo; el agente gasta sin más firmas
//! hasta llegar al tope o al fin del periodo. El contrato bloquea el sobregasto.
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, token, Address, Env};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Presupuesto {
    pub owner: Address,
    pub agent: Address,
    pub token: Address,
    pub cap: i128,        // tope por periodo (unidades del token, 7 decimales)
    pub period: u64,      // duración del periodo en segundos (ej. 30 días)
    pub inicio: u64,      // timestamp del inicio del periodo actual
    pub gastado: i128,    // gastado en el periodo actual
    pub activo: bool,
}

#[contracttype]
pub enum Clave {
    Presupuesto(Address), // por agente
}

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum Error {
    YaExiste = 1,
    NoExiste = 2,
    Inactivo = 3,
    PeriodoVencido = 4,
    TopeExcedido = 5,
    MontoInvalido = 6,
}

#[contract]
pub struct Budget;

#[contractimpl]
impl Budget {
    /// El dueño firma una vez: autoriza que el agente gaste hasta `cap` por `period`.
    pub fn crear_presupuesto(env: Env, owner: Address, agent: Address, token: Address, cap: i128, period: u64) -> Result<(), Error> {
        owner.require_auth();
        if cap <= 0 || period == 0 { return Err(Error::MontoInvalido); }
        let k = Clave::Presupuesto(agent.clone());
        if env.storage().persistent().has(&k) { return Err(Error::YaExiste); }
        let p = Presupuesto { owner, agent, token, cap, period, inicio: env.ledger().timestamp(), gastado: 0, activo: true };
        env.storage().persistent().set(&k, &p);
        // TODO: el dueño debe dar `approve` del token al contrato por `cap` (o usar allowance propia).
        Ok(())
    }

    /// El agente paga `amount` a `to`. Falla si pasa el tope o venció el periodo.
    pub fn gastar(env: Env, agent: Address, to: Address, amount: i128) -> Result<i128, Error> {
        agent.require_auth();
        if amount <= 0 { return Err(Error::MontoInvalido); }
        let k = Clave::Presupuesto(agent.clone());
        let mut p: Presupuesto = env.storage().persistent().get(&k).ok_or(Error::NoExiste)?;
        if !p.activo { return Err(Error::Inactivo); }
        if env.ledger().timestamp() >= p.inicio + p.period { return Err(Error::PeriodoVencido); }
        let nuevo = p.gastado.checked_add(amount).ok_or(Error::MontoInvalido)?;
        if nuevo > p.cap { return Err(Error::TopeExcedido); }
        p.gastado = nuevo;
        env.storage().persistent().set(&k, &p);
        // Mueve USDC del dueño al destino usando la allowance dada al contrato.
        token::Client::new(&env, &p.token).transfer_from(&env.current_contract_address(), &p.owner, &to, &amount);
        // TODO: comisión del 1% a la cuenta de QuickChain.
        Ok(p.cap - p.gastado)
    }

    /// Una firma del dueño abre un nuevo periodo (puede cambiar el tope).
    pub fn renovar(env: Env, agent: Address, cap: i128) -> Result<(), Error> {
        let k = Clave::Presupuesto(agent);
        let mut p: Presupuesto = env.storage().persistent().get(&k).ok_or(Error::NoExiste)?;
        p.owner.require_auth();
        if cap <= 0 { return Err(Error::MontoInvalido); }
        p.cap = cap; p.gastado = 0; p.inicio = env.ledger().timestamp(); p.activo = true;
        env.storage().persistent().set(&k, &p);
        Ok(())
    }

    /// El dueño apaga el presupuesto; el agente ya no puede gastar.
    pub fn cancelar(env: Env, agent: Address) -> Result<(), Error> {
        let k = Clave::Presupuesto(agent);
        let mut p: Presupuesto = env.storage().persistent().get(&k).ok_or(Error::NoExiste)?;
        p.owner.require_auth();
        p.activo = false;
        env.storage().persistent().set(&k, &p);
        Ok(())
    }

    pub fn consultar(env: Env, agent: Address) -> Result<Presupuesto, Error> {
        env.storage().persistent().get(&Clave::Presupuesto(agent)).ok_or(Error::NoExiste)
    }
}

#[cfg(test)]
mod test;
