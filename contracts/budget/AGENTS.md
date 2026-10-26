# contracts/budget — `cuenta-presupuesto` (Q1)

Contrato Soroban (Rust, `soroban-sdk` 22). Implementa la **cuenta-presupuesto** de [PLAN-OPUS §2](../../docs/PLAN-OPUS.md): una cuenta-contrato `C…` por dueño que guarda el USDC del mes. Su `__check_auth` solo deja pasar `USDC.transfer(C, to, monto)` firmadas por la llave ed25519 del agente, con `to ≠ C`, `0 < monto ≤ max_por_llamada`, `gastado + monto + comisión ≤ tope` y antes de `fin_periodo`. Sin admin ni `upgrade`.

## Interfaz
```rust
fn __constructor(owner: Address, agente: BytesN<32>, token: Address, tesoreria: Address, fee_bps: u32);
fn renovar(tope: i128, max_por_llamada: i128, fin_periodo: u64, recarga: i128); // dueño; la "firma única" del mes
fn pausar(); fn reanudar(); fn cambiar_agente(agente: BytesN<32>);              // dueño
fn retirar(a: Address, monto: i128);                                          // dueño; no toca la comisión pendiente
fn cobrar_comision() -> i128;                                                 // pública; a la tesorería fija
fn estado() -> Estado; // owner, agente, token, tesoreria, fee_bps, periodo, tope, max_por_llamada, gastado, restante, fin_periodo, pausada, comision_pendiente, saldo
```
- Firma del agente: `Vec<FirmaAgente { public_key: BytesN<32>, signature: BytesN<64> }>` con **exactamente una** firma (forma de `authorizeEntry` con `Keypair`).
- `renovar`: valida `ahora < fin_periodo ≤ ahora + 32 días`, `0 < max_por_llamada ≤ tope`; cobra la comisión pendiente, deposita `recarga` desde el dueño, reinicia `gastado`, sube `periodo` y extiende el TTL (64 días o el máximo de la red).
- Comisión: `ceil(monto × fee_bps / 10 000)`; con `fee_bps = 100`, US$0,01 deja 1 000 stroops. Está **incluida en el tope**.
- `__check_auth` no emite eventos. `renovar`, `pausar`/`reanudar` y `cambiar_agente` sí.
- Errores: `Pausada=1, PeriodoVencido=2, FirmaInvalida=3, AgenteDesconocido=4, OperacionNoPermitida=5, MontoInvalido=6, MontoPorLlamada=7, TopeAlcanzado=8, PeriodoInvalido=9, ParametrosInvalidos=10`.
- Montos en unidades de 7 decimales (`1 USDC = 10_000_000`).

## Comandos
- Tests: `cargo test -p budget` (11 tests).
- Wasm: `stellar contract build --package budget` (target `wasm32v1-none`, rustc ≥ 1.84).
- Deploy a testnet: `bash scripts/deploy-cuenta.sh` (lee `.env`: `DEPLOYER_SECRET_KEY`, `AGENT_SECRET_KEY` o `AGENT_PUBLIC_KEY_HEX`; opcionales `OWNER_ADDRESS`, `QUICKCHAIN_TESORERIA`, `USDC_CONTRACT_ID`, `FEE_BPS`). Escribe `deployments/testnet.json`.
- Despliegue actual: ver [`deployments/testnet.json`](../../deployments/testnet.json) y [Demo](../../docs/DEMO.md).
- `Cargo.lock` fija `ed25519-dalek` 2.2.0. No lo borres.

## Tests (11, PLAN-OPUS §2.3)
| Test | Qué prueba |
|---|---|
| t01_agente_paga_dentro_del_tope | pago válido; suben `gastado` y la comisión |
| t02_pasarse_del_tope_con_comision | 4 pagos de 0,01 pasan, el 5.º da `TopeAlcanzado` (cuenta la comisión) |
| t03_monto_mayor_al_maximo_por_llamada | `MontoPorLlamada`; monto 0 da `MontoInvalido` |
| t04_firma_de_otra_llave | otra llave da `AgenteDesconocido`; firma falsa falla; sin firmas da `FirmaInvalida` |
| t05_operaciones_no_permitidas | `approve`, otro contrato, `from` distinto, `to = C` y llamar a la propia cuenta dan `OperacionNoPermitida` |
| t06_periodo_vencido_y_renovar | `PeriodoVencido`; tras `renovar` vuelve a pasar |
| t07_pausada | `Pausada`; `reanudar`; rotación de llave invalida la vieja |
| t08_renovar_exige_dueno_y_periodo_valido | sin firma del dueño falla; fin fuera de rango da `PeriodoInvalido` |
| t09_retirar_no_toca_la_comision | `retirar` deja intacta la comisión pendiente |
| t10_cobrar_comision_publica_a_tesoreria_fija | cualquiera cobra; va solo a la tesorería; `renovar` también cobra |
| t11_comision_redondea_hacia_arriba | 0,01 USDC → 1 000 stroops; 1 stroop → 1 |

## Cambio de interfaz (afecta a otros paquetes)
Ya no existen `crear_presupuesto`, `gastar`, `consultar`, `cancelar` ni el flujo `approve`/`transfer_from`.
- **agent-client (Q2):** en vez de llamar `gastar`, arma `USDC.transfer(C, payTo, monto)` con `from = C` y firma la auth entry de `C` con la llave del agente (`authorizeEntry`); doble simulación.
- **web (Q4/Q5):** desplegar una cuenta por dueño con constructor y firmar `renovar(tope, max, fin_de_mes, recarga)` (necesita trustline y saldo USDC del dueño si `recarga > 0`). Panel: leer `estado()`.

<!-- enlaces:auto -->
## Enlaces

- ⬆️ Arriba: [Mapa (hub)](../../docs/MAPA.md) · [Índice de docs](../../docs/INDICE.md)
- 📖 Fuente de verdad: [PLAN-OPUS — la biblia](../../docs/PLAN-OPUS.md)
- Relacionado: [agent-client](../../agent-client/AGENTS.md) · [Seguridad](../../docs/SEGURIDAD.md) · [Demo (testnet)](../../docs/DEMO.md) · [Decisiones](../../docs/DECISIONES.md)
- Issue: [#1](https://github.com/jxsu404/quickchain/issues/1)
- Lo enlazan: [AGENTS (raíz)](../../AGENTS.md) · [Tareas](../../TAREAS.md) · [agent-client](../../agent-client/AGENTS.md) · [Arquitectura](../../docs/ARQUITECTURA.md) · [Mapa (hub)](../../docs/MAPA.md) · [PLAN-OPUS — la biblia](../../docs/PLAN-OPUS.md) · [Seguridad](../../docs/SEGURIDAD.md) · [web](../../web/AGENTS.md)
- Tags: #quickchain #paquete
<!-- /enlaces:auto -->
