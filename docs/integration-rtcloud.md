# Integración con RTCloud

## Objetivo

Definir a RTCloud como plano de control y punto de integración operacional de RTCrypt.

RTCloud no debe convertirse en una caja negra criptográfica. Su papel es orquestar políticas, proveedores de claves, configuración confiable, identidad y auditoría.

---

## 1. Responsabilidades de RTCloud

- Configurar perfiles activos
- Resolver proveedores de claves
- Gestionar metadata de claves públicas
- Integrar con KMS/HSM
- Registrar eventos de uso criptográfico
- Mantener configuración de emisores confiables
- Exponer políticas multi-tenant

---

## 2. Qué NO debe hacer RTCloud

- No debe almacenar claves privadas exportables por defecto.
- No debe reimplementar un KMS.
- No debe convertirse en proveedor OIDC completo.
- No debe autorizar “por atajo” fuera de las políticas definidas.
- No debe mezclar secretos operativos con código.

---

## 3. Integraciones externas previstas

### KMS/HSM
- AWS KMS
- Vault Transit
- HSM empresariales

### Identidad
- OIDC providers
- SPIFFE/SPIRE

### Observabilidad
- logs estructurados
- métricas
- trazas
- auditoría de eventos sensibles

---

## 4. Operaciones mínimas esperadas

### Sobre claves
- resolve key metadata
- sign
- wrap key
- unwrap key
- publish public material
- rotate reference

### Sobre trust
- registrar issuer confiable
- registrar audience esperadas
- registrar trust domain de workloads

### Sobre política
- publicar políticas
- versionarlas
- auditar cambios

---

## 5. Modelo lógico recomendado

```text
KeyRegistry
- key_id
- provider
- purpose
- tenant_scope
- status
- algorithm_set
- created_at
- rotation_state

IssuerRegistry
- issuer
- jwks_uri / discovery source
- accepted_algs
- audiences[]
- status

PolicyRegistry
- policy_id
- version
- scope
- effect
- rules[]
```

---

## 6. Auditoría mínima

Todo evento sensible debería registrar:

- timestamp
- actor principal o workload
- tenant
- action
- resource
- key_id o issuer si aplica
- outcome
- correlation_id

No registrar:
- payloads descifrados
- tokens completos
- secretos
- material de clave

---

## 7. Multi-tenant

RTCloud debe estar diseñado para evitar confusiones entre tenants:

- scopes de clave explícitos;
- policies con tenant scope;
- resolución de issuer por contexto;
- separación clara de recursos auditados.

---

## 8. Resultado esperado para Codex

Codex debe implementar RTCloud de forma que:

- el core pueda llamar providers sin conocer detalles cloud;
- exista un contrato estable de configuración;
- la auditoría sea estructurada;
- el sistema soporte cambios de proveedor sin reescribir todo el proyecto.
