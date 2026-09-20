---
layout: default
title: Desarrollo y despliegue
description: Desarrollo local y publicación de la documentación, separados de la distribución de la PWA.
status: Verificado contra el workflow actual
updated: 2026-09-20
permalink: /deployment/
---

## Desarrollo local

```powershell
python -m http.server 8080
```

Abre `http://localhost:8080/` para probar la PWA. El endpoint opcional del servicio es `http://127.0.0.1:17890/status`.

## GitHub Pages

Cada push a `main` ejecuta `.github/workflows/deploy-pages.yml`. El workflow construye directamente desde `/docs` con Jekyll y publica el HTML resultante. No copia la PWA, el MSI, el servicio Rust ni los artefactos de compilación.

La experiencia publicada está pensada como documentación tipo GitBook: portada, navegación lateral, filtro local de páginas, breadcrumbs, estados de evidencia y referencias cruzadas.

## Puertas de publicación

1. Validar que existan `docs/index.md`, `_config.yml`, `_layouts/default.html`, `_data/navigation.yml` y `assets/docs.css`.
2. Construir el sitio con `actions/jekyll-build-pages`.
3. Rechazar artefactos que contengan `app.js`, `sw.js`, `manifest.webmanifest`, binarios, MSI o archivos de runtime.
4. Subir únicamente el directorio HTML renderizado a Pages.

## Qué no se publica

- el flujo PWA como demo pública;
- `app.gnx` como origen servido por este repositorio;
- una API para `compute.gnx`;
- secretos o información del equipo local.
