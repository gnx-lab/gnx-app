[CmdletBinding()]
param(
  [ValidateSet('Plan', 'Reconcile')]
  [string]$Mode = 'Reconcile',
  [string]$StatePath = 'C:\ProgramData\GnX App Monitor\provisioning.json',
  [string]$ServiceUser = 'gnxsvc',
  [string]$Distribution = 'Ubuntu-24.04',
  [string]$LinuxInstallPath = '',
  [switch]$AutoReboot
)

$ErrorActionPreference = 'Stop'
$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
if (-not $LinuxInstallPath) { $LinuxInstallPath = Join-Path $scriptRoot 'linux\install-linux-service.sh' }
$stateDirectory = Split-Path -Parent $StatePath
$credentialPath = Join-Path $stateDirectory 'gnxsvc.credential.xml'
$bootId = ''

function New-DefaultState {
  [ordered]@{
    schemaVersion = 1
    phase = 'STARTING'
    code = 'STARTING'
    message = 'Provisioning has not run yet.'
    serviceUser = $ServiceUser
    distribution = $Distribution
    rebootPending = $false
    rebootBootId = $null
    attempts = 0
    lastAttempt = $null
    lastError = $null
    virtualization = $null
    linuxService = 'unknown'
  }
}

function Read-State {
  if (-not (Test-Path -LiteralPath $StatePath)) { return New-DefaultState }
  try {
    $raw = Get-Content -LiteralPath $StatePath -Raw
    $value = $raw | ConvertFrom-Json
    $result = New-DefaultState
    foreach ($property in $value.PSObject.Properties) { $result[$property.Name] = $property.Value }
    return $result
  } catch {
    $result = New-DefaultState
    $result.phase = 'RECOVERY_REQUIRED'
    $result.code = 'STATE_CORRUPT'
    $result.message = 'Provisioning state is not valid JSON.'
    $result.lastError = $_.Exception.Message
    return $result
  }
}

function Write-State([hashtable]$Value) {
  New-Item -ItemType Directory -Path $stateDirectory -Force | Out-Null
  $temporary = "$StatePath.$PID.tmp"
  $json = $Value | ConvertTo-Json -Depth 8
  $utf8NoBom = New-Object System.Text.UTF8Encoding($false)
  [System.IO.File]::WriteAllText($temporary, $json, $utf8NoBom)
  Move-Item -LiteralPath $temporary -Destination $StatePath -Force
}

function Set-State {
  param(
    [hashtable]$Value,
    [string]$Phase,
    [string]$Code,
    [string]$Message,
    [hashtable]$Extra = @{}
  )
  $Value.phase = $Phase
  $Value.code = $Code
  $Value.message = $Message
  $Value.lastAttempt = (Get-Date).ToUniversalTime().ToString('o')
  foreach ($key in $Extra.Keys) { $Value[$key] = $Extra[$key] }
  Write-State $Value
}

function Stop-Blocked([hashtable]$Value, [string]$Code, [string]$Message, [hashtable]$Extra = @{}) {
  Set-State $Value 'BLOCKED' $Code $Message $Extra
  exit 20
}

function Test-Administrator {
  $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
  $principal = New-Object Security.Principal.WindowsPrincipal($identity)
  return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Get-BootId {
  try {
    $os = Get-CimInstance Win32_OperatingSystem
    return ([Management.ManagementDateTimeConverter]::ToDateTime($os.LastBootUpTime)).ToUniversalTime().ToString('o')
  } catch { return 'unknown' }
}

function New-RandomPassword {
  $bytes = New-Object byte[] 24
  $generator = [Security.Cryptography.RandomNumberGenerator]::Create()
  $generator.GetBytes($bytes)
  $generator.Dispose()
  return ([Convert]::ToBase64String($bytes) + 'aA1!')
}

function Ensure-DedicatedUser([hashtable]$Value) {
  try {
    $existing = Get-LocalUser -Name $ServiceUser -ErrorAction SilentlyContinue
  } catch {
    Stop-Blocked $Value 'LOCAL_ACCOUNTS_UNAVAILABLE' 'The LocalAccounts PowerShell module is unavailable.'
  }

  if ($existing -and $existing.Description -notlike 'GnX managed WSL account*') {
    Stop-Blocked $Value 'DEDICATED_USER_CONFLICT' "The local user '$ServiceUser' already exists and is not owned by GnX."
  }

  $password = $null
  if (-not $existing) {
    $password = New-RandomPassword
    $secure = ConvertTo-SecureString $password -AsPlainText -Force
    New-LocalUser -Name $ServiceUser -Password $secure -Description 'GnX managed WSL account' `
      -AccountNeverExpires -PasswordNeverExpires -UserMayNotChangePassword | Out-Null
  } elseif (-not (Test-Path -LiteralPath $credentialPath)) {
    $password = New-RandomPassword
    $secure = ConvertTo-SecureString $password -AsPlainText -Force
    Set-LocalUser -Name $ServiceUser -Password $secure
  }

  if ($password) {
    $credential = New-Object System.Management.Automation.PSCredential($ServiceUser, (ConvertTo-SecureString $password -AsPlainText -Force))
    $credential | Export-Clixml -LiteralPath $credentialPath
    & icacls.exe $credentialPath /inheritance:r /grant:r 'SYSTEM:F' 'Administrators:F' | Out-Null
  }

  if (-not (Test-Path -LiteralPath $credentialPath)) {
    Stop-Blocked $Value 'DEDICATED_USER_CREDENTIAL_MISSING' 'The managed user exists but its protected credential is unavailable.'
  }
  $Value.serviceUser = $ServiceUser
  $Value.userReady = $true
}

function Invoke-WslAsDedicatedUser([string[]]$Arguments) {
  $credential = Import-Clixml -LiteralPath $credentialPath
  $profile = Join-Path $env:SystemDrive "Users\$ServiceUser"
  New-Item -ItemType Directory -Path $profile -Force | Out-Null
  $stdout = Join-Path $stateDirectory "wsl-$PID-out.txt"
  $stderr = Join-Path $stateDirectory "wsl-$PID-err.txt"
  try {
    $process = Start-Process -FilePath "$env:SystemRoot\System32\wsl.exe" -ArgumentList $Arguments `
      -Credential $credential -LoadUserProfile -WorkingDirectory $profile `
      -RedirectStandardOutput $stdout -RedirectStandardError $stderr -Wait -PassThru
    [pscustomobject]@{
      ExitCode = $process.ExitCode
      Stdout = if (Test-Path $stdout) { Get-Content $stdout -Raw } else { '' }
      Stderr = if (Test-Path $stderr) { Get-Content $stderr -Raw } else { '' }
    }
  } finally {
    Remove-Item $stdout, $stderr -Force -ErrorAction SilentlyContinue
  }
}

function Get-Virtualization {
  $computer = Get-CimInstance Win32_ComputerSystem
  $processor = Get-CimInstance Win32_Processor | Select-Object -First 1
  [ordered]@{
    firmwareEnabled = $processor.VirtualizationFirmwareEnabled
    hypervisorPresent = $computer.HypervisorPresent
  }
}

function Enable-HostVirtualization([hashtable]$Value) {
  $virtualization = Get-Virtualization
  $Value.virtualization = $virtualization
  if ($virtualization.firmwareEnabled -eq $false) {
    Stop-Blocked $Value 'VIRTUALIZATION_DISABLED' 'Firmware virtualization is disabled. Enable Intel VT-x/AMD-V in BIOS/UEFI and retry.' @{ virtualization = $virtualization }
  }
  if ($Mode -eq 'Plan') { return $false }

  $requiresReboot = $false
  foreach ($featureName in @('Microsoft-Windows-Subsystem-Linux', 'VirtualMachinePlatform')) {
    $feature = Get-WindowsOptionalFeature -Online -FeatureName $featureName
    if ($feature.State -ne 'Enabled') {
      if ($feature.State -eq 'EnablePending') {
        $requiresReboot = $true
        continue
      }
      Enable-WindowsOptionalFeature -Online -FeatureName $featureName -All -NoRestart | Out-Null
      $requiresReboot = $true
    }
  }

  & bcdedit.exe /set hypervisorlaunchtype auto | Out-Null
  if ($LASTEXITCODE -ne 0) { Stop-Blocked $Value 'HYPERVISOR_BOOT_CONFIGURATION_FAILED' 'Could not set hypervisorlaunchtype=auto.' }
  return $requiresReboot
}

function Schedule-HostReboot([hashtable]$Value, [string]$Reason) {
  $currentBoot = Get-BootId
  if ($Value.rebootPending -and $Value.rebootBootId -eq $currentBoot) {
    Set-State $Value 'REBOOT_WAITING' 'REBOOT_PENDING' 'The host reboot is already scheduled.' @{ rebootPending = $true; rebootBootId = $currentBoot }
    exit 10
  }
  $Value.rebootPending = $true
  $Value.rebootBootId = $currentBoot
  Set-State $Value 'REBOOT_SCHEDULED' 'REBOOT_REQUIRED' $Reason @{ rebootPending = $true; rebootBootId = $currentBoot }
  if ($Mode -eq 'Plan' -or -not $AutoReboot) { exit 10 }
  & shutdown.exe /r /t 60 /c 'GnX WSL provisioning requires a host restart. The GnX service will resume automatically.' /d p:4:1 | Out-Null
  if ($LASTEXITCODE -ne 0) { Stop-Blocked $Value 'HOST_REBOOT_SCHEDULE_FAILED' 'Windows rejected the scheduled reboot.' }
  exit 10
}

function Ensure-LinuxService([hashtable]$Value) {
  $listed = Invoke-WslAsDedicatedUser @('-l', '-q')
  if ($listed.ExitCode -ne 0 -or $listed.Stdout -notmatch [regex]::Escape($Distribution)) {
    $install = Invoke-WslAsDedicatedUser @('--install', '--distribution', $Distribution, '--no-launch')
    if ($install.ExitCode -notin @(0, 3010)) {
      Stop-Blocked $Value 'WSL_INSTALL_FAILED' ("Ubuntu installation failed: " + $install.Stderr.Trim())
    }
    if ($install.ExitCode -eq 3010 -or $install.Stdout -match '(?i)restart|reboot') {
      Schedule-HostReboot $Value 'Ubuntu installation requested a host restart.'
    }
    $listed = Invoke-WslAsDedicatedUser @('-l', '-q')
    if ($listed.Stdout -notmatch [regex]::Escape($Distribution)) {
      Stop-Blocked $Value 'WSL_DISTRIBUTION_NOT_READY' "The distribution '$Distribution' is not registered for the managed user."
    }
  }

  if (-not (Test-Path -LiteralPath $LinuxInstallPath)) {
    Stop-Blocked $Value 'LINUX_INSTALL_SCRIPT_MISSING' "Missing Linux service installer: $LinuxInstallPath"
  }
  $encoded = [Convert]::ToBase64String([Text.Encoding]::UTF8.GetBytes((Get-Content $LinuxInstallPath -Raw)))
  # The command contains no literal spaces, so Start-Process can pass it as one WSL argument.
  $linuxCommand = "echo`$IFS$encoded|base64`$IFS-d|bash"
  $installLinux = Invoke-WslAsDedicatedUser @('-d', $Distribution, '-u', 'root', '--', 'bash', '-c', $linuxCommand)
  if ($installLinux.ExitCode -eq 42) {
    Invoke-WslAsDedicatedUser @('--shutdown') | Out-Null
    Set-State $Value 'WSL_RESTART_REQUIRED' 'WSL_RESTART_REQUIRED' 'WSL systemd configuration changed; the distribution will be resumed on the next reconcile.' @{ linuxService = 'restart_pending' }
    exit 10
  }
  if ($installLinux.ExitCode -ne 0) {
    Stop-Blocked $Value 'LINUX_SERVICE_INSTALL_FAILED' ("Linux service installation failed: " + $installLinux.Stderr.Trim())
  }

  $check = Invoke-WslAsDedicatedUser @('-d', $Distribution, '-u', 'root', '--', 'systemctl', 'is-active', '--quiet', 'gnx-linux.service')
  if ($check.ExitCode -ne 0) {
    Invoke-WslAsDedicatedUser @('-d', $Distribution, '-u', 'root', '--', 'systemctl', 'restart', 'gnx-linux.service') | Out-Null
    $check = Invoke-WslAsDedicatedUser @('-d', $Distribution, '-u', 'root', '--', 'systemctl', 'is-active', '--quiet', 'gnx-linux.service')
  }
  if ($check.ExitCode -ne 0) {
    Stop-Blocked $Value 'LINUX_SERVICE_DOWN' 'The Linux service is not active after reconciliation.' @{ linuxService = 'down' }
  }
  $Value.linuxService = 'running'
}

$state = Read-State
$state.attempts = [int]$state.attempts + 1
$bootId = Get-BootId
if ($state.rebootPending -and $state.rebootBootId -ne $bootId) {
  $state.rebootPending = $false
  $state.rebootBootId = $null
}

if (-not (Test-Administrator)) { Stop-Blocked $state 'ADMINISTRATOR_REQUIRED' 'The provisioning reconciler must run elevated.' }
if ($Mode -eq 'Plan') {
  $virtualization = Get-Virtualization
  $state.virtualization = $virtualization
  $state.phase = 'PLAN'
  $state.code = if ($virtualization.firmwareEnabled -eq $false) { 'VIRTUALIZATION_DISABLED' } else { 'PLAN_READY' }
  $state.message = if ($virtualization.firmwareEnabled -eq $false) { 'Enable virtualization in BIOS/UEFI before applying.' } else { 'Plan complete; no host changes were made.' }
  Write-State $state
  exit 0
}

if ($state.rebootPending -and $state.rebootBootId -eq $bootId) {
  Set-State $state 'REBOOT_WAITING' 'REBOOT_PENDING' 'Waiting for the scheduled host restart.' @{ rebootPending = $true; rebootBootId = $bootId }
  exit 10
}

$requiresReboot = Enable-HostVirtualization $state
Ensure-DedicatedUser $state
if ($requiresReboot) { Schedule-HostReboot $state 'Windows WSL and VirtualMachinePlatform features require a host restart.' }
Ensure-LinuxService $state
Set-State $state 'READY' 'PROVISIONED' 'Dedicated user, WSL Ubuntu and the resilient Linux service are ready.' @{ rebootPending = $false; rebootBootId = $null; linuxService = 'running' }
exit 0
