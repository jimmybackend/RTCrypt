# Conformance

La suite de conformidad verifica si una implementación respeta los perfiles publicados en `specs/profiles/`.

## Objetivo

Separar claramente:
- “funciona en mi máquina”
- “es conforme al perfil”

## Cobertura esperada

- parseo
- typing
- headers protegidos
- claims obligatorios
- allow-list de algoritmos
- tiempos y expiración
- scope de claves
- default deny en autorización

## Resultado esperado

Una implementación debería poder ejecutar esta suite en CI antes de publicarse.
