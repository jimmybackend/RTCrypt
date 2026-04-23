# Ejemplo RT Pack

## Objetivo

Mostrar el flujo conceptual de un manifest firmado para RTPack.

## Archivos esperados

- `manifest.sample.json`
- `README.md`

## Flujo

1. Construir manifest.
2. Serializar.
3. Firmar con profile `rtpack-cose-v1`.
4. Verificar.
5. Inspeccionar headers y claims.

## Notas

Este ejemplo prioriza firma e integridad antes que cifrado.
