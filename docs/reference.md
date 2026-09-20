---
layout: default
title: Referencia y decisiones
description: Contratos pequeños, estados explícitos y decisiones que deben sobrevivir a futuras iteraciones.
status: Verificado + decisiones pendientes etiquetadas
updated: 2026-09-20
permalink: /reference/
---

## URLs por contrato

| URL | Papel | Estado |
|---|---|---|
| `http://localhost:8080/` | PWA durante desarrollo local | Ejecutable localmente |
| `http://127.0.0.1:17890/status` | Estado del monitor Windows | Contrato implementado |
| `https://app.gnx` | Entrada histórica de la PWA | Evidencia en historial; no publicada por este workflow |
| `https://compute.gnx` | Destino de cómputo solicitado | Propuesta; sin evidencia versionada |
| `https://gnx-obsidiana.mayas-alas-yx.chatgpt.site/` | Contexto editorial externo | Referencia navegable, no dependencia |
| `https://gnx.gitbook.io/docs` | Referencia de experiencia documental | Inspiración editorial, no dependencia |

## Estados de evidencia

- **Verificado:** se puede señalar un archivo, test o commit del checkout.
- **Histórico:** aparece en un commit anterior pero no necesariamente en el comportamiento actual.
- **Propuesto:** forma parte del brief o de un diseño futuro.
- **Bloqueado:** necesita una decisión, credencial, host o prueba que no está disponible.

## Servicio local

```json
{
  "serviceRunning": true,
  "urlAvailable": true,
  "manifestAvailable": true,
  "serviceWorkerAvailable": true,
  "appUrl": "http://localhost:8080/",
  "checkedAt": 0
}
```

Los nombres del JSON son camelCase. `checkedAt` es un timestamp Unix en segundos. Un `true` del monitor solo demuestra la comprobación HTTP configurada; no representa autorización de usuario.

## Convenciones editoriales

Cada página debe incluir:

- objetivo claro en el primer párrafo;
- estado de evidencia y fecha de actualización;
- comandos reproducibles o un enlace a la especificación pendiente;
- límites explícitos;
- enlaces externos con `target="_blank"` y `rel="noopener noreferrer"` cuando se escriban en HTML.

La navegación de la izquierda es una ayuda de lectura, no un índice de APIs. Las rutas futuras se añaden después de existir un contrato verificable.
