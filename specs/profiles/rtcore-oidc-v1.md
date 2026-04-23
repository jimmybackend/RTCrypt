# Perfil `rtcore-oidc-v1`

## Estado
Draft inicial para middleware de identidad y acceso en RTCore.

## Objetivo
Definir validación mínima de tokens OIDC/OAuth para convertirlos en un `Principal` usable por RTCore.

---

## 1. Alcance

Este perfil aplica a:

- access tokens recibidos por RTCore;
- validación de issuer, audience y tiempos;
- extracción de principal;
- autorización básica posterior.

No aplica a:
- login UX;
- gestión de usuarios;
- federación compleja fuera de la configuración explícita.

---

## 2. Entradas permitidas

- bearer token de issuer configurado
- opcionalmente client certificate o SPIFFE identity en rutas internas

---

## 3. Validaciones obligatorias del token

- firma válida
- `iss` permitido
- `aud` esperado
- `exp` vigente
- `nbf` si existe
- `alg` permitido
- `kid` resoluble
- claims requeridos presentes

### Claims RT recomendados
- `tenant_id`
- `scope`
- `client_id` si aplica
- `sub`
- `azp` si el flujo lo requiere

---

## 4. Construcción de principal

El resultado de la autenticación debe ser un tipo interno estable:

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

### Regla
Un principal válido no implica autorización automática.

---

## 5. Reglas de autorización v1

El authorizer debe recibir:
- principal
- action
- resource
- context

Y devolver:
- allow / deny
- reason code

### Default
deny

---

## 6. Riesgos que este profile debe evitar

- aceptar tokens para otra audience;
- aceptar tokens expirados por exceso de skew;
- aceptar claims sin verificar firma;
- tratar client y user como equivalente;
- confiar en headers internos en lugar del token.

---

## 7. Requisitos de implementación

- verificador desacoplado del framework;
- configuración explícita de issuers;
- caché segura de metadata si se usa;
- errores tipados;
- logs redacted.

---

## 8. Resultado esperado

Una implementación conforme a este perfil puede:

- autenticar requests de RTCore;
- construir un principal consistente;
- denegar acceso cuando issuer, audience o policy no coinciden.
