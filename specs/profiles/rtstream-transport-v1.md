# Perfil `rtstream-transport-v1`

## Estado
Draft inicial.

## Objetivo
Definir la seguridad mínima de transporte para RTStream.

---

## 1. Decisión central

Este perfil exige:

- TLS 1.3 para conexiones de red
- mTLS para canales internos sensibles
- validación explícita de peer identity
- autorización separada para acciones del stream

---

## 2. Alcance

Aplica a:
- conexiones cliente-servidor;
- conexiones servicio-servicio;
- publish / consume / manage a nivel de autorización.

No define:
- cifrado de objeto por evento;
- firma por frame;
- replay mitigation fuera del transporte y de la lógica de aplicación.

---

## 3. Reglas mínimas

- no aceptar downgrade de versión por debajo de TLS 1.3;
- no aceptar peers sin identidad verificable cuando la ruta exige mTLS;
- registrar identidad asociada a la conexión;
- evaluar permisos de operación.

---

## 4. Identidad de peer

Puede derivarse de:
- certificado de cliente;
- SPIFFE ID;
- token complementario si el diseño lo requiere.

La identidad debe traducirse a un principal o estructura equivalente.

---

## 5. Errores esperados

- tls_required
- mtls_required
- peer_identity_invalid
- peer_not_authorized
- stream_policy_denied

---

## 6. Resultado esperado

Una implementación conforme a este perfil puede:
- establecer transporte seguro;
- identificar peers de forma fuerte;
- separar autenticación de conexión y autorización de operación.
