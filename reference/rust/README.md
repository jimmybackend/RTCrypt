# Implementación de referencia en Rust

La implementación de referencia de RTCrypt se planifica en Rust por estas razones:

- buen control de errores;
- seguridad de memoria;
- ergonomía razonable para CLI, librerías y servicios;
- ecosistema fuerte para tooling y testing.

## Workspace esperado

```text
reference/rust/
├─ Cargo.toml
├─ rtcrypt-core/
├─ rtcrypt-cose/
├─ rtcrypt-auth/
├─ rtcrypt-kms/
└─ rtcrypt-cli/
```

## Regla

La implementación de referencia no debe adelantarse a las specs.  
Si un crate necesita una decisión que no está documentada, esa decisión debe reflejarse primero en `specs/` o en un ADR.
