# Perfil `rtpack-cose-v1`

## Estado
Draft inicial para implementación de referencia.

## Objetivo
Definir cómo proteger objetos RTPack mediante COSE y CBOR.

---

## 1. Alcance

Este perfil aplica a:

- manifiestos RTPack firmados;
- payloads cifrados o referenciados;
- validación básica de metadata protegida.

No define:
- transporte de red;
- autorización de negocio;
- formato universal de todo RT Stack.

---

## 2. Formato base

- serialización: **CBOR**
- firma principal: **COSE_Sign1**
- cifrado opcional: COSE según variante final
- derivación / recipients: según COSE + HPKE cuando aplique

---

## 3. Metadata protegida mínima

El conjunto mínimo protegido debe incluir:

- `typ`
- `profile`
- `ver`
- `iss`
- `sub` si aplica
- `iat`
- `kid`
- `tenant`
- `payload_digest` o referencia equivalente si existe payload
- `cty` si el tipo de contenido es relevante

### Reglas
- `typ` debe ser inequívoco
- `profile` debe ser exactamente `rtpack-cose-v1`
- `kid` es obligatorio
- `iat` debe estar en formato normalizado según implementación

---

## 4. Algoritmos permitidos en v1

La lista final debe implementarse como allow-list.  
Recomendación inicial:

### Firma
- EdDSA / Ed25519 cuando la infraestructura lo soporte
- soporte adicional según necesidad operativa documentada

### Cifrado de contenido
- AEAD estándar aprobada por la librería COSE seleccionada

### KDF / recipients
- HKDF cuando el algoritmo o profile lo requiera

### HPKE
- solo mediante suites soportadas y documentadas por la implementación

> Este perfil no debe aceptar algoritmos fuera del set configurado.

---

## 5. Validación

Para verificar un envelope de este perfil se debe:

1. parsear estructura COSE;
2. validar headers protegidos;
3. validar `typ`;
4. validar `profile`;
5. resolver `kid` dentro del tenant/contexto correcto;
6. verificar firma;
7. decodificar manifest;
8. validar schema semántico del manifest;
9. validar tiempos si existen;
10. validar payload digest si aplica.

---

## 6. Errores esperados

- envelope_malformed
- profile_mismatch
- missing_protected_header
- invalid_typ
- invalid_signature
- key_not_found
- key_scope_mismatch
- invalid_manifest
- payload_digest_mismatch
- expired_object

---

## 7. Consideraciones de diseño

- El manifest es la pieza principal a autenticar.
- El payload puede ser inline o externo.
- El sistema no debe confiar en campos no protegidos para routing crítico.
- El perfil debe ser fácil de inspeccionar por CLI.

---

## 8. Requisitos para implementación de referencia

- parser determinista;
- inspector legible;
- tests positivos y negativos;
- fixtures reproducibles;
- separación entre parsing, verification y schema validation.

---

## 9. Resultado esperado

Una implementación conforme a este perfil puede:

- firmar un manifiesto RTPack;
- verificarlo con `kid`;
- inspeccionar headers y claims relevantes;
- rechazar envelopes ambiguos o fuera de perfil.
