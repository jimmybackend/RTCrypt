# Ejemplo RTStream mTLS

## Objetivo

Explicar un escenario interno donde dos workloads se comunican por TLS 1.3 con autenticación mutua.

## Flujo conceptual

1. El productor presenta identidad.
2. El servidor valida certificado o SVID.
3. Se establece conexión segura.
4. Se autoriza `publish` para el stream solicitado.
5. Se audita la operación.

## Punto importante

La identidad del canal no reemplaza la política de stream.
