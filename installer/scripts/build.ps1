param([string]$Configuration="release")
$ErrorActionPreference="Stop"; $root=Resolve-Path "$PSScriptRoot\..\.."
Push-Location "$root\gnx-node"; cargo build --release; Pop-Location
if (Get-Command dotnet -ErrorAction SilentlyContinue) { Push-Location "$root\installer"; dotnet build GnxNode.wixproj; if ($?) { dotnet build bundle\Bundle.wixproj -p:MsiPath="$root\installer\bin\GnxNode.msi" }; Pop-Location } else { Write-Error "WiX build unavailable: dotnet SDK not found" }
