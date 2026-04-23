# rtcrypt-kms

## Propósito

Proveer una abstracción uniforme para usar proveedores de claves externos.

## Responsabilidades

- trait `KeyProvider`;
- metadata de claves;
- adapters a proveedores;
- mock provider para tests;
- normalización de errores.

## Qué no debe contener

- lógica de autorización;
- lógica de transporte;
- suposiciones sobre un único cloud.

## Primera entrega sugerida

1. trait `KeyProvider`
2. `MockKeyProvider`
3. modelos `KeyMetadata`, `ResolvedPublicKey`
4. tests de uso y de errores
