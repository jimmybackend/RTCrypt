# Integración con RTCore

## Objetivo

Definir cómo RTCore usa RTCrypt para identidad, autenticación y autorización de APIs y servicios internos.

RTCore es el lugar donde la seguridad entra al flujo de negocio. Por eso aquí la prioridad es validar correctamente el principal y aplicar políticas antes de ejecutar acciones.

---

## 1. Responsabilidades de RTCrypt dentro de RTCore

- Verificar tokens
- Construir `Principal`
- Validar contexto de request
- Aplicar autorización base
- Resolver identidad de workloads cuando aplique
- Dejar trazas auditables sin exponer secretos

---

## 2. Modelo recomendado

### Entrada de identidad
- Bearer tokens OAuth 2.0 / OIDC
- mTLS client certificates
- SPIFFE SVIDs para workloads internos

### Salida hacia la aplicación
Un objeto de dominio mínimo:

```text
Principal
- kind
- issuer
- subject
- tenant_id
- audiences[]
- scopes[]
- claims{}
- auth_method
```

---

## 3. Flujo HTTP sugerido

1. Recibir request.
2. Extraer método de autenticación.
3. Verificar token o peer identity.
4. Construir `Principal`.
5. Asociar `resource` y `action`.
6. Evaluar política.
7. Si autorizado, pasar principal al handler.
8. Registrar evento auditado.

---

## 4. Reglas de validación de token

RTCore no debe aceptar un token solo porque está bien firmado.

Debe validar también:

- `iss`
- `aud`
- `exp`
- `nbf` si existe
- `iat` si aplica
- `alg`
- `kid`
- typing cuando el formato lo permita
- scopes requeridos
- relación del token con el recurso

---

## 5. Autorización base

En v0 basta un modelo simple, determinista y auditable.

### Entradas
- principal
- action
- resource
- context

### Salida
- allow / deny
- reason code
- policy id opcional

### Regla fundacional
**default deny**

---

## 6. Qué no debe hacer RTCore

- No mezclar parseo del token con lógica de negocio.
- No confiar en claims no verificados.
- No asumir que un access token sirve para cualquier audience.
- No permitir bypass por headers internos no autenticados.
- No usar scopes como sustituto total de autorización contextual.

---

## 7. Integración recomendada en middleware

### Middleware de autenticación
Responsable de:
- extraer credenciales;
- validar;
- construir principal;
- inyectar contexto.

### Middleware de autorización
Responsable de:
- mapear recurso/acción;
- evaluar política;
- devolver `403` cuando corresponda.

Esto facilita pruebas y evita handlers contaminados con detalles de seguridad.

---

## 8. Ejemplo de mapeo recurso/acción

```text
POST /rtpacks              -> action=create   resource=rtpack
GET  /rtpacks/{id}         -> action=read     resource=rtpack:{id}
POST /streams/{id}/publish -> action=publish  resource=stream:{id}
GET  /admin/keys           -> action=list     resource=key-management
```

---

## 9. Errores recomendados

- `invalid_token`
- `invalid_issuer`
- `invalid_audience`
- `token_expired`
- `token_not_yet_valid`
- `insufficient_scope`
- `unauthorized_workload`
- `policy_denied`

---

## 10. Resultado esperado para Codex

Codex debe implementar RTCore de forma que:

- exista un verificador desacoplado del framework HTTP;
- el `Principal` sea estable y portable;
- la autorización sea una capa separada;
- los errores sean distinguibles;
- el middleware sea fácil de integrar en más de un runtime.
