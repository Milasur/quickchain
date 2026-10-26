# web — contexto para agentes

Next.js (App Router, TS) desplegado en Vercel (root directory `web`).
- Login: Cavos `@cavos/kit` (`CavosProvider` en `app/providers.tsx`, `useCavos()` en páginas). Correo, sin frase semilla, Stellar testnet.
- `app/page.tsx`: entrar/salir y ver la cuenta G….
- `app/presupuesto/page.tsx`: formulario de tope. TODO: firmar `approve` + `crear_presupuesto` con la wallet Cavos.
- Variable: `NEXT_PUBLIC_CAVOS_APP_ID` (crear app en el dashboard de Cavos y registrar las URLs de callback).
- No se rescató la web vieja (Vite + Freighter, sin Cavos): se escribió nueva.
Correr: `pnpm dev:web`.
