param(
  [string]$AppUrl = "https://mayas-alas.github.io/gnx-app",
  [string]$Runtime = "win-x64"
)

$ErrorActionPreference = "Stop"
$root = $PSScriptRoot
$publish = Join-Path $root "service/publish"

dotnet publish (Join-Path $root "service/GnxApp.Monitor.csproj") `
  --configuration Release --runtime $Runtime --self-contained true --output $publish

dotnet build (Join-Path $root "GnxApp.wixproj") `
  -p:AppUrl=$AppUrl -p:IncludeService=1 -p:InstallScope=perMachine `
  -p:ServicePayloadDir=$publish

Write-Host "MSI generado con el servicio Windows incluido. Ejecuta como administrador para instalarlo."
