param(
  [string]$AppUrl = "https://mayas-alas.github.io/gnx-app",
  [string]$Runtime = "win-x64"
)

$ErrorActionPreference = "Stop"
$root = $PSScriptRoot
$publish = Join-Path $root "service/publish"
$dotnet = (Get-Command dotnet -ErrorAction SilentlyContinue).Source
if (-not $dotnet -and (Test-Path "C:\Program Files\dotnet\dotnet.exe")) {
  $dotnet = "C:\Program Files\dotnet\dotnet.exe"
}
if (-not $dotnet) { throw "No se encontró el SDK de .NET. Instala .NET 8 o agrega dotnet al PATH." }

& $dotnet publish (Join-Path $root "service/GnxApp.Monitor.csproj") `
  --configuration Release --runtime $Runtime --self-contained true --output $publish

$configPath = Join-Path $publish "appsettings.json"
$config = Get-Content $configPath -Raw | ConvertFrom-Json
$config.Monitor.AppUrl = $AppUrl
$config | ConvertTo-Json -Depth 8 | Set-Content $configPath -Encoding utf8

& $dotnet build (Join-Path $root "GnxApp.wixproj") `
  -p:AppUrl=$AppUrl -p:IncludeService=1 -p:InstallScope=perMachine `
  -p:ServicePayloadDir=$publish

Write-Host "MSI generado con el servicio Windows incluido. Ejecuta como administrador para instalarlo."
