# Test vectors

Esta carpeta contendrá vectores de prueba positivos y negativos.

## Objetivo

Asegurar que la implementación:

- firma y verifica correctamente;
- rechaza envelopes malformados;
- rechaza profiles no soportados;
- detecta digests incorrectos;
- rechaza claims fuera de política;
- detecta errores de tenant scope.

## Reglas

- Todos los vectores deben ser reproducibles.
- Los vectores deben documentar su intención.
- Deben existir casos negativos, no solo felices.
