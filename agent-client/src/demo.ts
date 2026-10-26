// Demo: el agente llama a la API varias veces hasta chocar con el tope.
import "dotenv/config";
import { fetchConPresupuesto } from "./fetchConPresupuesto.js";
import { leerConfigAgente } from "./presupuesto.js";

const url = process.env.API_DEMO_URL || "http://localhost:4020/api/clima";
const cfg = leerConfigAgente();
for (let i = 1; i <= Number(process.env.LLAMADAS || 5); i++) {
  try {
    const r = await fetchConPresupuesto(url, cfg);
    console.log(`#${i}`, r.status, await r.text());
  } catch (e) {
    console.log(`#${i} bloqueado:`, (e as Error).message);
  }
}
