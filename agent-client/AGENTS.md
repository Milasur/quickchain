# agent-client — contexto para agentes

Cliente TypeScript que usa el agente de IA. Ante un `402 Payment Required`:
1. lee lo que pide la API (`accepts[]`: `payTo`, `amount`),
2. llama `gastar` del contrato budget firmando con la llave del agente (el contrato bloquea si pasa el tope),
3. reintenta la llamada con el hash de la tx (`X-QUICKCHAIN-TX`).

Archivos: `src/presupuesto.ts` (llamada al contrato), `src/fetchConPresupuesto.ts` (402 → pagar → reintentar), `src/demo.ts`.
Variables: `BUDGET_CONTRACT_ID`, `AGENT_SECRET_KEY`, `STELLAR_RPC_URL`, `API_DEMO_URL`. La llave va en `.env`, nunca en git.
Correr: `pnpm agente`.
Pendiente: que api-demo acepte `X-QUICKCHAIN-TX` (ver docs/DECISIONES.md).
