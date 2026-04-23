# Quickstart conceptual de RTCrypt

> Este quickstart no asume que el software ya esté completo. Sirve para explicar el flujo esperado de uso e implementación.

## 1. Caso mínimo: firmar un manifiesto RTPack

Objetivo:
- Crear un manifiesto RT.
- Firmarlo con un perfil COSE.
- Verificarlo con una clave pública resuelta por `kid`.

### Flujo esperado

1. Construir un manifiesto válido según `schemas/manifest.cddl`.
2. Añadir metadatos protegidos mínimos:
   - `typ`
   - `profile`
   - `issuer`
   - `issued_at`
   - `kid`
3. Serializar a CBOR.
4. Firmar usando `COSE_Sign1`.
5. Exportar envelope.
6. Verificar envelope en consumidor.

### Comando CLI esperado

```bash
rtcrypt sign \
  --profile rtpack-cose-v1 \
  --input ./examples/rtpack/manifest.sample.json \
  --key-ref kms:alias/rtcrypt-signing \
  --output ./out/manifest.rtse.cbor
```

### Verificación

```bash
rtcrypt verify \
  --profile rtpack-cose-v1 \
  --input ./out/manifest.rtse.cbor
```

---

## 2. Caso mínimo: validar un access token en RTCore

Objetivo:
- Recibir un bearer token.
- Validar firma, issuer, audience y expiración.
- Construir un `Principal`.
- Evaluar autorización.

### Flujo esperado

1. Extraer token del request.
2. Resolver metadata del issuer confiable.
3. Verificar `alg`, `kid`, firma y claims.
4. Validar:
   - `iss`
   - `aud`
   - `exp`
   - `nbf` si existe
   - scopes requeridos
5. Construir `Principal`.
6. Pasar el principal a `Authorizer`.

### Resultado esperado

- Si la validación falla: `401 Unauthorized`
- Si la identidad es válida pero no tiene permisos: `403 Forbidden`

---

## 3. Caso mínimo: mTLS entre workloads en RTStream

Objetivo:
- Permitir canal interno seguro entre productores y consumidores.

### Reglas mínimas

- TLS 1.3 obligatorio
- Certificados válidos y no expirados
- Identidad del peer validada
- Si se usa SPIFFE, el `spiffe_id` debe mapear a una política autorizada

### No hacer en v0

- No diseñar cifrado por frame propio
- No mezclar el canal TLS con la semántica de autorización de negocio

---

## 4. Caso mínimo: provider KMS en RTCloud

Objetivo:
- Firmar o descifrar sin exponer claves privadas al core.

### Trait conceptual

```rust
pub trait KeyProvider {
    fn sign(&self, key_ref: &KeyRef, payload: &[u8], profile: &SecurityProfile) -> Result<Vec<u8>, KeyProviderError>;
    fn verify_public_key(&self, key_ref: &KeyRef) -> Result<ResolvedPublicKey, KeyProviderError>;
    fn wrap_key(&self, key_ref: &KeyRef, cek: &[u8], profile: &SecurityProfile) -> Result<Vec<u8>, KeyProviderError>;
    fn unwrap_key(&self, key_ref: &KeyRef, wrapped: &[u8], profile: &SecurityProfile) -> Result<Vec<u8>, KeyProviderError>;
}
```

---

## 5. Orden recomendado de implementación

### Paso 1
Implementar `rtcrypt-core`:
- tipos,
- errores,
- profiles,
- parsing de configuración.

### Paso 2
Implementar `rtcrypt-cose`:
- sign,
- verify,
- inspect,
- headers protegidos obligatorios.

### Paso 3
Implementar `rtcrypt-auth`:
- token verifier,
- principal extractor,
- authorizer básico.

### Paso 4
Implementar `rtcrypt-kms`:
- trait,
- mock provider,
- adapter inicial.

### Paso 5
Implementar `rtcrypt-cli`:
- `sign`
- `verify`
- `inspect`

---

## 6. Criterios para dar por bueno un primer prototipo

- Puede firmar y verificar un manifiesto
- Rechaza inputs fuera de perfil
- Puede validar un token OIDC básico
- Puede construir un principal coherente
- No usa claves privadas hardcodeadas
- Tiene test vectors reproducibles

---

## 7. Qué no debe hacer un quickstart engañoso

No se deben mostrar ejemplos que:

- omitan validaciones críticas;
- acepten cualquier algoritmo;
- acepten clocks sin límite;
- usen claves en texto plano como práctica normal;
- mezclen payload de demo con flujo productivo sin indicarlo.

---

## 8. Próximo ejercicio sugerido para Codex

Después del quickstart, pedir a Codex:

1. Crear workspace Rust.
2. Implementar `rtcrypt-core`.
3. Añadir tipos `Principal`, `KeyRef`, `SecurityProfile`, `EnvelopeType`.
4. Crear errores tipados.
5. Implementar un lector de profiles desde archivo.
6. Escribir tests.
