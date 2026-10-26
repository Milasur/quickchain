"use client";
import { useState } from "react";
import { useCavos } from "@cavos/kit/react";

export default function Presupuesto() {
  const { isAuthenticated } = useCavos();
  const [agente, setAgente] = useState("");
  const [tope, setTope] = useState("10");
  if (!isAuthenticated) return <p>Primero entrá con tu correo.</p>;
  return (
    <main>
      <h2>Presupuesto mensual del agente</h2>
      <label>Cuenta del agente (G…)<br /><input value={agente} onChange={(e) => setAgente(e.target.value)} size={60} /></label>
      <br />
      <label>Tope por mes (USDC)<br /><input value={tope} onChange={(e) => setTope(e.target.value)} /></label>
      <p>Comisión QuickChain: 1 % de lo que gaste el agente. Sin mensualidad.</p>
      {/* TODO(tarea 4): firmar con la wallet de Cavos `approve` + `crear_presupuesto` del contrato (una sola firma por mes). */}
      <button disabled>Firmar tope (pendiente)</button>
    </main>
  );
}
