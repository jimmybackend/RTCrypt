# Arquitectura de RTCrypt

## 1. Visión de arquitectura

RTCrypt es una arquitectura de seguridad en capas para RT Stack. No es un único crate, binario o servicio. Es una combinación de:

- perfiles normativos;
- librerías de referencia;
- integraciones con identidad y claves;
- esquemas y contratos serializables;
- pruebas de conformidad.

El propósito de esta arquitectura es ofrecer una base consistente para seguridad transversal en RT Stack sin diseñar criptografía propietaria.

---

## 2. Objetivos de arquitectura

1. Proteger datos en tránsito y, cuando aplique, como objetos persistibles.
2. Soportar identidad de usuarios, clientes y workloads.
3. Separar autenticación, integridad, cifrado y autorización.
4. Hacer interoperable la seguridad entre módulos.
5. Mantener bajo acoplamiento con proveedores externos.
6. Favorecer validación estricta y pruebas negativas.

---

## 3. Capas del sistema

### 3.1 Capa de perfiles
Define reglas operativas y restricciones:
- formatos permitidos;
- algoritmos permitidos;
- claims obligatorios;
- validaciones obligatorias;
- defaults y límites.

### 3.2 Capa core
Contiene tipos, errores, configuración y abstracciones comunes.

### 3.3 Capa criptográfica
Implementa firma, verificación, cifrado y descifrado usando estándares aprobados.

### 3.4 Capa de identidad
Valida tokens OIDC/OAuth, mTLS y SPIFFE.

### 3.5 Capa de autorización
Evalúa acciones sobre recursos en contexto.

### 3.6 Capa de integración
Une el sistema con RTPack, RTCore, RTStream y RTCloud.

---

## 4. Componentes lógicos

```text
+-------------------+
|   RT Applications |
+---------+---------+
          |
          v
+-------------------+
|  RTCore / RT APIs |
+---------+---------+
          |
          v
+-------------------+      +------------------+
|   rtcrypt-auth    |<---->|  IdP / OIDC      |
+---------+---------+      +------------------+
          |
          v
+-------------------+      +------------------+
|   rtcrypt-policy  |      |  Policy Source   |
+---------+---------+      +------------------+
          |
          v
+-------------------+      +------------------+
|   rtcrypt-cose    |<---->|  Key Provider    |
+---------+---------+      +------------------+
          |
          v
+-------------------+
|   rtcrypt-core    |
+-------------------+
```

---

## 5. Límites claros de cada módulo

### rtcrypt-core
- tipos comunes
- perfiles
- errores
- configuración
- traits base

### rtcrypt-cose
- envelopes
- firma/verificación
- cifrado/descifrado
- inspección

### rtcrypt-auth
- validación de token
- extracción de principal
- validación de workload identity

### rtcrypt-policy
- decisión allow/deny
- razón de decisión
- evaluación contextual

### rtcrypt-kms
- adaptadores a proveedores
- resolución de claves
- metadata de claves

### rtcrypt-cli
- comandos operativos y de prueba

---

## 6. Decisiones arquitectónicas clave

### 6.1 Objeto vs transporte
La arquitectura diferencia claramente:

- seguridad de canal: TLS/mTLS;
- seguridad de objeto: COSE/HPKE.

### 6.2 Usuario vs workload
No se trata del mismo tipo de identidad.  
Ambas deben entrar al sistema como `Principal`, pero con distinto `kind`.

### 6.3 Core sin cloud lock-in
El core no puede depender de AWS KMS, Vault o un vendor específico.

### 6.4 Specs primero
La implementación debe seguir las specs del repositorio, no al revés.

### 6.5 Fail closed
Si un componente no puede verificar dentro de perfil, rechaza.

---

## 7. Requisitos de implementación

- dependencias criptográficas revisables;
- errores tipados;
- trazabilidad de decisiones;
- serialización estable;
- tests reproducibles;
- compatibilidad con conformance suite.

---

## 8. Formatos

### Para RTPack
- manifest: CBOR
- protección: COSE

### Para identidad de API
- access tokens OIDC/OAuth compatibles
- internal workload identity via mTLS/SPIFFE

### Para config interna
- YAML, TOML o JSON según módulo, con esquema definido

---

## 9. Versionado

El proyecto debe versionar por separado:

- la implementación
- los perfiles
- los schemas

Ejemplo:
- implementación: `0.1.0`
- profile id: `rtpack-cose-v1`
- schema version: `1`

---

## 10. Reglas de compatibilidad

Un cambio rompe compatibilidad si altera:

- campos obligatorios del manifest;
- headers protegidos obligatorios;
- semántica de claims RT;
- contractos públicos de traits;
- decisiones normativas de un profile.

En esos casos debe existir ADR y changelog.

---

## 11. Puntos donde Codex debe empezar

1. `rtcrypt-core`
2. `schemas/`
3. `specs/profiles/rtpack-cose-v1.md`
4. `rtcrypt-cose`
5. `test-vectors/`

Ese orden reduce el riesgo de construir código sin contrato.
