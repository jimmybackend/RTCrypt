# Perfil `rtcloud-kms-v1`

## Estado
Draft inicial.

## Objetivo
Definir la interfaz mínima entre RTCrypt y proveedores de claves externos.

---

## 1. Principio base

RTCrypt no custodia claves privadas como núcleo del sistema.  
Las operaciones sensibles deben delegarse a un proveedor externo cuando el entorno lo requiera.

---

## 2. Operaciones mínimas

- `sign`
- `resolve_public_key`
- `wrap_key`
- `unwrap_key`
- `get_key_metadata`

---

## 3. Metadata mínima de clave

```text
KeyMetadata
- kid
- provider
- purpose
- tenant_scope
- algorithm_set
- exportable: false/true
- status
```

---

## 4. Reglas

- no usar una clave para propósitos incompatibles;
- validar scope de tenant;
- registrar auditoría de uso;
- no exponer secretos por logs o errores.

---

## 5. Requisitos del adapter

- desacoplado del core;
- testeable con mock provider;
- errores normalizados;
- timeouts y reintentos controlados;
- separación entre metadata y operación criptográfica.

---

## 6. Resultado esperado

Una implementación conforme a este perfil puede:
- firmar o envolver claves mediante proveedor externo;
- publicar metadata útil al verificador;
- cambiar de proveedor con mínimo impacto en capas superiores.
