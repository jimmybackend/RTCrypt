# Ejemplo RTCloud KMS

## Objetivo

Mostrar cómo el core delega una operación de firma a un proveedor externo.

## Flujo conceptual

1. Resolver `KeyRef`.
2. Validar propósito y scope.
3. Enviar payload al provider.
4. Recibir firma.
5. Registrar auditoría.

## Nota

El core no necesita conocer detalles del proveedor más allá del contrato definido por `KeyProvider`.
