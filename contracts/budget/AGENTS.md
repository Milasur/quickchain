# contracts/budget — contexto para agentes

Contrato Soroban (Rust, `soroban-sdk` 22) que guarda un presupuesto por agente.
- Funciones: `crear_presupuesto(owner, agent, token, cap, period)`, `gastar(agent, to, amount)`, `renovar(agent, cap)`, `cancelar(agent)`, `consultar(agent)`.
- `gastar` falla con `TopeExcedido` si se pasa del tope y con `PeriodoVencido` si terminó el periodo.
- El USDC sale del dueño con `transfer_from`: el dueño debe dar `approve` al contrato (una firma).
- Pendiente (TODO en código): comisión 1 %, script de despliegue a testnet.

Comandos: `cargo test -p budget` · build wasm: `stellar contract build` (requiere target wasm32v1-none).
Nota: `Cargo.lock` fija `ed25519-dalek` 2.2.0 para compilar con rustc 1.85. No lo borres.
