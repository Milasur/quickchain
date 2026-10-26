# Seguridad — modelo de amenazas

| Amenaza | Mitigación | Estado |
|---|---|---|
| Agente comprometido gasta todo | tope por periodo en el contrato; `gastar` exige firma del agente | hecho (tests) |
| Otra cuenta se hace pasar por el agente | `agent.require_auth()` | hecho (test `agente_sin_firma_rechazado`) |
| El agente cambia su propio tope | `renovar`/`cancelar` exigen firma del dueño | hecho (test `solo_dueno_renueva_o_cancela`) |
| Gasto después de fin de mes | `PeriodoVencido` | hecho (test) |
| Desbordamiento de enteros | `checked_add` + `overflow-checks = true` | hecho |
| Allowance mayor que el tope | el contrato limita igual por `cap`; la web debe pedir `approve` = tope | pendiente (web) |
| API cobra sin entregar | `api-demo` prepara la respuesta antes de liquidar | hecho (heredado) |
| Pago falso con `X-QUICKCHAIN-TX` | verificar tx en RPC (destino, monto, contrato, no reutilizada) | pendiente (pregunta abierta 1) |
| Filtración de llaves | `.gitignore` cubre `.env*`; `.env.example` solo nombres | hecho |
| Mensajes que suenen a inversión | reglas en AGENTS.md y CONTRIBUTING.md | hecho |

Riesgos aceptados para la demo: solo testnet; un presupuesto por agente; sin auditoría externa del contrato.
