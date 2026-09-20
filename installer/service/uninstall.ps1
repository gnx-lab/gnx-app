[CmdletBinding()]
param([string]$ServiceName = 'GnxAppMonitor')
$principal = [Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()
if (-not $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) { throw 'Run this script from an elevated PowerShell prompt.' }
$service = Get-Service -Name $ServiceName -ErrorAction SilentlyContinue
if ($service) { Stop-Service $ServiceName -Force -ErrorAction SilentlyContinue; sc.exe delete $ServiceName | Out-Null; Write-Host "Removed $ServiceName." }
else { Write-Host "$ServiceName is not installed." }
