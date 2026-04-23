# rtcrypt-auth

## Propósito

Validar identidades de usuario, cliente y workload para RTCore y otros componentes.

## Responsabilidades

- validación OIDC/OAuth;
- construcción de `Principal`;
- validación de peer identity;
- integración con SPIFFE/SVID cuando aplique;
- errores de autenticación bien tipados.

## Qué no debe contener

- autorización completa de negocio;
- handlers HTTP acoplados a un framework específico;
- lógica de serialización COSE.

## Primera entrega sugerida

- `TokenVerifier`
- `PrincipalBuilder`
- `WorkloadAuthenticator`
- fixtures de claims válidos e inválidos
