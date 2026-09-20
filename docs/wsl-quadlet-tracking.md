---
layout: default
title: Tracking WSL y Podman Quadlet
description: Objetivos, estados y evidencia del reconciliador Windows → Ubuntu 24.04 → Podman Quadlet.
status: Implementación lista; aceptación de host pendiente
updated: 2026-09-20
permalink: /wsl-quadlet-tracking/
---

## Alcance cerrado

El servicio Windows `GnxMeshMonitor` reconcilia una cadena mínima y explícita:

```text
Servicio Windows (LocalSystem)
  → usuario local dedicado gnxmeshsvc
  → WSL2 + Ubuntu-24.04 registrada como gnx-mesh
  → systemd de WSL
  → Podman Quadlet
  → gnx-mesh.service generado por Quadlet
```

No instala un backend de GnX, no conecta `compute.gnx`, no abre puertos públicos y no convierte el monitor de disponibilidad en un sistema de autenticación.

## Tabla de objetivos

| ID | Objetivo | Implementación | Estado | Evidencia / gate |
|---|---|---|---|---|
| WSL-001 | Crear una identidad Windows dedicada | Crea `gnxmeshsvc` solo si no existe; bloquea conflictos de ownership | Implementado | `DEDICATED_USER_CONFLICT` o `userReady=true` en estado |
| WSL-002 | Proteger la credencial de la cuenta gestionada | Guarda una credencial DPAPI en `ProgramData`, ACL para SYSTEM/Administrators | Implementado | `gnxmeshsvc.credential.xml`; nunca se escribe el secreto en logs |
| WSL-003 | Activar requisitos de host | Habilita `Microsoft-Windows-Subsystem-Linux`, `VirtualMachinePlatform` y `hypervisorlaunchtype=auto` | Implementado | `VIRTUALIZATION_DISABLED`, `REBOOT_REQUIRED` o siguiente fase |
| WSL-004 | Reiniciar sin bucle | Persiste boot ID y programa un reinicio único de 60 s cuando es necesario | Implementado | `rebootPending`, `rebootBootId`; el servicio vuelve con inicio automático |
| WSL-005 | Instalar Ubuntu 24.04 bajo la identidad dedicada | Usa `wsl --install Ubuntu-24.04 --name gnx-mesh --location ... --version 2 --no-launch --web-download` como `gnxmeshsvc` | Implementado | `WSL_INSTALL_FAILED` o distribución `gnx-mesh` registrada |
| WSL-006 | Adoptar systemd en WSL | Activa `systemd=true` y reinicia solo WSL para aplicar el cambio | Implementado | `WSL_RESTART_REQUIRED` antes de continuar |
| WSL-007 | Adoptar Podman Quadlets | Instala Podman y escribe `/etc/containers/systemd/gnx-mesh.container` | Implementado | Quadlet fuente gestionado por el reconciliador |
| WSL-008 | Servicio Linux resiliente | systemd genera `gnx-mesh.service`; Quadlet usa `Restart=always`, 5 s de backoff, rootfs read-only y red aislada | Implementado | `systemctl is-active gnx-mesh.service` |
| WSL-009 | Resiliencia del servicio Windows | WiX configura inicio automático y reinicio tras fallos; el monitor reconcilia cada 120 s | Implementado | configuración del MSI y `provisioning*` en `/status` |
| WSL-010 | Aceptación en host real | Instalar MSI, permitir reboot, confirmar distro y Quadlet después del arranque | Pendiente | Equipo Windows desechable/elevado; no se afirma READY sin esta evidencia |

## Estados operativos

| Código | Significado | Acción segura |
|---|---|---|
| `PROVISIONED` | Usuario, WSL, Ubuntu, Podman y Quadlet están activos | Observar `/status`; el reconciliador mantiene el estado. |
| `REBOOT_REQUIRED` / `REBOOT_PENDING` | Windows debe reiniciarse o el reinicio ya fue programado | Dejar que el host reinicie; no ejecutar instalaciones paralelas. |
| `WSL_RESTART_REQUIRED` | Se activó systemd en `/etc/wsl.conf` | El reconciliador ejecuta `wsl --shutdown` y reintenta. |
| `VIRTUALIZATION_DISABLED` | VT-x/AMD-V está desactivado en firmware | Activar virtualización en BIOS/UEFI y reintentar. |
| `DEDICATED_USER_CONFLICT` | `gnxmeshsvc` existe pero no es propiedad de GnX Mesh | Resolver con un administrador; nunca adoptar la cuenta a ciegas. |
| `BLOCKED` / `RECOVERY_REQUIRED` | Un requisito o estado persistido impide continuar | Revisar `provisioning.json` y corregir solo la causa indicada. |

## Contrato de observación local

```http
GET http://127.0.0.1:17890/status
```

Campos añadidos:

```json
{
  "provisioningPhase": "READY",
  "provisioningCode": "PROVISIONED",
  "provisioningRebootPending": false,
  "dedicatedUser": "gnxmeshsvc",
  "wslDistribution": "gnx-mesh",
  "linuxService": "running"
}
```

`provisioningPhase` es el estado de reconciliación, no una prueba de que una carga de negocio o `compute.gnx` esté lista.

## Validación requerida antes de cerrar WSL-010

```powershell
Get-Service GnxMeshMonitor
Get-Content 'C:\ProgramData\GnX Mesh\provisioning.json' -Raw
Invoke-WebRequest http://127.0.0.1:17890/status | Select-Object -Expand Content
# Bajo la identidad gestionada o con un administrador autorizado:
wsl -d gnx-mesh -u root -- systemctl is-active gnx-mesh.service
wsl -d gnx-mesh -u root -- systemctl cat gnx-mesh.service
```

La instalación altera características de Windows, crea una cuenta local, descarga Ubuntu/Podman y puede reiniciar el host. Debe probarse solo en un equipo Windows elevado y desechable antes de distribuir el MSI.
