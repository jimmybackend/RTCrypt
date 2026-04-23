# rtcrypt-cli

## Propósito

Exponer operaciones básicas para inspección, firma, verificación y pruebas manuales.

## Comandos previstos

- `sign`
- `verify`
- `inspect`
- `encrypt`
- `decrypt`
- `validate-token` (opcional posterior)

## Reglas

- la CLI no debe contener lógica criptográfica duplicada;
- debe usar crates internos;
- los mensajes deben ser útiles sin revelar secretos;
- debe poder operar sobre fixtures y test vectors del repositorio.
