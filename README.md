# GnX App

Aplicación web instalable y disponible sin conexión.

## Instalación PWA en Windows

1. Abre la instalación publicada en **https://app.gnx**. Para desarrollo local puede usarse `localhost`; `file://` no permite instalar ni registrar el Service Worker.
2. Abre la URL en Microsoft Edge o Google Chrome actualizado.
3. Usa el icono de instalación de la barra de direcciones o el menú `… > Aplicaciones > Instalar GnX App`.
4. En el diálogo del navegador, activa **crear acceso directo de escritorio** si aparece.

Después de instalarla, Windows la muestra como una aplicación independiente. Desde el menú Inicio se puede elegir `Anclar a Inicio` y, desde la barra de tareas, `Anclar a la barra de tareas`. El menú y los textos exactos dependen de la versión de Windows y del navegador.

### Qué garantiza una PWA y qué no

El manifiesto (`manifest.webmanifest`) y el Service Worker permiten la instalación de la PWA, su ventana independiente y el almacenamiento en caché sin conexión de los recursos incluidos. Edge/Chrome pueden crear el acceso directo de escritorio y el usuario puede anclar la aplicación a Inicio o a la barra de tareas; una PWA no puede crear silenciosamente esos accesos ni elegir sus ubicaciones.

Una PWA tampoco es un instalador de Windows: no crea por sí sola un `.msi`, no instala para todos los usuarios y no puede garantizar el anclaje a la barra de tareas. En este proyecto, el instalador de Windows está en `installer/` y usa **WiX Toolset 5**: genera un MSI con accesos directos que abren la URL HTTPS publicada de la aplicación web. El MSI no empaqueta `localhost`; el anclaje a la barra de tareas sigue requiriendo una acción del usuario.

## Instalación real

El botón `Instalar` solo aparece después de que el navegador emite `beforeinstallprompt` y abre exclusivamente el diálogo nativo. `appinstalled` actualiza el estado cuando el navegador confirma una instalación. Si el navegador no ofrece instalación, debe usarse su menú de aplicaciones.

## Recursos y limitaciones de iconos

Se inspeccionaron los seis recursos originales existentes en `assets/`. Para completar los tamaños habituales de instalación PWA se generaron dos derivados PNG, redimensionando `branding-install-logo.png` sin cambiar su diseño:

- `assets/icon-192.png`: derivado de `branding-install-logo.png`, validado como PNG RGBA de 192×192.
- `assets/icon-512.png`: derivado de `branding-install-logo.png`, validado como PNG RGBA de 512×512.
- `assets/tray-icon.png`: PNG original real de 128×128.
- `assets/branding-install-logo.png`: PNG original real de 64×64.
- `assets/tray-icon.ico`: ICO existente (16–20 px), usado por la página como favicon.
- `assets/branding-install-logo.ico`: ICO existente de 64×64, usado por la interfaz.
- `assets/banner-install-side.png`: PNG de 165×312.
- `assets/bg-installer-banner.png`: PNG de 1254×1254.

Los dos derivados conservan la marca existente y sus dimensiones fueron comprobadas con `file`; el manifiesto declara los cuatro tamaños PNG reales disponibles. No se añadió un propósito `maskable` porque los recursos no fueron diseñados ni validados para el área segura maskable.

## Instalador de Windows (WiX 5)

Con .NET SDK y WiX Toolset 5 restaurado, compila el instalador desde la raíz:

```powershell
dotnet build installer\GnxApp.wixproj -p:AppUrl=https://app.gnx
```

El valor predeterminado de `AppUrl` es **https://app.gnx**. El MSI resultante crea accesos directos en Inicio y escritorio que abren la aplicación web. WiX no puede anclar automáticamente a la barra de tareas.

## Validación rápida

Desde la raíz del proyecto:

```powershell
Get-Content .\manifest.webmanifest -Raw | ConvertFrom-Json
'./', './index.html', './styles.css', './app.js', './sw.js', './manifest.webmanifest', './assets/icon-192.png', './assets/icon-512.png', './assets/tray-icon.ico', './assets/tray-icon.png', './assets/branding-install-logo.ico', './assets/branding-install-logo.png' | % { if (!(Test-Path $_)) { throw "Falta $_" } }
```

La primera línea valida el JSON y la segunda comprueba todas las rutas que precarga el Service Worker. La instalación y el Service Worker deben probarse desde HTTPS o `localhost`, no abriendo `index.html` directamente.

## Modos de ejecución

- **Público (GitHub Pages):** shell PWA sin credenciales. Permite generar un código de sesión de transferencia y comprobar si se está ejecutando instalada.
- **Privado (`localhost`):** acepta el código generado públicamente y prepara la sesión para la API configurada mediante `window.GNX_API_URL`; sólo muestra autenticación cuando la API responde a `/session/verify`.
- **Instalador Windows:** crea accesos a la URL pública. El monitor Windows opcional verifica disponibilidad de la URL y hashes de un manifiesto local; no reemplaza autenticación ni un sistema de actualización.

La autenticación es deliberadamente explícita: un código local no es una credencial válida por sí mismo. No se publican secretos ni endpoints privados en GitHub Pages.

## Publicación y desarrollo

```powershell
# privado
python -m http.server 8080
# después abrir http://localhost:8080

# instalador, usando la URL pública
 dotnet build installer\\GnxApp.wixproj -p:AppUrl=https://mayas-alas.github.io/gnx-app
```

Cada push a `main` publica el shell estático mediante `.github/workflows/deploy-pages.yml`. La configuración de GitHub Pages debe usar **GitHub Actions** como origen.

## Archivos principales

- `index.html`: interfaz y referencias a los recursos.
- `app.js`: flujo `beforeinstallprompt` y evento `appinstalled`.
- `sw.js`: caché offline.
- `manifest.webmanifest`: metadatos y recursos de instalación.
- `assets/`: recursos de marca originales y derivados documentados.
