# Control de trabajo del instalable

**Responsable:** agente principal únicamente (sin subagentes)  
**Inicio:** 2026-09-20T11:53:37-06:00  
**Repositorio:** `gnx-lab/gnx-app`  
**Rama:** `main`

## Objetivo

Generar, instalar y validar un MSI de GnX App que incluya el servicio Windows `GnxAppMonitor`, compruebe disponibilidad de GitHub Pages y verifique la integridad de archivos mediante hashes.

## Estado inicial

| ID | Prioridad | Trabajo | Estado | Evidencia |
|---|---:|---|---|---|
| INST-001 | P0 | Confirmar SDK .NET 8 y WiX disponibles | COMPLETADO | `C:\Program Files\dotnet\dotnet.exe` existe; `winget` confirma SDK instalado. WiX llega vía `WixToolset.Sdk` NuGet |
| INST-002 | P0 | Compilar Worker .NET | COMPLETADO | SDK 8.0.425; Release: 0 errores, 0 advertencias |
| INST-003 | P0 | Publicar Worker `win-x64` | COMPLETADO | Publicación self-contained en `installer/service/publish` |
| INST-004 | P0 | Generar MSI completo con servicio | COMPLETADO | WiX 5: 0 errores, 0 advertencias; MSI contiene `MonitorExecutable` y DLLs |
| INST-005 | P0 | Instalar MSI en host de prueba | COMPLETADO | `msiexec`: código 0; archivos instalados |
| INST-006 | P0 | Validar servicio automático | COMPLETADO | `GnxAppMonitor`: `Running`, `Automatic` |
| INST-007 | P1 | Validar disponibilidad de Pages | COMPLETADO | `Invoke-WebRequest`: HTTP 200; URL configurada en host |
| INST-008 | P1 | Validar hashes y detectar alteración | IMPLEMENTADO EN CÓDIGO | Falta crear manifiesto y ejecutar prueba instalada |
| INST-009 | P1 | Probar desinstalación | PENDIENTE | Depende de INST-005 |
| INST-010 | P1 | Firmar MSI | PENDIENTE | Requiere certificado |

## Registro de evidencias

- `installer/service/Program.cs`: Worker con comprobación HTTP y hashes.
- `installer/Product.wxs`: integración condicional del servicio mediante `IncludeService=1`.
- `installer/build-installer.ps1`: publica el Worker self-contained y genera el MSI completo.
- `installer/service/install.ps1` / `uninstall.ps1`: instalación manual elevada y retirada.
- MSI existente en `installer/bin/Debug/`: evidencia de compilación previa; no se considera el MSI final con servicio hasta verificarlo.

## Reglas de control

1. Actualizar esta tabla después de cada comando relevante.
2. No marcar una tarea como completada sin comando, salida o archivo verificable.
3. No subir `bin/`, `obj/`, MSI, certificados ni secretos al repositorio.
4. No usar subagentes para este seguimiento.
5. Registrar bloqueos con fecha, causa y siguiente acción.

## Evidencia 2026-09-20T11:53:55-06:00

- `winget install --id Microsoft.DotNet.SDK.8 --exact`: SDK ya instalado, sin actualización pendiente.
- `C:\Program Files\dotnet\dotnet.exe`: presente.
- `dotnet` no está disponible en el PATH de la sesión actual; se usará la ruta absoluta o se renovará la sesión.

## Evidencia de compilación 2026-09-20T11:54:00-06:00

```text
SDK 8.0.425 / Runtime 8.0.31 / RID win-x64
GnxApp.Monitor -> installer/service/bin/Release/net8.0-windows/GnxAppMonitor.dll
Compilación correcta. 0 Advertencia(s), 0 Errores
```

## Evidencia de instalación 2026-09-20T12:04:03-06:00

- Build del MSI con payload self-contained: 0 errores, 0 advertencias.
- Corrección aplicada: el MSI ahora incluye `GnxAppMonitor.exe`, `appsettings.json`, DLLs, `.deps.json` y `.runtimeconfig.json` en la carpeta del servicio.
- Instalación silenciosa: `exit=0`.
- Servicio: `GnxAppMonitor`, estado `Running`, inicio `Automatic`.
- Pages: `https://gnx-lab.github.io/gnx-app/` respondió HTTP 200.
- `build-installer.ps1` probado end-to-end; ahora localiza el SDK aunque `dotnet` no esté en PATH y escribe `Monitor:AppUrl` en el payload.
- MSI final probado: `msiexec` código 0; `GnxAppMonitor` quedó `Running` y `Automatic`.
- Bloqueo corregido: el primer MSI instalaba solo el ejecutable sin sus dependencias y el servicio no arrancaba.

## Próxima acción

Regenerar el MSI usando el `appsettings.json` actualizado y completar la prueba de manifiesto/hash. Después probar desinstalación y firma.
