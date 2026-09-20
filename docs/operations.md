---
layout: default
title: Operación
description: Cómo construir, verificar y observar la instalación sin confundir un build con una instalación lista.
status: Verificado contra el checkout actual
updated: 2026-09-20
permalink: /operations/
---

## Flujo operativo

```text
servir PWA → instalar monitor → comprobar /status → construir MSI → validar instalación
```

Cada paso produce una evidencia distinta. Que el MSI compile no prueba que el servicio esté instalado; que `/status` responda no prueba que el frontend pueda autenticarse.

## Servicio local

El contrato actual es:

```http
GET http://127.0.0.1:17890/status
```

La respuesta incluye el estado del servicio, la URL comprobada, la disponibilidad del manifest y del Service Worker, además de la hora de la última comprobación.

```powershell
Invoke-WebRequest http://127.0.0.1:17890/status | Select-Object -Expand Content
Get-Service GnxAppMonitor
```

El monitor realiza comprobaciones periódicas de la URL configurada. Configura `AppUrl`, `IntervalSeconds`, `RequestTimeoutSeconds` y `ListenPort` en `installer/service/appsettings.json` o durante el build; no pongas secretos en ese archivo.

## Build del MSI

```powershell
.\installer\build-installer.ps1 -AppUrl http://localhost:8080/
```

El build actual requiere Cargo y dotnet para WiX. El binario release se copia al payload ignorado de `installer/service/publish/`; el MSI se genera en `installer/bin/`.

## Evidencia útil

Guarda en el ticket o release:

- commit exacto;
- comando ejecutado;
- resultado de `cargo check`;
- resultado de `node --check`;
- resultado de `dotnet build`;
- respuesta sanitizada de `/status`;
- versión de Windows, Rust, dotnet y WiX.

No guardes tokens, rutas de usuario innecesarias, cookies ni dumps completos de equipos.

## Lo que sigue siendo manual

- El navegador decide si ofrece el prompt de instalación PWA.
- Windows decide cómo se muestran y anclan los accesos.
- La instalación del MSI y la verificación del servicio requieren un equipo Windows real.
- La conexión remota a `compute.gnx` no forma parte del monitor actual.
