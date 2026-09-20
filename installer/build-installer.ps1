param(
  [string]$AppUrl = "http://localhost:8080/",
  [string]$Target = "x86_64-pc-windows-msvc"
)

$ErrorActionPreference = "Stop"
$root = $PSScriptRoot
$service = Join-Path $root "service"
$publish = Join-Path $service "publish"
$cargo = (Get-Command cargo -ErrorAction SilentlyContinue).Source
if (-not $cargo) { throw "No se encontró Cargo. Instala Rust y agrega cargo al PATH." }
$dotnet = (Get-Command dotnet -ErrorAction SilentlyContinue).Source
if (-not $dotnet -and (Test-Path "C:\Program Files\dotnet\dotnet.exe")) {
  $dotnet = "C:\Program Files\dotnet\dotnet.exe"
}
if (-not $dotnet) { throw "No se encontró dotnet para compilar WiX. Instala el SDK requerido por WiX." }

& $cargo build --manifest-path (Join-Path $service "Cargo.toml") --release --target $Target
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Remove-Item $publish -Recurse -Force -ErrorAction SilentlyContinue
New-Item $publish -ItemType Directory | Out-Null
Copy-Item (Join-Path $service "target/$Target/release/gnx-mesh-monitor.exe") (Join-Path $publish "gnx-mesh-monitor.exe")
Copy-Item (Join-Path $service "provision-wsl.ps1") (Join-Path $publish "provision-wsl.ps1")
Copy-Item (Join-Path $service "linux") (Join-Path $publish "linux") -Recurse
$configPath = Join-Path $publish "appsettings.json"
$config = Get-Content (Join-Path $service "appsettings.json") -Raw | ConvertFrom-Json
$config.Monitor.AppUrl = $AppUrl
$utf8NoBom = New-Object System.Text.UTF8Encoding($false)
[System.IO.File]::WriteAllText($configPath, ($config | ConvertTo-Json -Depth 8), $utf8NoBom)

& $dotnet build (Join-Path $root "GnxApp.wixproj") `
  -p:AppUrl=$AppUrl -p:IncludeService=1 -p:InstallScope=perMachine `
  -p:ServicePayloadDir=$publish
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "MSI generado con el servicio Rust, el reconciliador WSL y el Quadlet de Podman. Ejecuta como administrador para instalarlo; el reinicio del host puede programarse tras la instalación."
