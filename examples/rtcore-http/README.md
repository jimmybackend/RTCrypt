# Ejemplo RTCore HTTP

## Objetivo

Mostrar un flujo típico de middleware:

- extracción de bearer token;
- validación;
- construcción de principal;
- autorización de una acción.

## Ruta de ejemplo

`POST /streams/stream-01/publish`

## Resultado esperado

- `401` si token inválido
- `403` si principal válido pero sin permiso
- `200/202` si autorizado
