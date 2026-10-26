# Decisiones

| Fecha | Decisión | Motivo |
|---|---|---|
| 2026-10-25 | Pivot: presupuesto del lado del **comprador** (el dueño del agente), no del vendedor de APIs | el dolor es firmar cada pago o dar una llave sin límite |
| 2026-10-25 | Nombre: **QuickChain** | — |
| 2026-10-25 | Modelo: **1 %** de lo que gasta el agente, sin mensualidad | el dueño paga solo si usa |
| 2026-10-25 | **Writz y Bitcoin fuera** | no son parte del producto |
| 2026-10-25 | **Repo nuevo** `jxsu404/quickchain`; el viejo `caja-tica-escudo` queda intacto | empezar limpio; se rescató solo el cobro x402 |
| 2026-10-25 | Web nueva en Next.js con Cavos (la vieja era Vite + Freighter, sin Cavos) | stack del equipo |
| 2026-10-25 | Sin base de datos | el estado vive en el contrato |
| 2026-10-25 | Q1: el contrato pasa a ser la **cuenta-presupuesto** `C…` de PLAN-OPUS §2 (`__check_auth`, una cuenta por dueño, el USDC se deposita con `renovar`). Se reemplaza el diseño `gastar` + `approve`/`transfer_from` | lo pide el plan y los facilitadores x402 rechazan `transfer_from` (AUDITORIA) |
| 2026-10-25 | Q1: se mantiene la carpeta `contracts/budget` y el crate `budget` (struct `CuentaPresupuesto`) en vez de `contracts/cuenta-presupuesto/` | no romper `pnpm contrato:test`, enlaces ni CI; el plan lo marcaba como "propuesto" |
| 2026-10-25 | Q1: comisión 1 % (`fee_bps = 100`) redondeada hacia arriba, incluida en el tope, acumulada en `comision_pendiente` y cobrada en `renovar` o con `cobrar_comision()` (pública, a tesorería fija) | `__check_auth` no puede mover fondos (PLAN-OPUS §2.2); cierra la pregunta abierta 2 |
| 2026-10-25 | Q1: token = SAC del USDC de Circle en testnet (`USDC:GBBD47…` → `CBIELTK6…DAMA`). Ojo: el plan dice no usar `CBIELTK6…` "de Writz", pero ese ID **es** el SAC del USDC de Circle (`stellar contract id asset`). Lo confirma Josué | dato verificado con la CLI |

## Preguntas abiertas
1. **Cómo prueba el agente el pago a la API.** El esquema x402 `exact` espera un pago firmado por quien paga; en QuickChain paga el contrato. Opción MVP: el agente manda `X-QUICKCHAIN-TX` y `api-demo` verifica la tx en el RPC (destino, monto, contrato). Alternativa: el contrato autoriza y el agente firma un `exact` normal. Decidir en tarea 2/3.
2. ~~**Comisión 1 %**~~ resuelto en Q1 (ver tabla).
3. **Allowance:** `approve` del token tiene vencimiento por ledger; la web debe pedir uno que cubra el periodo.

<!-- enlaces:auto -->
## Enlaces

- ⬆️ Arriba: [Mapa (hub)](MAPA.md) · [Índice de docs](INDICE.md)
- 📖 Fuente de verdad: [PLAN-OPUS — la biblia](PLAN-OPUS.md)
- Relacionado: [Auditoría](AUDITORIA.md) · [Stack](STACK.md)
- Lo enlazan: [Auditoría](AUDITORIA.md) · [Índice de docs](INDICE.md) · [Mapa (hub)](MAPA.md) · [Plan (resumen)](PLAN.md) · [Stack](STACK.md)
- Tags: #quickchain #docs
<!-- /enlaces:auto -->
