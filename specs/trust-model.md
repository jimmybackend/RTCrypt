# Trust model de RTCrypt

## Objetivo

Definir qué entidades confían en cuáles, bajo qué condiciones, y qué artefactos sostienen esa confianza.

La mayoría de errores graves en sistemas de identidad y seguridad no nacen de la criptografía, sino de un trust model implícito o ambiguo. RTCrypt necesita hacerlo explícito.

---

## 1. Entidades de confianza

- Usuario final
- Cliente / aplicación
- Workload / servicio interno
- Issuer OIDC
- Key Provider / KMS
- Verifier / consumer
- RTCloud como plano de control
- Tenant / boundary organizacional

---

## 2. Tipos de confianza

### 2.1 Confianza criptográfica
Existe cuando una firma, certificado o estructura protegida puede verificarse con material confiable.

### 2.2 Confianza de configuración
Existe cuando una entidad usa una lista aprobada de issuers, audiences, trust domains o key scopes.

### 2.3 Confianza operacional
Existe cuando procesos de rotación, auditoría y despliegue se cumplen.

### 2.4 Confianza contextual
Existe cuando una identidad es válida en un contexto específico: tenant, recurso, acción o entorno.

---

## 3. Raíces de confianza

### Para transporte
- CA o trust bundle configurado

### Para OIDC
- issuer configurado como confiable
- metadata y JWKS obtenidos de fuente aprobada

### Para workloads
- trust domain SPIFFE aprobado
- bundle de confianza correspondiente

### Para KMS
- credenciales y endpoint del proveedor aprobados

---

## 4. Reglas fundamentales

1. Ninguna firma vale sin contexto.
2. Ningún token vale para todas las audiencias.
3. Ninguna identidad vale automáticamente para cualquier tenant.
4. Ningún `kid` debe resolverse sin scope.
5. Ningún peer interno es “confiable” solo por estar en la red interna.
6. La confianza debe ser renovable y revocable.

---

## 5. Relaciones principales

### Consumidor de RTPack
Confía en:
- profile soportado;
- clave pública correcta;
- manifest bien formado;
- contexto autorizado.

### RTCore
Confía en:
- issuer configurado;
- audience esperada;
- middleware de verificación;
- authorizer local.

### RTStream interno
Confía en:
- identidad de workload verificada;
- canal TLS;
- política de stream.

### RTCloud
Confía en:
- configuración administrativa válida;
- KMS integrado correctamente;
- metadata de issuer y bundles vigentes.

---

## 6. Revocación y rotación

El trust model asume que la confianza puede cambiar.

Por eso deben existir mecanismos o procedimientos para:

- rotar claves;
- deshabilitar issuers;
- revocar workloads;
- retirar perfiles inseguros;
- invalidar políticas antiguas.

---

## 7. Límites de confianza multi-tenant

Cada tenant debe poder aislar:

- recursos;
- claves;
- policies;
- audiences;
- audit trails.

Una de las reglas más importantes del proyecto es **no resolver confianza cruzada entre tenants por inferencia**.

---

## 8. Riesgos de trust model a evitar

- confiar en cualquier issuer público;
- usar la misma clave para demasiados propósitos;
- mezclar identidad humana y de workload;
- depender de headers no autenticados;
- asumir que un entorno interno es suficiente prueba de identidad.

---

## 9. Resultado esperado para implementación

La implementación debe reflejar este trust model con:

- registries explícitos;
- configuraciones versionables;
- validación de scope;
- interfaces separadas por tipo de confianza.
