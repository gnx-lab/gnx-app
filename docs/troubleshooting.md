---
layout: default
title: Troubleshooting
description: Diagnóstico por síntomas, con comandos seguros y una frontera clara entre fallo local y trabajo futuro.
status: Operativo
updated: 2026-09-20
permalink: /troubleshooting/
---

## La PWA no abre o no instala

1. Comprueba que el servidor está activo: `python -m http.server 8080`.
2. Usa `http://localhost:8080/`, no `file://`.
3. Revisa la consola del navegador y el registro del Service Worker.
4. Si el navegador no ofrece `beforeinstallprompt`, usa su menú de aplicaciones; no es un error que GnX pueda resolver desde JavaScript.

## La fila «Servicio local» dice «No conectado»

```powershell
Get-Service GnxMeshMonitor -ErrorAction SilentlyContinue
Invoke-WebRequest http://127.0.0.1:17890/status -TimeoutSec 3
```

Si el servicio no existe, construye e instala el MSI con permisos adecuados. Si existe pero el endpoint no responde, revisa que `ListenPort` no esté ocupado y que el servicio pueda leer el `appsettings.json` vecino al ejecutable.

## El monitor dice «Revisar URL»

Comprueba de forma independiente:

```powershell
Invoke-WebRequest http://localhost:8080/
Invoke-WebRequest http://localhost:8080/manifest.webmanifest
Invoke-WebRequest http://localhost:8080/sw.js
```

El monitor no puede verificar una URL que no esté servida y no convierte un error de red en una sesión autenticada.

## El MSI falla al compilar

- Confirma que `cargo` y `dotnet` están en `PATH`.
- Usa el target Rust `x86_64-pc-windows-msvc`.
- Borra únicamente `installer/service/publish/` si hay un payload antiguo.
- Ejecuta el build desde la raíz del repo.
- Conserva el primer error completo; no publiques el MSI hasta entenderlo.

## «Conectar» no llega a `compute.gnx`

Eso no es un fallo soportado por la implementación actual: el checkout no contiene un cliente remoto ni un contrato de `compute.gnx`. Consulta [Mapa del producto]({{ '/product-flow/' | relative_url }}) y no intentes solucionar la ausencia de contrato cambiando el timeout o exponiendo el endpoint local.

## Regla de escalamiento

Si el problema requiere credenciales, cambios DNS, permisos de Windows, despliegue remoto o modificación del contrato de `compute.gnx`, detén el diagnóstico local y abre una tarea con evidencia mínima y sanitizada.
