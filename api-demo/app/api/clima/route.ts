// API de prueba: dato simple cobrado por llamada con x402.
import { cobrar } from "@/lib/x402";

export async function GET(req: Request) {
  return cobrar(req, "Clima de prueba (QuickChain demo)", async () => ({
    ciudad: "San José", nota: "Dato de demostración, no es un servicio real", hora: new Date().toISOString(),
  }));
}
