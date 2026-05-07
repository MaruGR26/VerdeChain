# VerdeChain 🌱📊
### Infraestructura de cumplimiento automatizado para la mitigación de riesgos de Greenwashing y PLD en activos biológicos.

![Solana](https://img.shields.io/badge/Solana-9945FF?style=for-the-badge&logo=solana&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)
![Status: Prototype](https://img.shields.io/badge/Status-Prototype-green?style=for-the-badge)

VerdeChain es un protocolo de **Bio-Escrow** diseñado para garantizar la transparencia en proyectos de reforestación en la Riviera Maya, México. 

## Problema
Los proyectos de compensación de carbono suelen sufrir de **Greenwashing** (falta de pruebas biológicas) y riesgos de **Lavado de Dinero** por flujos de efectivo no auditados.

## Solución
Este Smart Contract en Solana automatiza la auditoría mediante dos filtros críticos:
1. **Filtro PLD (Financiero):** Filtro de Cumplimiento Normativo (PLD): Implementación de reglas de negocio basadas en la LFPIORPI (México) para la restricción de operaciones en efectivo. El contrato actúa como una aduana lógica que impide la inicialización de proyectos que no cumplan con los umbrales de debida diligencia..
2. **Hito Biológico (NDVI):** Los fondos se mantienen en custodia (escrow) hasta que la métrica de salud vegetal alcance el umbral crítico definido (50% de la densidad óptima de 0.8 NDVI). Esto asegura la existencia real del activo biológico antes de la liquidación financiera.

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
- **Resultado esperado:** El contrato detecta que se superó el umbral de salud vegetal (50%, es decir, NDVI 0.4, umbral de crecimiento vigoroso; el 0.8 representa madurez total de acuerdo a diversos autores para el caso de manglares) y cambiará automáticamente `fondosLiberados` a `true`. Esto garantiza que el proveedor solo cobre si hay éxito biológico real.

## Futuras Iteraciones (Roadmap del escenario ideal)
- Descentralización de Oráculos: Integración con Chainlink para la ingesta automatizada de datos satelitales (eliminando la captura manual).

- Identidad Digital (KYC): Incorporación de protocolos de Identidad Descentralizada (DID) como Civic para la identificación plena del Beneficiario Final.

- Arquitectura Híbrida: Migración de almacenamiento de evidencias (fotos/reportes) a IPFS/Arweave para optimizar costos de almacenamiento en mainnet.
