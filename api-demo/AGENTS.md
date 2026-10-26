# api-demo — contexto para agentes

API de prueba en Next.js (App Router) que cobra por llamada con x402 en Stellar testnet.
- `lib/x402.ts`: función `cobrar(req, descripcion, entregar)`. Rescatada de `caja-tica-escudo/escudo/riesgo.ts`.
- `app/api/clima/route.ts`: endpoint de ejemplo. Agregá más endpoints envolviendo con `cobrar`.
- Variables: `X402_PAY_TO`, `X402_PRECIO`, `X402_FACILITATOR_URL` (ver `.env.example` en la raíz).
- Correr: `pnpm dev:api` → http://localhost:4020/api/clima (sin pago responde 402).
- Desplegar: Vercel, root directory `api-demo`.
