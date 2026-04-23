# rtcrypt-cose

## Propósito

Implementar protección de objetos RT con COSE.

## Responsabilidades

- parseo de envelopes;
- sign / verify;
- encrypt / decrypt;
- inspect;
- validación de headers protegidos requeridos.

## Entrada y salida

### Entrada
- manifest u objeto RT
- profile
- signing context o recipient context

### Salida
- envelope serializado
- resultado verificado y tipado
- metadata inspeccionable para CLI

## Qué no debe contener

- autorización;
- política de negocio;
- llamadas directas a AWS KMS acopladas;
- parsing de tokens OIDC.

## Orden recomendado

1. parser seguro
2. sign/verify
3. inspect
4. encrypt/decrypt
5. test vectors
