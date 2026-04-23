# Integración con RTPack

## Objetivo

Definir cómo RTCrypt protege objetos de tipo RTPack.

RTPack representa unidades transportables o persistibles del ecosistema RT. Por tanto, su protección no debe depender únicamente del canal. Un pack debe poder mantenerse íntegro y, cuando corresponda, confidencial aunque pase por colas, disco, caches, backups o intermediarios.

---

## 1. Decisión principal

Para RTPack se usará **seguridad de objeto** con:

- **CBOR** como serialización
- **COSE** como estructura de firma/cifrado
- **HPKE** cuando se requiera cifrado híbrido por destinatario

Esto evita acoplar la seguridad a HTTP, gRPC o a cualquier transporte concreto.

---

## 2. Modelo de objeto

### Entidades principales

- `Manifest`
- `Payload`
- `Recipients`
- `Protected Metadata`
- `Envelope`

### Recomendación v0

Firmar siempre el **manifest** y decidir según caso si:
- el payload va incluido,
- el payload va referenciado,
- el payload va cifrado,
- el payload queda fuera pero el digest sí va en el manifest.

---

## 3. Qué debe contener el manifest

Campos mínimos conceptuales:

- `typ`
- `profile`
- `version`
- `issuer`
- `subject`
- `issued_at`
- `expires_at` opcional
- `content_type`
- `payload_digest`
- `payload_length`
- `labels` opcionales
- `tenant_id`
- `resource_id`

### Principio

No todos los campos son públicos ni todos deben ser protegidos del mismo modo.  
Pero el conjunto mínimo que determina la interpretación del objeto debe quedar **autenticado**.

---

## 4. Envelope recomendado

### Firma
Usar `COSE_Sign1` para manifiestos firmados por un emisor único.

### Cifrado
Usar `COSE_Encrypt` / `COSE_Encrypt0` según diseño final del perfil.  
Si hay múltiples destinatarios, usar un modelo explícito de recipients y key distribution.

### Cifrado híbrido
Si se cifra para uno o varios destinatarios y no existe clave compartida previa, usar **HPKE** para envolver la CEK.

---

## 5. Validaciones obligatorias

Al verificar un RTPack protegido:

1. Validar formato y typing.
2. Validar profile.
3. Validar headers protegidos obligatorios.
4. Resolver `kid`.
5. Verificar firma o descifrado.
6. Validar claims del manifest.
7. Validar digest del payload si existe payload externo o adjunto.
8. Validar vigencia temporal si aplica.
9. Validar tenant/contexto de autorización.

---

## 6. Qué no debe hacer RTPack

- No debe inventar un formato binario criptográfico propio.
- No debe asumir que “estar firmado” implica que puede procesarse sin autorización.
- No debe confiar en metadatos no autenticados para routing crítico.
- No debe aceptar múltiples interpretaciones del mismo envelope.

---

## 7. Casos de uso esperados

### Caso A: Manifiesto firmado
Para integridad, procedencia y no repudio operativo básico.

### Caso B: Manifiesto firmado + payload cifrado
Para intercambio entre componentes autorizados.

### Caso C: Manifiesto firmado + payload referenciado
Cuando el contenido vive en otro storage y el manifest solo fija integridad.

---

## 8. Interfaz deseada en la implementación de referencia

```rust
pub trait ObjectProtector {
    fn sign_manifest(&self, manifest: &RtManifest, signer: &SigningContext) -> Result<ProtectedEnvelope, ProtectError>;
    fn verify_manifest(&self, envelope: &[u8], ctx: &VerificationContext) -> Result<VerifiedManifest, VerifyError>;
    fn encrypt_payload(&self, payload: &[u8], recipients: &[RecipientRef], ctx: &EncryptionContext) -> Result<ProtectedEnvelope, ProtectError>;
    fn decrypt_payload(&self, envelope: &[u8], recipient: &RecipientContext) -> Result<DecryptedPayload, DecryptError>;
}
```

---

## 9. Riesgos a vigilar

- Reutilización de nonce/IV
- Falta de binding entre manifest y payload
- Resolver `kid` a una clave equivocada
- Confusión entre perfiles o versiones
- Normalización inconsistente del digest
- Aceptar claims fuera del set permitido

---

## 10. Resultado esperado para Codex

Codex debe implementar RTPack de manera que:

- el manifest tenga un tipo estable y serializable;
- la firma/verificación no dependan del transporte;
- el envelope sea parseable e inspeccionable;
- las validaciones estén separadas por etapas;
- existan tests positivos y negativos.
