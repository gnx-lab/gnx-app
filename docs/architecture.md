---
layout: default
title: Arquitectura
description: Una vista de extremo a extremo de la PWA local, el Service Worker, el MSI y el monitor Windows.
status: Verificado contra el checkout actual
updated: 2026-09-20
permalink: /architecture/
---

## Vista de sistema

```text
Navegador
  └─ PWA local (index.html + app.js + Service Worker)
       └─ GET http://127.0.0.1:17890/status
            └─ GnxMeshMonitor (Rust / Windows Service)
                 └─ GET a la URL configurada
```

GitHub Pages aloja esta documentación, no el sistema operativo anterior. El sitio de docs no participa en la operación local ni en una futura conexión a `compute.gnx`.

## Piezas

| Pieza | Responsabilidad | Evidencia |
|---|---|---|
| PWA | Estado de sesión local, instalación y estado del monitor | `index.html`, `app.js`, `manifest.webmanifest` |
| Service Worker | Cachear el shell y permitir uso offline | `sw.js` |
| Monitor | Comprobar URL, manifest y Service Worker; exponer `/status` | `installer/service/src/main.rs` |
| MSI | Instalar accesos y registrar el servicio | `installer/Product.wxs` |
| Docs | Renderizar guía y referencia sin runtime | `docs/`, workflow de Pages |

## Ciclo de comprobación

1. El monitor lee `appsettings.json` junto al ejecutable.
2. Hace GET a `AppUrl`, `/manifest.webmanifest` y `/sw.js`.
3. Guarda el último resultado en memoria.
4. Expone solo `GET /status` en loopback.
5. La PWA local consulta el JSON con timeout y muestra un estado humano.

El monitor verifica disponibilidad HTTP, no salud de negocio, identidad, permisos ni autorización remota.

## Contrato `/status`

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

La respuesta se sirve como JSON y el CORS solo permite orígenes `localhost`/`127.0.0.1`. El endpoint no debe exponerse mediante un proxy público.

## Publicación docs-only

El workflow:

1. toma `/docs` como raíz Jekyll;
2. aplica `_layouts/default.html`, navegación y CSS locales;
3. genera HTML en `_site`;
4. rechaza runtime, instalables y posibles secretos;
5. publica únicamente `_site`.

Este diseño permite una experiencia similar a GitBook sin convertir Pages en el hosting de la PWA.

## Frontera con `app.gnx` y `compute.gnx`

`https://app.gnx` es un punto de entrada histórico documentado en los commits originales. `https://compute.gnx` es una dirección de producto solicitada, pero no existe un cliente, contrato o prueba versionada en este checkout. La arquitectura presente no dibuja un canal remoto que no pueda señalarse en código.
