# Política de seguridad de RTCrypt

## Objetivo

Este documento define cómo se reportan vulnerabilidades y cuáles son las reglas de seguridad del proyecto.

RTCrypt es un proyecto orientado a seguridad. Por eso el estándar de revisión debe ser mayor que en una librería utilitaria general.

---

## 1. Alcance de esta política

Esta política aplica a:

- Código fuente de RTCrypt
- Especificaciones y perfiles del repositorio
- CLI, ejemplos y artefactos de prueba
- Integraciones oficiales mantenidas por el proyecto

No aplica automáticamente a:

- Forks no oficiales
- Deployments de terceros
- Configuraciones inseguras hechas fuera del proyecto

---

## 2. Cómo reportar una vulnerabilidad

No abras una issue pública para una vulnerabilidad explotable.

Usa un canal privado definido por el repositorio, por ejemplo:

- GitHub Security Advisories
- security@retha.example (reemplazar con dirección real del proyecto)

Incluye:

- Descripción clara
- Componente afectado
- Impacto esperado
- Suposiciones del atacante
- Prueba de concepto mínima, si existe
- Versión o commit afectado
- Recomendación preliminar, si la tienes

---

## 3. Qué se considera vulnerabilidad

Ejemplos no exhaustivos:

- Aceptar algoritmos fuera de la allow-list
- Bypass de validación de firma
- Confusión de tipos entre envelopes o tokens
- Uso inseguro de claves o nonces
- Omisión de validaciones de issuer, audience o expiration
- Fuga de datos sensibles en logs
- Deserialización ambigua de objetos RT
- Aceptación de certificados o bundles no confiables
- Errores que permitan replay, downgrade o confusion attacks
- Integraciones KMS que filtren material sensible o permitan uso indebido

---

## 4. Principios de respuesta

1. **Privacidad del reporte**
   - El equipo debe mantener el caso privado hasta entender el impacto.

2. **Confirmación rápida**
   - Se debe confirmar recepción del reporte y clasificar severidad inicial.

3. **Corrección con pruebas**
   - No se debe cerrar una corrección sin tests positivos y negativos.

4. **Divulgación responsable**
   - Publicar advisory una vez exista fix o mitigación clara.

5. **Backports cuando corresponda**
   - Si hay varias ramas mantenidas, evaluar backport.

---

## 5. Reglas de desarrollo seguro

### 5.1 No inventar criptografía
Ningún PR puede introducir algoritmos, esquemas o construcciones no estandarizadas como base del sistema.

### 5.2 Validación estricta
Todo parser o verificador debe fallar ante entradas ambiguas, incompletas o fuera de perfil.

### 5.3 Fail closed
Si una validación depende de configuración crítica y ésta no está disponible, la operación debe rechazar.

### 5.4 Claves privadas fuera del repositorio
No se aceptan commits con claves privadas reales, secretos productivos ni material sensible exportable.

### 5.5 Logs sin secretos
No registrar plaintext sensible, tokens completos, claves, payloads descifrados ni trazas que comprometan material protegido.

### 5.6 Seguridad por capas
No asumir que TLS reemplaza firma de objeto. No asumir que una firma de objeto reemplaza autorización.

---

## 6. Superficies críticas a revisar siempre

- Parsing y canonicalización
- Selección de algoritmos
- Validación de headers protegidos
- Nonces, IVs y AEAD
- Key IDs (`kid`) y resolución de claves
- Relación `iss` / `aud` / `sub` / `scope`
- mTLS y validación de identidad de workload
- Integración con proveedores KMS
- Rotación y expiración de claves
- Manejo de errores y mensajes observables

---

## 7. Requisitos para releases

Antes de etiquetar una release:

- Tests unitarios y de integración en verde
- Test vectors actualizados
- Revisión de algoritmos permitidos
- Revisión de dependencias criptográficas
- Revisión de documentación de seguridad
- Changelog con fixes de seguridad si aplica

---

## 8. Dependencias

Las dependencias criptográficas deben cumplir:

- Mantenimiento activo
- Historial razonable de seguridad
- API estable o entendible
- Licencia compatible
- Uso explícito y revisable

No se debe aceptar una dependencia solo porque “funciona”.  
Debe entenderse qué parte del sistema de seguridad resuelve y con qué garantías.

---

## 9. Amenazas fuera de alcance inmediato

RTCrypt no puede eliminar por sí solo:

- Compromiso del host
- Robo de credenciales en un IdP externo
- Errores de configuración operativa
- Filtración de secretos por pipelines ajenos
- Fallos del proveedor cloud o KMS

Sin embargo, el diseño del proyecto debe minimizar el impacto de estos escenarios.

---

## 10. Compatibilidad con disclosure pública

Cuando una vulnerabilidad sea pública, el advisory debe indicar:

- Componente afectado
- Versiones afectadas
- Tipo de impacto
- Condiciones para explotar
- Fix o mitigación
- Tests añadidos
- Acciones recomendadas para operadores

---

## 11. Contacto operativo

Sustituir antes de publicación pública:

- Security contact
- PGP key o mecanismo equivalente
- SLA interno de respuesta
- Política de severidad del proyecto
