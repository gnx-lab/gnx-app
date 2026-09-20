---
layout: default
title: Inicio rápido
description: Lleva el checkout desde cero hasta una verificación local reproducible.
status: Verificado contra el checkout actual
updated: 2026-09-20
permalink: /quickstart/
---

## 1. Qué necesitas

- Windows con PowerShell.
- Rust estable y el target `x86_64-pc-windows-msvc`.
- .NET SDK para compilar WiX 5.
- Un navegador moderno para la PWA.

No abras `index.html` con `file://`: el Service Worker y la instalación de la PWA necesitan `localhost` o HTTPS.

## 2. Levantar la PWA local

Desde la raíz:

```powershell
python -m http.server 8080
```

Abre `http://localhost:8080/`. La aplicación local intenta consultar el monitor en:

```text
http://127.0.0.1:17890/status
```

Si el servicio no está instalado, la PWA sigue cargando; simplemente mostrará `No conectado`.

## 3. Validar el frontend

```powershell
node --check app.js
node --check sw.js
Get-Content .\manifest.webmanifest -Raw | ConvertFrom-Json
```

El botón de instalación solo aparece cuando el navegador ofrece `beforeinstallprompt`. La PWA no puede crear accesos ni anclarse silenciosamente a la barra de tareas.

## 4. Validar el servicio Rust

```powershell
cargo check --manifest-path installer/service/Cargo.toml
```

En consola, el monitor puede ejecutarse con `--console` usando su `appsettings.json` vecino. El endpoint debe escuchar únicamente en loopback.

## 5. Construir el MSI

```powershell
.\installer\build-installer.ps1 -AppUrl http://localhost:8080/
```

El script compila el binario Rust, copia `appsettings.json`, genera el MSI con WiX y registra `GnxMeshMonitor` cuando se usa `IncludeService=1`.

## Checklist de salida

- [ ] La PWA abre desde `localhost`.
- [ ] `node --check` pasa.
- [ ] `cargo check` pasa.
- [ ] El MSI se genera sin advertencias.
- [ ] `GET /status` devuelve JSON cuando el servicio está instalado.
- [ ] No se han incluido secretos, `obj/`, `target/`, `publish/` ni el MSI en Git.

<div class="callout warning">
<strong>Importante:</strong> el dominio de documentación no sustituye a la PWA local. Pages publica Markdown renderizado; no sirve <code>app.js</code>, el manifest ni el Service Worker.
</div>
