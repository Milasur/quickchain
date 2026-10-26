// fetch que, ante un 402, paga desde el presupuesto y reintenta una vez.
import { gastar, linkExpert, type ConfigAgente } from "./presupuesto.js";

interface Requisito { payTo: string; amount: string; network: string }

export async function fetchConPresupuesto(url: string, cfg: ConfigAgente, init: RequestInit = {}): Promise<Response> {
  const r1 = await fetch(url, init);
  if (r1.status !== 402) return r1;
  const cuerpo = (await r1.json()) as { accepts?: Requisito[] };
  const req = cuerpo.accepts?.find((a) => a.network === "stellar:testnet");
  if (!req) throw new Error("La API no acepta pagos en Stellar testnet");
  const hash = await gastar(cfg, req.payTo, BigInt(req.amount));
  console.log(`Pagado ${req.amount} a ${req.payTo}: ${linkExpert(hash)}`);
  // TODO(tarea 2/3): acordar con api-demo cómo probar el pago. Hoy enviamos el hash;
  // api-demo debe verificarlo en el RPC (ver docs/DECISIONES.md, pregunta abierta).
  const headers = new Headers(init.headers);
  headers.set("X-QUICKCHAIN-TX", hash);
  return fetch(url, { ...init, headers });
}
