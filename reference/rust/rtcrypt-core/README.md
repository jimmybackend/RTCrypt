# rtcrypt-core

## Propósito

`rtcrypt-core` contiene los tipos y contratos comunes usados por el resto de módulos.

## Responsabilidades

- tipos de dominio;
- errores compartidos;
- carga de profiles;
- traits base;
- utilidades no criptográficas;
- contexto multi-tenant;
- validación estructural común.

## Tipos mínimos esperados

- `Principal`
- `PrincipalKind`
- `KeyRef`
- `SecurityProfile`
- `VerificationContext`
- `TenantContext`
- `AuditEvent`
- `RtError`

## Qué no debe contener

- llamadas a proveedores cloud concretos;
- lógica HTTP;
- lógica de CLI;
- dependencias fuertes a librerías COSE específicas.

## Primera entrega sugerida

1. `types.rs`
2. `errors.rs`
3. `profile.rs`
4. `context.rs`
5. tests unitarios
