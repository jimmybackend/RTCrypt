# Threat model de RTCrypt

## Objetivo

Describir amenazas relevantes para el diseño de RTCrypt y orientar decisiones de perfil, validación y operación.

Este documento no pretende modelar todas las amenazas posibles del universo RT, sino las que impactan directamente la arquitectura de seguridad del proyecto.

---

## 1. Activos a proteger

- manifiestos RTPack
- payloads protegidos
- integridad de metadatos críticos
- autenticidad de tokens
- identidad de workloads
- políticas de autorización
- referencias y metadata de claves
- eventos de auditoría
- configuración confiable del sistema

---

## 2. Adversarios considerados

### A1. Atacante de red
Puede observar, interceptar, modificar o reenviar tráfico.

### A2. Servicio intermedio no confiable
Tiene acceso a objetos en tránsito o almacenamiento, pero no debería poder alterar contenido sin detección.

### A3. Cliente autenticado pero sin privilegios suficientes
Busca ampliar acceso mediante abuso de claims, audiences o scopes.

### A4. Workload interno comprometido
Posee cierta identidad o conectividad, pero intenta actuar fuera de su alcance.

### A5. Operador o integrador con configuración defectuosa
No es un atacante malicioso, pero puede causar exposición por mala configuración.

### A6. Atacante con acceso parcial a logs o artefactos
Busca tokens, payloads o metadata sensible expuesta accidentalmente.

---

## 3. Amenazas principales

### T1. Eavesdropping
Lectura no autorizada del tráfico.
Mitigación:
- TLS 1.3
- cifrado de objeto cuando aplique

### T2. Tampering
Modificación de datos en tránsito o almacenamiento.
Mitigación:
- firmas COSE
- AEAD
- digests protegidos

### T3. Replay
Reenvío de objetos o mensajes válidos fuera de contexto.
Mitigación:
- expiración
- audience
- context binding
- controles de aplicación según caso

### T4. Confusion attacks
Usar un token o envelope válido en un contexto distinto al esperado.
Mitigación:
- typing
- profiles explícitos
- audience/resource validation

### T5. Algorithm downgrade
Forzar o aceptar algoritmos no permitidos.
Mitigación:
- allow-list estricta
- validación de profile

### T6. Key confusion
Resolver un `kid` a una clave equivocada o de otro tenant.
Mitigación:
- key scope
- metadata de propósito
- tenant context

### T7. Identity confusion
Tratar un workload como usuario o viceversa.
Mitigación:
- principal kind explícito
- rutas de autenticación separadas

### T8. Authorization bypass
Ejecutar acciones sin revisar política o con claims insuficientes.
Mitigación:
- default deny
- authorizer separado
- tests de policy

### T9. Sensitive data exposure
Exponer tokens, payloads o claves en logs.
Mitigación:
- logging redactado
- eventos estructurados
- prohibición de dumps inseguros

---

## 4. Suposiciones del modelo

- Los estándares criptográficos seleccionados se consideran sólidos cuando se usan correctamente.
- Los proveedores externos de identidad y claves son confiables dentro del contexto operativo.
- El host puede ser comprometido; RTCrypt no puede eliminar ese riesgo por completo.
- La seguridad depende tanto de implementación como de configuración.

---

## 5. Áreas de alto riesgo en implementación

- parseo de envelopes;
- validación parcial de claims;
- resolución de claves;
- no separación entre autenticación y autorización;
- manejo de errores con exceso de detalle;
- fixtures inseguros reutilizados como si fueran producción.

---

## 6. Controles requeridos

- allow-list de algoritmos;
- typing obligatorio;
- verificación temporal;
- context binding;
- segregación multi-tenant;
- identidad fuerte de workload;
- pruebas negativas;
- auditoría.

---

## 7. Controles diferidos a operación

- hardening de host;
- control de secretos del pipeline;
- rotación operativa de credenciales;
- monitoreo e incident response;
- segmentación de red.

---

## 8. Casos explícitamente fuera de alcance técnico directo

- malware en el endpoint del usuario;
- exfiltración completa del host comprometido;
- compromiso del proveedor cloud;
- controles físicos sobre hardware ajeno.

Estos escenarios no desaparecen, pero el diseño del proyecto debe minimizar impacto y favorecer contención.

---

## 9. Resultado esperado

El threat model debe servir para:

- justificar perfiles;
- guiar revisión de PRs;
- construir conformance tests;
- decidir prioridades del roadmap.
