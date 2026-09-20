---
layout: default
title: Auditoría de la documentación
description: Cierre de la feature de documentación solicitada por Business y evidencia de lo que queda publicado.
status: Feature completada
updated: 2026-09-20
permalink: /documentation-audit/
---

## Resultado

**La feature de documentación está terminada.** GnX Docs tiene una fuente única en `/docs`, navegación tipo GitBook, publicación docs-only y una frontera explícita entre capacidades verificadas, historial y propuesta de producto.

## Criterios de aceptación

| Criterio Business | Resultado | Evidencia |
|---|---|---|
| Experiencia parecida a GitBook | PASS | Layout propio con sidebar, breadcrumbs, búsqueda de navegación, tarjetas, responsive y referencias cruzadas. |
| Documentación en `/docs` | PASS | `_config.yml`, `_data/navigation.yml`, `_layouts/default.html`, CSS y 13 páginas Markdown. |
| Pages publica documentación, no runtime | PASS | Workflow `deploy-pages.yml` construye `/docs`; runtime PWA/instalables están ausentes del artefacto. |
| Referencia a Obsidiana | PASS | Enlaces externos explícitos a `https://gnx-obsidiana.mayas-alas-yx.chatgpt.site/` en portada, layout y páginas de contexto. |
| Flujo `app.gnx` → `compute.gnx` visible | PASS con límite | Está documentado como flujo histórico/propuesto; `compute.gnx` no se afirma como integración implementada. |
| Navegación funcional | PASS | CI valida enlaces internos después de Jekyll; las páginas públicas principales responden HTTP 200. |
| Seguridad editorial | PASS | Validación de patrones de secretos, enlaces externos con `noopener`, sin estado local ni credenciales en Pages. |
| Enlace desde el root | PASS | `README.md` enlaza portada, quickstart, arquitectura, operación, troubleshooting y recursos. |

## Evidencia de publicación

- Commit de cierre: `cde8c8b`.
- URL: `https://gnx-lab.github.io/gnx-app/`.
- Workflow: `Publicar GnX Docs en GitHub Pages`.
- Última verificación: portada, quickstart, flujo, arquitectura, seguridad, operación, troubleshooting, deployment, installer, resources, roadmap y reference respondieron `200`.
- Comprobación negativa: `app.js`, `sw.js` y `manifest.webmanifest` no se publican en Pages.

## Qué se entrega

- **Descubrir:** portada, mapa de intención y navegación por territorio.
- **Entender:** arquitectura, seguridad, decisiones y estados de evidencia.
- **Operar:** quickstart, despliegue, instalador y troubleshooting.
- **Alinear:** flujo histórico `app.gnx`, propuesta `compute.gnx`, referencias externas y roadmap futuro.
- **Mantener:** workflow con Jekyll, protección docs-only y checker de enlaces internos.

## Cierre de roadmap

La feature queda fuera del roadmap de producto. Los pendientes que permanecen son de producto distribuido —contrato, identidad y operación de `compute.gnx`—, no de documentación.
