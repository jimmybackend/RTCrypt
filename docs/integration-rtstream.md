# Integración con RTStream

## Objetivo

Definir una v0 realista para la seguridad de RTStream sin sobrediseñar el sistema.

RTStream probablemente manejará tráfico continuo, eventos o segmentos. El error habitual en este tipo de módulos es intentar resolver E2E, autorización, transporte y serialización en una sola etapa. RTCrypt debe evitar eso.

---

## 1. Decisión de v0

La v0 de RTStream debe priorizar:

- **TLS 1.3 obligatorio** para canales de red
- **mTLS** para tráfico interno entre workloads confiables
- validación explícita de identidad de peers
- autorización de publish/subscribe/consume en el plano de aplicación

Esto es suficiente para una base segura y operable.

---

## 2. Qué no entra en v0

No entra todavía:

- cifrado por evento definido por RTCrypt para todos los casos;
- E2E por frame con formato propio;
- replay protection compleja fuera del transporte;
- federación multi-dominio avanzada.

Si en el futuro RTStream necesita seguridad de objeto persistente o cross-broker, se define como fase posterior y con perfil separado.

---

## 3. Modelo de canal

### Cliente o productor
Se conecta usando TLS 1.3.

### Servicio interno
Debe usar mTLS cuando transporte datos o comandos sensibles.

### Identidad de workload
Idealmente basada en SPIFFE/SPIRE o mecanismo equivalente.

---

## 4. Autorización esperada

Aunque TLS/mTLS autentiquen el canal o al peer, RTStream sigue necesitando autorización de negocio.

Ejemplos:

- quién puede publicar en un stream;
- quién puede consumir desde un tenant;
- quién puede administrar bindings o topics;
- quién puede leer metadatos de stream.

---

## 5. Modelo mínimo recomendado

```text
ConnectionIdentity
- transport_authenticated: bool
- workload_id / client_id / user_id
- tenant_id
- source
- auth_method

StreamPermission
- stream_id
- action: publish | consume | manage | inspect
- effect: allow | deny
```

---

## 6. Riesgos principales

- Confundir autenticación de transporte con autorización de negocio.
- Aceptar conexiones internas sin identidad verificable.
- Permitir downgrade a TLS inseguro.
- Asumir que el broker aplica políticas suficientes por sí solo.
- No registrar la identidad asociada a publish/consume.

---

## 7. Futuro posible

Cuando RTStream requiera seguridad de objeto, la extensión debe definirse como:

- perfil de evento o segmento;
- metadatos protegidos;
- firma o cifrado con COSE;
- rules claras para replay y ordering.

Pero eso debe quedar como perfil aparte, no mezclado en la v0 de transporte.

---

## 8. Resultado esperado para Codex

Codex debe construir RTStream v0 de manera que:

- TLS y mTLS sean configuración obligatoria o muy explícita;
- la identidad del peer pueda mapearse a un principal RT;
- la autorización de operaciones quede modelada;
- el diseño permita crecer hacia seguridad de objeto sin romper compatibilidad.
