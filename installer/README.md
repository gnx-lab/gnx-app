# Instalador de GnX App para Windows (WiX Toolset 5)

Este proyecto genera un MSI **solo con accesos directos** a la aplicación web publicada. No contiene un servidor, no empaqueta `localhost` y no convierte la aplicación web en una aplicación nativa. Los accesos de Inicio y escritorio abren la URL configurada en el navegador.

## Requisitos

- [.NET SDK](https://dotnet.microsoft.com/download) compatible con el SDK de WiX.
- WiX Toolset 5.x vía NuGet (`WixToolset.Sdk` y `WixToolset.Util.wixext`), restaurado por `dotnet`.

No se incluyen binarios, el instalador resultante ni dependencias copiadas en este repositorio.

## Compilar con la instalación publicada

Desde la raíz del proyecto:

```powershell
dotnet build installer\GnxApp.wixproj -p:AppUrl=https://app.gnx
```

`AppUrl` debe ser una URL HTTPS pública. Si no se especifica, el proyecto usa **https://app.gnx**. Para inspeccionar el MSI generado, ejecuta `dotnet build` y revisa `installer\bin\`.

Para otro entorno, sobrescribe `AppUrl` durante la compilación. La distribución debe usar siempre una URL HTTPS accesible desde los dispositivos de destino.

## Verificación y firma

La compilación local fue verificada con .NET SDK 8 y WiX Toolset 5 mediante:

```powershell
dotnet build installer\GnxApp.wixproj -p:AppUrl=https://app.gnx
```

El build genera el MSI correctamente, con 0 advertencias y 0 errores. El MSI generado no tiene firma digital; para distribuirlo se debe firmar con un certificado de firma de código válido.

## Servicio opcional de supervisión

`service/` contiene un Worker .NET 8 que puede registrarse como servicio de Windows. En cada intervalo configurable hace una solicitud HTTP(S) pública a `Monitor:AppUrl` (sin autenticación ni credenciales) y, si se configura `Monitor:ManifestPath`, calcula los hashes de los archivos declarados en un manifiesto JSON local. El manifiesto usa este formato:

```json
{"baseDirectory":".","files":{"index.html":"HASH_SHA256_EN_MAYUSCULAS"}}
```

El servicio registra resultados en el Event Log/host de Windows y detiene sus comprobaciones de forma cooperativa. Una URL no disponible o un hash incorrecto se registra como diagnóstico; el servicio no repara archivos, no autentica usuarios y no garantiza que un servidor remoto esté sano más allá de la respuesta HTTP. Si no existe el manifiesto configurado, se registra una advertencia y se continúa.

Para publicar y registrar el servicio, desde una consola PowerShell **elevada**:

```powershell
dotnet publish installer\service\GnxApp.Monitor.csproj -c Release -r win-x64 --self-contained false -o installer\service\publish
& installer\service\install.ps1 -ServiceRoot (Resolve-Path installer\service\publish) -AppUrl https://app.gnx
& installer\service\uninstall.ps1
```

El runtime .NET 8 debe estar instalado cuando se usa `--self-contained false`; una instalación self-contained requiere más espacio y debe publicarse explícitamente. Los scripts no descargan código, no crean credenciales y exigen privilegios de administrador.

### Integración WiX

La compilación predeterminada sigue generando solo los accesos directos y no requiere binarios del servicio. Después de publicar, se puede incluir el payload con un paquete por máquina:

```powershell
dotnet build installer\GnxApp.wixproj -p:AppUrl=https://app.gnx -p:IncludeService=1 -p:InstallScope=perMachine -p:ServicePayloadDir=$PWD\installer\service\publish
```

La referencia condicional de WiX instala el EXE y `appsettings.json`, registra `GnxAppMonitor` y lo elimina al desinstalar. No se incluyen EXE/MSI generados en el repositorio.

## Alcance y limitaciones

- El MSI crea un acceso en el menú Inicio y otro en el escritorio, ambos dirigidos a la aplicación web.
- Windows/WiX no puede garantizar el anclaje a la barra de tareas; el usuario debe hacerlo manualmente si lo desea.
- El MSI no instala un runtime, servidor local ni navegador.
- WiX/.NET no está necesariamente instalado en cada entorno. Si `dotnet` no está disponible, ejecuta la restauración y compilación en una máquina con el SDK; no se declara ningún MSI/EXE generado.

## Validación

```powershell
dotnet --info
dotnet build installer\GnxApp.wixproj -p:AppUrl=https://app.gnx
```

La validación realizada en el entorno de trabajo debe reportarse junto con el resultado exacto de esos comandos; no se debe sustituir por un artefacto inventado.
