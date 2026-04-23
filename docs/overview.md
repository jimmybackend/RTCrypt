# Overview de RTCrypt

## Qué es

RTCrypt es una capa de seguridad transversal para **RT Stack**. Su función no es reemplazar estándares existentes, sino ensamblarlos de forma consistente para que los componentes del ecosistema compartan una misma política de seguridad.

RTCrypt debe servir como referencia para:

- diseño de objetos protegidos;
- validación de identidad y acceso;
- uso de claves y proveedores externos;
- implementación de perfiles seguros por componente;
- pruebas de conformidad.

---

## Qué problema evita

Sin una capa como RTCrypt, un ecosistema termina con:

- formatos de seguridad incompatibles entre servicios;
- distintas interpretaciones de claims y políticas;
- dependencia excesiva del canal TLS;
- duplicación de validadores y adaptadores;
- decisiones de seguridad repartidas por el código.

RTCrypt busca convertir la seguridad en una **capacidad de plataforma**, no en lógica dispersa.

---

## Capas de responsabilidad

### 1. Seguridad de transporte
Responsabilidad primaria: proteger conexiones activas.

Tecnología recomendada:
- TLS 1.3
- mTLS interno cuando el caso lo requiera

### 2. Seguridad de objeto
Responsabilidad primaria: proteger un objeto aunque salga del canal.

Tecnología recomendada:
- CBOR
- COSE
- HPKE

### 3. Identidad
Responsabilidad primaria: determinar quién es el principal.

Tecnología recomendada:
- OpenID Connect para usuarios y clientes
- SPIFFE para workloads

### 4. Autorización
Responsabilidad primaria: determinar qué puede hacer el principal.

Tecnología base:
- scopes
- audience
- resource descriptors
- claims RT
- policy engine simple y explícito

### 5. Custodia de claves
Responsabilidad primaria: firmar, derivar o descifrar sin exponer material sensible.

Tecnología base:
- KMS/HSM externos
- providers desacoplados

---

## Modelo mental recomendado

Piensa RTCrypt como un sistema formado por cuatro planos:

1. **Plano normativo**  
   Perfiles, reglas, allow-lists, amenazas y trust model.

2. **Plano de ejecución**  
   Librerías, middleware, CLI y adaptadores.

3. **Plano de integración**  
   RTPack, RTCore, RTStream, RTCloud.

4. **Plano de operación**  
   Rotación, auditoría, observabilidad, despliegue, incidentes.

---

## Qué debe ser portable

Para que RTCrypt sea útil dentro de RT Stack, debe ser portable a:

- distintos lenguajes;
- distintos clouds;
- distintas topologías de despliegue;
- distintos proveedores de identidad y KMS.

Por eso el repositorio define primero **contratos** y luego implementaciones concretas.

---

## Vocabulario base del proyecto

### Principal
Entidad autenticada que solicita o ejecuta una acción.  
Puede ser `user`, `client` o `workload`.

### Envelope
Contenedor protegido que puede envolver un manifiesto o payload con firma, MAC o cifrado.

### Security Profile
Conjunto nombrado de reglas operativas:
- formatos permitidos,
- algoritmos permitidos,
- validaciones obligatorias,
- límites y defaults.

### Key Provider
Adaptador que permite resolver o usar claves sin acoplar el core a un proveedor concreto.

### Protected Metadata
Metadatos que deben quedar autenticados por la estructura criptográfica.

### Tenant Context
Contexto multi-tenant usado por autorización y resolución de claves.

---

## Resultado esperado para un desarrollador

Después de leer este overview, cualquier desarrollador o agente debería entender:

- qué problema resuelve RTCrypt;
- por qué no se debe inventar criptografía;
- dónde usar seguridad de canal y dónde seguridad de objeto;
- por qué el proyecto separa perfiles, schemas, implementación y ejemplos.

---

## Resultado esperado para Codex

Después de leer este overview, Codex debería inferir correctamente que:

- no debe implementar algoritmos propios;
- debe respetar perfiles y esquemas;
- debe priorizar contratos y validación;
- debe tratar KMS, OIDC y SPIFFE como integraciones, no como funciones internas del core.
