param([string]$Msi=(Join-Path $PSScriptRoot '..\GnxNode.msi'))
if (!(Test-Path $Msi)) { throw "Missing MSI artifact: $Msi (build requires WiX Toolset 5 and dotnet SDK)." }; Write-Output "Verified MSI: $Msi"
