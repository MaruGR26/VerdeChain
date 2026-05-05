# VerdeChain
### Infraestructura de cumplimiento automatizado para la mitigación de riesgos de Greenwashing y PLD en activos biológicos.

VerdeChain es un protocolo de **Bio-Escrow** diseñado para garantizar la transparencia en proyectos de reforestación en la Riviera Maya, México. 

## Problema
Los proyectos de compensación de carbono suelen sufrir de **Greenwashing** (falta de pruebas biológicas) y riesgos de **Lavado de Dinero** por flujos de efectivo no auditados.

## Solución
Este Smart Contract en Solana automatiza la auditoría mediante dos filtros críticos:
1. **Filtro PLD (Financiero):** Bloquea transacciones en efectivo que superen los límites de la ley mexicana ($871,274 MXN).
2. **Hito Biológico (NDVI):** Los fondos del proyecto permanecen congelados y solo se liberan al proveedor si el índice NDVI (salud de vegetación) alcanza el 50%.

## Tecnologías
- **Lenguaje:** Rust
- **Framework:** Anchor
- **Blockchain:** Solana (Devnet)
- **Concepto:** RWA (Real World Assets) / ReFi

## Pruebas de Auditoría (Cómo Probar)

Para validar la lógica de cumplimiento del contrato, se pueden realizar las siguientes pruebas en el panel de **Testing** de Solana Playground:

### 1. Prueba de Cumplimiento Financiero (PLD/Antilavado)
- **Acción:** Ejecutar `inicializar_proyecto` marcando `esEfectivo: true` y un `montoMxn` mayor a `871274`.
- **Resultado esperado:** La transacción fallará con el error `ExcesoLimiteEfectivo`. Esto demuestra que el contrato bloquea automáticamente pagos en efectivo que violan la ley mexicana.

### 2. Prueba de Hito Biológico (NDVI/Eco-Escrow)
- **Paso A:** Crear un proyecto con datos válidos (ej. Monto: 100,000, Efectivo: false). Al consultar la cuenta, verás que `fondosLiberados` es `false`.
- **Paso B:** Ejecutar `actualizar_ndvi` con un valor de `20`. Los fondos seguirán bloqueados porque el mangle aún no está sano.
- **Paso C:** Ejecutar `actualizar_ndvi` con un valor de `60`.
- **Resultado esperado:** El contrato detectará que se superó el umbral de salud vegetal (50%) y cambiará automáticamente `fondosLiberados` a `true`. Esto garantiza que el proveedor solo cobre si hay éxito biológico real.
