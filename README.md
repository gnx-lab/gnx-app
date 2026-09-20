# GnX App

GnX App es una PWA local con un monitor Windows Rust. La documentación pública vive en [`/docs`](docs/index.md) y se publica en GitHub Pages como Markdown renderizado con Jekyll; Pages no publica la PWA, el MSI ni el servicio.

## Empieza por la documentación

- [GnX Docs](docs/index.md)
- [Inicio rápido](docs/quickstart.md)
- [Mapa del producto](docs/product-flow.md)
- [Arquitectura](docs/architecture.md)
- [Seguridad y límites](docs/security.md)
- [Operación](docs/operations.md)
- [Troubleshooting](docs/troubleshooting.md)
- [Recursos y referencias](docs/resources.md)
- [Auditoría de la documentación](docs/documentation-audit.md)
- [Tracking WSL y Quadlet](docs/wsl-quadlet-tracking.md)

La experiencia editorial toma como referencia [GitBook de GnX](https://gnx.gitbook.io/docs) y el contexto externo de [gnx-obsidiana](https://gnx-obsidiana.mayas-alas-yx.chatgpt.site/). Son referencias de lectura, no dependencias de ejecución.

## Ejecutar localmente

```powershell
python -m http.server 8080
```

Abre `http://localhost:8080/`. El estado del servicio, si está instalado, está en `http://127.0.0.1:17890/status`.

## Construir el instalable

```powershell
.\installer\build-installer.ps1 -AppUrl http://localhost:8080/
```

El MSI instala accesos locales y el servicio `GnxMeshMonitor`. Revisa [desarrollo y despliegue](docs/deployment.md) y [control del instalable](docs/installer-worklog.md) antes de distribuirlo.

## Flujo de producto histórico y futuro

`https://app.gnx` aparece en el historial como entrada pública de la PWA. El flujo solicitado `app.gnx → Conectar → https://compute.gnx` está documentado como propuesta: `compute.gnx` no aparece en los commits disponibles ni tiene un contrato implementado en este checkout.

## Estructura

```text
index.html / styles.css / app.js     PWA local
manifest.webmanifest / sw.js         instalación y offline
assets/                               marca e iconos
installer/                            MSI WiX y servicio Windows
docs/                                 documentación fuente y sitio Jekyll
.github/workflows/                    publicación docs-only
```

## Estado honesto

No hay backend, autenticación remota, cliente de `compute.gnx`, proxy SOCKS, sincronización remota ni reparación automática. La documentación etiqueta cada afirmación como verificada, histórica, propuesta o bloqueada para que una pantalla o un dominio no se confundan con una capacidad implementada.
