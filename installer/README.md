# Instalador Windows de GnX App

El instalador WiX 5 contiene la PWA como acceso y el servicio local `GnxAppMonitor`. El servicio reconcilia un usuario dedicado, WSL Ubuntu 24.04 y un Quadlet Podman; no instala backend de GnX, navegador ni proxy.

## Requisitos

- Rust estable con target `x86_64-pc-windows-msvc`.
- .NET SDK 8 únicamente para compilar WiX.
- Windows con privilegios de administrador para instalar el MSI.
- WiX 5 se restaura desde NuGet.

## Compilación

Desde la raíz:

```powershell
.\installer\build-installer.ps1 -AppUrl http://localhost:8080/
```

El script compila el servicio Rust self-contained y genera `installer/bin/Debug/GnxAppSetup.msi`. Los archivos generados están ignorados por Git.

## Qué instala

- Acceso de Inicio y escritorio a la URL local configurada.
- Servicio automático `GnxAppMonitor`.
- Endpoint local `http://127.0.0.1:17890/status`.
- Usuario Windows dedicado `gnxsvc`, creado solo cuando no hay conflicto de ownership.
- WSL2 con `Ubuntu-24.04`, systemd y `gnx-linux.service` generado desde un Podman Quadlet.

El servicio Rust realiza comprobaciones básicas de la URL configurada, `manifest.webmanifest` y `sw.js`. También reconcilia WSL y el Quadlet cada 120 segundos, expone ese estado por loopback y usa una credencial DPAPI con ACL exclusiva para SYSTEM/Administrators.

> **Impacto de host:** el primer reconcile habilita características de virtualización de Windows y puede programar un reinicio automático con 60 segundos de aviso. Primero pruébalo en un Windows elevado y desechable. Si la virtualización de firmware está desactivada, el proceso se bloquea sin intentar cambiar BIOS/UEFI.

## Pruebas

```powershell
Get-Service GnxAppMonitor
Invoke-WebRequest http://127.0.0.1:17890/status
Get-Content 'C:\ProgramData\GnX App Monitor\provisioning.json' -Raw
wsl -d Ubuntu-24.04 -u root -- systemctl is-active gnx-linux.service
```

Para quitar el producto usa **Aplicaciones instaladas de Windows** o el MSI registrado. No se deben subir el MSI, `bin/`, `obj/` ni `publish/` al repositorio.
