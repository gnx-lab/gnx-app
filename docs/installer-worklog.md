---
layout: default
title: Control del instalable
description: Historial verificable del MSI, el monitor Rust y las decisiones que aún requieren una prueba de instalación real.
status: Evidencia de build; instalación real pendiente de equipo
updated: 2026-09-20
permalink: /installer-worklog/
---

## Estado

| ID | Trabajo | Estado | Evidencia |
|---|---|---|---|
| INST-001 | PWA local consulta el servicio | Completado | `app.js` usa `127.0.0.1:17890/status`. |
| INST-002 | Servicio Windows en Rust | Completado | `installer/service/Cargo.toml` y `src/main.rs`. |
| INST-003 | Comprobar URL, manifest y Service Worker | Completado | El monitor realiza tres GET. |
| INST-004 | Endpoint local de estado | Completado | `GET /status` en loopback. |
| INST-005 | MSI con ejecutable Rust | Build verificado | WiX compila sin errores en este checkout. |
| INST-006 | Instalar y arrancar servicio | Pendiente de repetir | Requiere instalación real y permisos de Windows. |
| INST-007 | Desinstalar sin residuos | Pendiente | Requiere validar servicio, carpeta y accesos. |

## Decisiones

- Rust es el servicio actual; la implementación C# histórica fue eliminada del checkout.
- El servicio solo escucha en `127.0.0.1`.
- Pages publica únicamente documentación.
- `app.gnx` es una referencia histórica; `compute.gnx` es una propuesta sin contrato versionado.
- Un código de sesión local no es una credencial válida por sí mismo.

## Evidencia conocida

```text
cargo check: correcto
cargo build --release: correcto
MSI WiX: 0 errores, 0 advertencias
GET /status: contrato definido en loopback
```

Esta evidencia no sustituye la instalación en un Windows limpio, el arranque automático, una prueba de reinicio ni la desinstalación completa.

## Próxima acción

Ejecutar una instalación controlada, capturar únicamente evidencia sanitizada y actualizar el estado de INST-006/007 con comandos, versiones, resultado y residuos observados.
