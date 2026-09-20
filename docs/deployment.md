# Despliegue de GnX App

## Publicación pública: GitHub Pages

La rama `main` publica automáticamente la PWA mediante
`.github/workflows/deploy-pages.yml`. El workflow crea un artefacto mínimo con el
shell web y `assets/`; no expone `docs/`, el proyecto WiX ni archivos de compilación.
El sitio queda disponible en la URL de GitHub Pages del repositorio (por ejemplo,
`https://<organización>.github.io/<repositorio>/`). La ruta exacta la asigna GitHub.

Configuración inicial, realizada una sola vez por un administrador del repositorio:

1. En **Settings > Pages**, elegir **GitHub Actions** como origen de build/deploy.
2. Confirmar que la rama de publicación sea `main` y que el workflow tenga permisos
   de Pages habilitados.
3. Hacer push a `main` o ejecutar el workflow manualmente desde **Actions**.
4. Abrir la URL indicada por el environment `github-pages` y revisar instalación,
   manifest y funcionamiento offline en un navegador compatible.

El sitio público sólo debe contener datos y código que puedan ser públicos. GitHub
Pages no proporciona backend, almacenamiento privado, autenticación ni protección de
credenciales.

## Desarrollo local

El Service Worker y la instalación PWA requieren HTTPS o `localhost`; no abrir
`index.html` con `file://`. Desde la raíz del repositorio se puede usar, por ejemplo:

```powershell
python -m http.server 8080
```

Después abrir `http://localhost:8080/`. También se puede usar cualquier servidor
estático equivalente que preserve las rutas relativas. Para comprobar el instalador
Windows (que sólo crea accesos a una URL), usar:

```powershell
dotnet build installer\GnxApp.wixproj -p:AppUrl=https://<sitio-publicado>
```

El MSI no empaqueta `localhost`, el servidor ni secretos.

## Límite de API, SOCKS y autenticación

Actualmente no hay backend, API, proxy SOCKS ni autenticación implementados. Si una
instancia local necesita una API o una salida SOCKS, esos procesos deben vivir en el
entorno privado de desarrollo y documentar sus puertos/configuración fuera del sitio
público. Nunca incrustar claves, contraseñas, tokens o credenciales en JavaScript,
manifest, Service Worker, workflow o archivos publicados.

GitHub Pages no puede ocultar una URL ni validar permisos. La autenticación real debe
resolverse en el servicio privado (o en un proveedor gestionado) antes de exponer
datos; no se debe añadir una pantalla de login que sólo simule seguridad. Si se añade
una integración futura, definir explícitamente el propietario, origen permitido,
transporte HTTPS y manejo de secretos antes de modificar el runtime.

## Operación segura para agentes

- Cambiar sólo el área solicitada y conservar los recursos de `assets/`.
- No subir secretos ni artefactos locales (`installer/bin`, `installer/obj`, MSI o
  símbolos); están ignorados por `.gitignore`.
- Revisar el diff antes de publicar y comprobar que el workflow sólo copie el shell
  público.
- No convertir GitHub Pages en sustituto de una API, autenticación o proxy privado.
- Reportar cualquier credencial encontrada y rotarla por el canal operativo adecuado;
  nunca copiarla al repositorio ni a los logs de Actions.
