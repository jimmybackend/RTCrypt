# RTCrypt

**RTCrypt** es la capa de seguridad del ecosistema **RT Stack** de **Rethra Communications**.

Su objetivo es integrar **confidencialidad, integridad, autenticación y control de acceso** en RTPack, RTCore, RTStream y RTCloud **sin inventar criptografía propia** y sin convertir la seguridad en una colección de utilidades inconexas.

RTCrypt define **perfiles, contratos, validaciones y adaptadores** para usar estándares abiertos y maduros como:

- **TLS 1.3** para seguridad en tránsito.
- **COSE/CBOR** para objetos binarios firmados o cifrados.
- **HPKE** para cifrado híbrido orientado a destinatarios.
- **OAuth 2.0 / OpenID Connect** para autenticación y autorización federada.
- **SPIFFE/SPIRE** para identidad de workloads y mTLS en entornos distribuidos.
- **KMS/HSM** externos para custodia de claves y operaciones sensibles.

> **Regla fundacional:** RTCrypt **no diseña algoritmos criptográficos nuevos**. Solo define cómo usar estándares existentes de forma coherente dentro de RT Stack.

---

## 1. Problema que resuelve

En sistemas distribuidos suele ocurrir uno de estos escenarios:

1. Cada servicio implementa seguridad de forma diferente.
2. El cifrado solo existe a nivel de transporte y los objetos quedan sin protección fuera del canal.
3. La autenticación se mezcla con autorización y con formato de mensajes.
4. Las claves privadas terminan acopladas al código o al runtime.
5. No existen perfiles claros ni pruebas de conformidad.

RTCrypt nace para evitar ese desorden. El proyecto define una capa transversal que permite que todos los componentes de RT Stack hablen el mismo lenguaje de seguridad.

---

## 2. Alcance de RTCrypt

RTCrypt **sí** debe encargarse de:

- Definir perfiles de seguridad por componente y caso de uso.
- Decidir qué estándares usa RT Stack en cada capa.
- Definir formatos de metadatos y claims RT.
- Validar identidades, tokens, envelopes y políticas.
- Resolver claves a través de interfaces desacopladas.
- Integrar autorización de forma uniforme.
- Exponer contratos claros para SDKs, CLI y pruebas de conformidad.

RTCrypt **no** debe encargarse de:

- Crear algoritmos propios.
- Reimplementar TLS.
- Reimplementar un proveedor OIDC.
- Reimplementar un KMS o HSM.
- Guardar claves privadas sensibles en texto plano o en repositorio.
- Crear un sistema de contraseñas como núcleo del ecosistema.

---

## 3. Componentes del ecosistema RT

### RTPack
Protección de objetos transportables o persistibles.  
Aquí RTCrypt aporta seguridad **de objeto**, no solo seguridad de canal.

### RTCore
Seguridad de APIs, middleware e identidad.  
Aquí RTCrypt valida tokens, extrae principals y aplica autorización.

### RTStream
Seguridad de transporte y, más adelante, perfiles de protección para eventos/segmentos.  
En v0 la prioridad es **TLS 1.3 + mTLS interno**.

### RTCloud
Plano de control, configuración, políticas, auditoría y conexión con KMS/IdP/PKI externos.

---

## 4. Principios no negociables

1. **No crypto home-made.**
2. **Fail closed por defecto.**
3. **Allow-list de algoritmos y perfiles.**
4. **Typing obligatorio en objetos y tokens cuando aplique.**
5. **Separación estricta entre autenticación y autorización.**
6. **Separación estricta entre seguridad de transporte y seguridad de objeto.**
7. **Claves privadas fuera del código.**
8. **Compatibilidad con multi-tenant desde el diseño.**
9. **Observabilidad y auditoría sin filtrar secretos.**
10. **Conformance tests desde el inicio.**

---

## 5. Estándares base seleccionados

### Transporte
- TLS 1.3
- mTLS para tráfico interno entre workloads cuando corresponda

### Objetos RT
- CBOR como serialización compacta
- COSE para firmas, MACs y cifrado de objetos
- HPKE para cifrado híbrido a destinatarios

### Identidad y acceso
- OAuth 2.0
- OpenID Connect
- SPIFFE/SPIRE para identidad de servicios

### Derivación y credenciales
- HKDF para derivación de claves
- Argon2id solo si alguna instalación necesita almacenamiento local de credenciales humanas

### Custodia de claves
- AWS KMS, HashiCorp Vault Transit, HSM u otros proveedores equivalentes

---

## 6. Decisiones de arquitectura para v0

La v0 se enfoca en lo que aporta valor real sin sobrediseñar:

- **RTPack profile v1** para manifiestos y payloads protegidos.
- **RTCore auth middleware** para validación OIDC/OAuth.
- **RTCloud key provider abstraction** para integrar KMS.
- **RTStream transport security profile** con TLS 1.3 y mTLS.

Se deja fuera de v0:

- E2E streaming complejo por frame.
- PKI propia.
- Gestión de usuarios final.
- Cripto selectiva sofisticada.
- Soporte productivo para múltiples lenguajes desde el primer día.

---

## 7. Estructura del repositorio

```text
rtcrypt/
├─ README.md
├─ LICENSE
├─ SECURITY.md
├─ CONTRIBUTING.md
├─ CODE_OF_CONDUCT.md
├─ .github/
│  ├─ workflows/
│  └─ ISSUE_TEMPLATE/
├─ docs/
├─ specs/
├─ schemas/
├─ reference/
│  └─ rust/
├─ examples/
├─ test-vectors/
├─ conformance/
└─ scripts/
```

La carpeta `specs/` es el corazón del proyecto.  
La carpeta `reference/` contiene la implementación de referencia.  
La carpeta `schemas/` contiene contratos serializables y validables.  
La carpeta `examples/` ofrece contexto operativo y casos de uso.

---

## 8. Cómo debe trabajar Codex sobre este repositorio

Este repositorio está preparado para que un agente de programación pueda trabajar con bajo nivel de ambigüedad. Para eso:

- Cada archivo describe **qué debe hacer** el componente.
- Se indica explícitamente **qué no debe hacer**.
- Se separan perfiles, esquemas y ejemplos.
- Se definen decisiones arquitectónicas antes de escribir código.
- Se incluyen contratos base para manifiestos, claims y políticas.
- Se indican pasos de implementación por módulo.

### Reglas para Codex

1. No cambiar estándares seleccionados sin ADR explícito.
2. No introducir algoritmos no aprobados por el perfil.
3. No acoplar una implementación concreta de KMS dentro de `rtcrypt-core`.
4. No meter lógica de autorización dentro de parseadores criptográficos.
5. No usar secretos hardcodeados ni claves de ejemplo como si fueran reales.
6. No suponer tablas, servicios o componentes no definidos por especificación.
7. Implementar primero validación segura y luego ergonomía.

---

## 9. Resultado esperado del primer ciclo de desarrollo

El primer ciclo de desarrollo debe producir:

- Un crate `rtcrypt-core` con tipos y errores comunes.
- Un crate `rtcrypt-cose` con firma/verificación de manifiestos.
- Un crate `rtcrypt-auth` con validación de tokens y extracción de principal.
- Un crate `rtcrypt-kms` con trait de proveedor de claves y un mock provider.
- Un crate `rtcrypt-cli` con comandos `sign`, `verify`, `inspect`.
- Test vectors y pruebas negativas.

---

## 10. Estado actual

Este repositorio inicial contiene documentación operativa y técnica suficiente para comenzar la implementación.

No es todavía una release del software. Es una **base fundacional** para desarrollar RTCrypt con dirección clara, compatible con estándares y apta para trabajo asistido por Codex.

---

## 11. Referencias normativas recomendadas

- RFC 8446 - TLS 1.3
- RFC 9052 / RFC 9053 / RFC 9054 - COSE
- RFC 9180 - HPKE
- RFC 5869 - HKDF
- RFC 9106 - Argon2
- OpenID Connect Core 1.0
- OAuth 2.0
- SPIFFE / X509-SVID / Workload API

Ver más detalle en `specs/architecture.md` y `specs/profiles/`.

---

## 12. Licencia

Este proyecto está preparado para publicarse bajo **Apache-2.0** salvo decisión posterior del equipo.

La elección de Apache-2.0 es razonable para un proyecto open source técnico, con buena interoperabilidad y una concesión de patentes útil para ecosistemas de infraestructura.

---

## 13. Próximo paso recomendado

Después de subir este repositorio a GitHub:

1. Crear el repositorio `RTCrypt`.
2. Hacer commit de esta base documental.
3. Abrir issues iniciales por módulo.
4. Pedir a Codex implementar `rtcrypt-core` y `rtcrypt-cose` siguiendo los perfiles ya definidos.
5. No comenzar por integraciones cloud reales hasta tener contratos y pruebas de conformidad.
