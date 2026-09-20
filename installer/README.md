# Instalador Windows de GnX App

El instalador WiX 5 contiene la PWA como acceso y el servicio local `GnxAppMonitor`. No instala backend, navegador ni proxy.

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

El servicio Rust realiza comprobaciones básicas de la URL configurada, `manifest.webmanifest` y `sw.js`. Expone el estado por loopback y no almacena credenciales.

## Pruebas

```powershell
Get-Service GnxAppMonitor
Invoke-WebRequest http://127.0.0.1:17890/status
```

Para quitar el producto usa **Aplicaciones instaladas de Windows** o el MSI registrado. No se deben subir el MSI, `bin/`, `obj/` ni `publish/` al repositorio.
