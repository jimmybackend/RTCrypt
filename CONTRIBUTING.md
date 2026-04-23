# Guía de contribución

Gracias por contribuir a **RTCrypt**.

Este proyecto no es una colección de snippets sueltos. Es una base de seguridad para un ecosistema completo, por lo que cada contribución debe priorizar claridad, trazabilidad y compatibilidad con estándares.

---

## 1. Antes de abrir un PR

Lee primero:

- `README.md`
- `specs/architecture.md`
- `specs/threat-model.md`
- `specs/trust-model.md`
- `specs/profiles/`
- `schemas/`

Si tu cambio altera comportamiento, formato, algoritmo permitido o contrato entre módulos, primero crea o actualiza un **ADR**.

---

## 2. Tipos de contribuciones aceptadas

- Correcciones de documentación
- Implementación de módulos definidos en specs
- Mejoras de tests y conformance
- Adaptadores a KMS o identity providers
- Ejemplos operativos
- Hardening de validación y manejo de errores

No se aceptarán contribuciones cuyo efecto sea:

- introducir criptografía casera;
- mezclar autorización y parsing criptográfico;
- usar configuraciones inseguras por defecto;
- relajar validaciones para “hacerlo más cómodo”;
- exponer secretos en ejemplos o fixtures.

---

## 3. Flujo recomendado

1. Abre una issue.
2. Declara alcance y componente.
3. Referencia la especificación afectada.
4. Implementa cambios pequeños y revisables.
5. Añade tests positivos y negativos.
6. Actualiza documentación si cambia comportamiento.
7. Envía PR con checklist completo.

---

## 4. Convenciones de diseño

### 4.1 Separación por capas
Mantén separados:

- parsing
- verificación criptográfica
- resolución de claves
- autenticación
- autorización
- serialización
- IO / red / filesystem

### 4.2 Seguridad por defecto
La configuración por defecto debe ser conservadora.

### 4.3 Contratos explícitos
Todo tipo relevante debe tener semántica clara:
- qué representa
- de dónde viene
- qué validaciones requiere
- qué errores puede devolver

### 4.4 Errores auditables
Los errores deben ser distinguibles para depuración, pero no deben filtrar secretos.

---

## 5. Requisitos de PR

Todo PR debería incluir:

- motivación del cambio;
- componente afectado;
- riesgos;
- tests añadidos;
- documentación actualizada;
- compatibilidad o ruptura de compatibilidad.

### Plantilla mínima sugerida

```text
## Objetivo
## Alcance
## Specs afectadas
## Riesgos
## Tests
## Compatibilidad
## Pendientes
```

---

## 6. Reglas para cambios criptográficos

Cualquier cambio en:
- algoritmos permitidos,
- parámetros criptográficos,
- formatos de envelope,
- políticas de verificación,
- validación de claims,

requiere:

1. justificación técnica,
2. referencia normativa,
3. tests,
4. actualización de perfil,
5. revisión adicional.

---

## 7. Estilo de documentación

La documentación debe ayudar tanto a humanos como a agentes de programación.

Por eso se recomienda:

- títulos claros;
- listas con reglas operativas;
- ejemplos mínimos pero reales;
- diferenciar “debe”, “puede” y “no debe”;
- indicar explícitamente out-of-scope cuando aplique.

Evita documentación ambigua del tipo “se podría considerar”.

---

## 8. Estructura de commits sugerida

Ejemplos:

- `docs: define rtpack cose v1 profile`
- `core: add security profile types`
- `cose: verify protected header typing`
- `auth: validate issuer and audience`
- `kms: add aws kms mock provider`
- `tests: add negative vectors for bad kid`

---

## 9. Testing esperado

Todo componente debe tener, como mínimo:

- tests unitarios;
- tests de error;
- tests de entradas inválidas;
- fixtures reproducibles;
- tests de compatibilidad con el perfil definido.

Los parsers y validadores deben probar explícitamente casos negativos.

---

## 10. Qué debe hacer Codex al contribuir

Si usas un agente tipo Codex:

1. Leer primero `README.md` y `specs/`.
2. No crear componentes no definidos.
3. No inventar claims RT sin documentarlos.
4. No agregar dependencias criptográficas sin justificar.
5. No asumir que TODO es JWT.
6. Respetar `schemas/` y perfiles.
7. Mantener bajo acoplamiento.

---

## 11. Checklist final

Antes de merge:

- [ ] El cambio respeta los perfiles definidos
- [ ] No introduce defaults inseguros
- [ ] No filtra secretos
- [ ] Tiene tests positivos y negativos
- [ ] Documenta el comportamiento nuevo
- [ ] No rompe contratos sin ADR
