# ADR-0001: Estándares base de RTCrypt

## Estado
Accepted

## Contexto

RTCrypt necesita una base interoperable y mantenible para proteger el ecosistema RT sin crear criptografía propietaria.

---

## Decisión

Se adoptan como estándares base:

- TLS 1.3 para transporte
- COSE/CBOR para seguridad de objeto binario
- HPKE para cifrado híbrido cuando aplique
- OAuth 2.0 / OpenID Connect para identidad de usuarios y clientes
- SPIFFE/SPIRE para identidad de workloads
- HKDF para derivación
- Argon2id solo para credenciales humanas locales si alguna instalación lo necesita
- KMS/HSM externos para custodia de claves

---

## Consecuencias

### Positivas
- buena interoperabilidad;
- menos riesgo conceptual;
- mejor alineación con ecosistemas cloud y de identidad;
- menor necesidad de diseñar formatos nuevos.

### Negativas
- hay que convivir con varios estándares y sus matices;
- algunas integraciones dependen de proveedores externos;
- la documentación debe ser más explícita para evitar confusión.

---

## Alternativas descartadas

### Diseñar protocolo propio RT para todo
Descartado por alto riesgo y alto costo de mantenimiento.

### Usar solo JWT para todos los objetos
Descartado por mala adecuación a objetos binarios y a ciertos casos de persistencia compacta.

### Hacer seguridad solo a nivel TLS
Descartado porque RTPack puede sobrevivir fuera del canal y necesita seguridad de objeto.

---

## Resultado

El repositorio y las implementaciones futuras deben asumir esta decisión como línea base.
