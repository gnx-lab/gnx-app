[CmdletBinding()]
param(
    [string]$ServiceRoot = (Join-Path $PSScriptRoot 'publish'),
    [string]$ServiceName = 'GnxAppMonitor',
    [string]$AppUrl,
    [string]$ManifestPath
)

$principal = [Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()
if (-not $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) { throw 'Run this script from an elevated PowerShell prompt.' }
$exe = Join-Path $ServiceRoot 'GnxAppMonitor.exe'
if (-not (Test-Path -LiteralPath $exe)) { throw "Service executable not found at $exe. Publish the project first." }
$configPath = Join-Path $ServiceRoot 'appsettings.json'
if ($AppUrl -or $ManifestPath) {
    $config = if (Test-Path -LiteralPath $configPath) { Get-Content -Raw $configPath | ConvertFrom-Json } else { [pscustomobject]@{ Monitor = [pscustomobject]@{} } }
    if (-not $config.Monitor) { $config | Add-Member NoteProperty Monitor ([pscustomobject]@{}) }
    if ($AppUrl) { $config.Monitor | Add-Member -Force NoteProperty AppUrl $AppUrl }
    if ($ManifestPath) { $config.Monitor | Add-Member -Force NoteProperty ManifestPath $ManifestPath }
    $config | ConvertTo-Json -Depth 8 | Set-Content -Encoding UTF8 $configPath
}
if (Get-Service -Name $ServiceName -ErrorAction SilentlyContinue) { Stop-Service $ServiceName -ErrorAction SilentlyContinue; sc.exe delete $ServiceName | Out-Null; Start-Sleep -Seconds 1 }
New-Service -Name $ServiceName -DisplayName 'GnX App Monitor' -Description 'Checks GnX App availability and an optional local install manifest.' -BinaryPathName "`"$exe`"" -StartupType Automatic
sc.exe failure $ServiceName reset= 86400 actions= restart/5000/restart/30000/none/0 | Out-Null
Start-Service $ServiceName
Write-Host "Installed and started $ServiceName."
