---
layout: default
title: Recursos y referencias
description: Enlaces externos, historial local y reglas para usar contexto sin convertirlo en dependencia.
status: Referencias etiquetadas
updated: 2026-09-20
permalink: /resources/
---

## Contexto GnX Obsidiana

<a href="https://gnx-obsidiana.mayas-alas-yx.chatgpt.site/" target="_blank" rel="noopener noreferrer">Abrir gnx-obsidiana.mayas-alas-yx.chatgpt.site ↗</a>

Este sitio externo se incorpora como referencia de exploración y lenguaje de producto. No es una API de GnX App, no recibe el estado de `GnxAppMonitor`, no autentica esta documentación y no debe tratarse como una dependencia de build o runtime.

## Referencia editorial

<a href="https://gnx.gitbook.io/docs" target="_blank" rel="noopener noreferrer">Abrir GitBook original ↗</a>

GnX Docs adopta de esa experiencia los principios de navegación lateral, páginas orientadas a intención, lectura progresiva y referencias cruzadas. La implementación es propia: Jekyll, Markdown y CSS dentro de `/docs`.

## Referencias de producto

- <a href="https://app.gnx" target="_blank" rel="noopener noreferrer">app.gnx ↗</a> — entrada histórica de la PWA descrita en commits anteriores.
- <a href="https://compute.gnx" target="_blank" rel="noopener noreferrer">compute.gnx ↗</a> — destino nombrado en el brief actual; su disponibilidad y contrato no están verificados por este repo.

Al abrir dominios de producto se aplican las mismas reglas: observar, no introducir secretos y no inferir capacidades a partir de una pantalla que no tenga contrato publicado.

## Fuentes dentro del repo

- `README.md`: entrada para colaboradores y build.
- `docs/architecture.md`: piezas y límites técnicos.
- `docs/deployment.md`: desarrollo y despliegue local.
- `docs/installer-worklog.md`: historial operativo del MSI y del monitor.
- `.github/workflows/deploy-pages.yml`: política de publicación docs-only.
- `git log --all`: evidencia del cambio histórico de `app.gnx` a PWA local.
