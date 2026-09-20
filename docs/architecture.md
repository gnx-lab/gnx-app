# Arquitectura de GnX App

## 1. Alcance actual

GnX App es una aplicaciÃ³n web estÃ¡tica instalable como PWA. La versiÃ³n actual no tiene backend, base de datos ni servicios de aplicaciÃ³n: entrega una interfaz local, registra un Service Worker y permite la instalaciÃ³n nativa cuando el navegador la ofrece.

La distribuciÃ³n Windows se realiza mediante un instalador WiX Toolset 5 que crea accesos directos a una URL HTTPS publicada. El instalador no contiene un servidor ni convierte la PWA en una aplicaciÃ³n nativa.

## 2. Contexto del sistema

```mermaid
flowchart LR
    U[Usuario]
    B[Navegador compatible]
    H[Hosting HTTPS: app.gnx]
    P[GnX App PWA]
    W[Windows]
    I[MSI WiX 5]

    U --> B
    B -->|GET recursos estÃ¡ticos| H
    H --> P
    B -->|InstalaciÃ³n PWA| W
    I -->|Accesos Inicio y escritorio| W
    W -->|Abre URL publicada| B
```

## 3. Componentes

```mermaid
flowchart TB
    subgraph Source[Fuente del proyecto]
        HTML[index.html]
        CSS[styles.css]
        JS[app.js]
        MAN[manifest.webmanifest]
        SW[sw.js]
        ASSETS[assets/]
    end

    subgraph Browser[Navegador]
        UI[Interfaz GnX App]
        INSTALL[Prompt nativo de instalaciÃ³n]
        CACHE[Cache Storage]
        STANDALONE[Ventana standalone]
    end

    HTML --> UI
    CSS --> UI
    JS --> UI
    MAN --> INSTALL
    MAN --> STANDALONE
    ASSETS --> UI
    SW --> CACHE
    SW --> UI
    UI -->|beforeinstallprompt| INSTALL
    INSTALL -->|appinstalled| STANDALONE
```

## 4. Flujo de ejecuciÃ³n y offline

```mermaid
sequenceDiagram
    participant User as Usuario
    participant Browser as Navegador
    participant Host as Host HTTPS app.gnx
    participant SW as Service Worker
    participant Cache as Cache Storage

    User->>Browser: Abre la URL
    Browser->>Host: Solicita index.html y recursos
    Host-->>Browser: HTML, CSS, JS, manifest y assets
    Browser->>SW: Registra sw.js
    SW->>Host: Precarga APP_SHELL
    SW->>Cache: Guarda cache gnx-app-v5

    User->>Browser: Navega o recarga
    Browser->>SW: Solicita recurso GET
    SW->>Cache: Busca respuesta existente
    Cache-->>SW: Respuesta cacheada, si existe
    SW->>Host: Actualiza desde red cuando corresponde
    SW-->>Browser: Respuesta online u offline
```

El Service Worker aplica estas polÃ­ticas:

- InstalaciÃ³n: precache del shell y los iconos del manifest.
- NavegaciÃ³n: red primero; si falla, sirve `index.html` desde cachÃ©.
- Recursos GET: devuelve cachÃ© inmediatamente y actualiza desde red.
- ActivaciÃ³n: elimina versiones antiguas `gnx-app-*`.
- Peticiones externas o mÃ©todos distintos de `GET`: quedan fuera del Service Worker.

## 5. InstalaciÃ³n PWA

```mermaid
sequenceDiagram
    participant User as Usuario
    participant App as app.js
    participant Browser as Navegador
    participant OS as Sistema operativo

    Browser-->>App: beforeinstallprompt
    App->>App: Guarda el evento y muestra Instalar
    User->>App: Pulsa Instalar
    App->>Browser: prompt()
    Browser->>User: DiÃ¡logo nativo
    User-->>Browser: Acepta o cancela
    Browser-->>App: userChoice
    Browser-->>App: appinstalled si se completÃ³
    Browser->>OS: Registra la PWA
```

La pÃ¡gina no crea accesos directos de Windows directamente. La ubicaciÃ³n final â€”Inicio, escritorio o barra de tareasâ€” depende del navegador y de la acciÃ³n del usuario.

## 6. DistribuciÃ³n Windows con WiX 5

```mermaid
flowchart LR
    URL[AppUrl HTTPS] --> BUILD[dotnet build]
    WIX[WiX Toolset 5 vÃ­a NuGet] --> BUILD
    SOURCE[installer/Product.wxs] --> BUILD
    BUILD --> MSI[GnxAppSetup.msi]
    MSI --> START[Acceso en menÃº Inicio]
    MSI --> DESKTOP[Acceso en escritorio]
    START --> URL
    DESKTOP --> URL
```

CaracterÃ­sticas actuales del MSI:

- InstalaciÃ³n por usuario (`Scope=perUser`).
- URL configurable mediante `AppUrl`.
- Acceso en el menÃº Inicio.
- Acceso en el escritorio.
- DesinstalaciÃ³n de entradas y carpetas administradas.
- No instala runtime, servidor, navegador ni backend.
- No fija automÃ¡ticamente la aplicaciÃ³n en la barra de tareas.
- El MSI generado localmente no estÃ¡ firmado digitalmente.

Comando de compilaciÃ³n:

```powershell
dotnet build installer\GnxApp.wixproj -p:AppUrl=https://app.gnx
```

## 7. Mapa de archivos

| Ruta | Responsabilidad |
|---|---|
| `index.html` | Estructura y controles de la interfaz |
| `styles.css` | PresentaciÃ³n visual y estados de foco |
| `app.js` | InstalaciÃ³n PWA real y mensajes de estado |
| `manifest.webmanifest` | Identidad, iconos y modo standalone |
| `sw.js` | Precache, cachÃ© offline y actualizaciÃ³n |
| `assets/` | Iconos y recursos de marca |
| `installer/GnxApp.wixproj` | Proyecto WiX 5 |
| `installer/Product.wxs` | MSI, accesos y propiedades |
| `docs/` | DocumentaciÃ³n tÃ©cnica |

## 8. Contratos y lÃ­mites

### Contratos actuales

- La instalación pública se sirve desde la URL HTTPS de GitHub Pages (o un dominio personalizado configurado allí); `localhost` queda reservado para desarrollo.
- El manifest y el Service Worker deben permanecer en el mismo origen.
- `AppUrl` apunta por defecto a `https://app.gnx`; debe ser accesible desde los dispositivos de destino antes de distribuir el MSI.
- Los iconos declarados en el manifest deben existir y estar incluidos en el shell offline.

### Fuera del alcance actual

- Servicios de backend.
- AutenticaciÃ³n y autorizaciÃ³n.
- Persistencia remota.
- SincronizaciÃ³n de datos entre dispositivos.
- Firma de cÃ³digo del MSI.
- FijaciÃ³n automÃ¡tica en la barra de tareas.

## 9. Estado de calidad

Validaciones realizadas:

- `node --check app.js`
- `node --check sw.js`
- JSON vÃ¡lido para `manifest.webmanifest`.
- Rutas del manifest y `APP_SHELL` verificadas.
- CompilaciÃ³n WiX 5 verificada con .NET SDK 8.
- Build MSI actual: 0 errores y 0 advertencias.
- Artefactos `bin/`, `obj/`, `.msi` y `.wixpdb` excluidos por `.gitignore`.

## 10. Frontera pÃºblica y privada

La publicaciÃ³n pÃºblica contiene Ãºnicamente el shell estÃ¡tico de la PWA (`index.html`,
CSS, JavaScript, manifest, Service Worker y `assets/`). El workflow de GitHub Pages
construye `_site/` con esos archivos y no publica `installer/` ni `docs/` como parte
del sitio. GitHub Pages es hosting estÃ¡tico: no es un lÃ­mite de seguridad ni un lugar
para secretos.

El desarrollo local se sirve en `localhost` y es la Ãºnica superficie prevista para
probar integraciones privadas. Una futura API, proxy SOCKS o fuente de datos privada
debe ejecutarse fuera de GitHub Pages y aceptar conexiones sÃ³lo desde el entorno
autorizado. El navegador no debe recibir credenciales de API, claves SOCKS, tokens ni
URLs con secretos; la autenticaciÃ³n real, si se necesita, pertenece al servicio
privado y no se simula en esta PWA.

No existe actualmente backend, API, proxy SOCKS ni autenticaciÃ³n implementada. Los
orÃ­genes de API/SOCKS son dependencias operativas futuras o locales, no contratos que
la aplicaciÃ³n pÃºblica pueda asumir.

## 11. Reglas para agentes y colaboradores

- Mantener `assets/` y sus rutas; no reemplazar recursos de marca sin autorizaciÃ³n.
- Tratar `index.html`, `styles.css`, `app.js`, `manifest.webmanifest` y `sw.js` como
  runtime: los cambios en ellos requieren una tarea explÃ­cita de producto.
- No aÃ±adir secretos, credenciales, cookies, tokens, archivos `.env` ni datos de
  usuarios al repositorio o al artefacto pÃºblico.
- No crear autenticaciÃ³n ficticia, backend, proxy o llamadas a API para aparentar una
  integraciÃ³n inexistente.
- Revisar cambios de workflow y permisos; el deploy sÃ³lo necesita lectura del cÃ³digo,
  escritura de Pages y el token OIDC de despliegue.

