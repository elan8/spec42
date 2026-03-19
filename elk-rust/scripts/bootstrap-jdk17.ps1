param(
  [string]$InstallDir = (Join-Path $PSScriptRoot "..\\vendor\\jdks"),
  [string]$FeatureVersion = "17"
)

$ErrorActionPreference = "Stop"

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

$jdkDir = Join-Path $InstallDir "jdk-17"
$javaExe = Join-Path $jdkDir "bin\\java.exe"

if (Test-Path $javaExe) {
  Write-Host "JDK 17 already present at $jdkDir"
  Write-Output $jdkDir
  exit 0
}

# Adoptium binary API (returns a zip)
$url = "https://api.adoptium.net/v3/binary/latest/$FeatureVersion/ga/windows/x64/jdk/hotspot/normal/eclipse"
$zipPath = Join-Path $InstallDir "jdk-17.zip"

Write-Host "Downloading JDK 17..."
Invoke-WebRequest -Uri $url -OutFile $zipPath

Write-Host "Extracting JDK 17..."
Expand-Archive -Path $zipPath -DestinationPath $InstallDir -Force

# The zip typically extracts to a versioned folder like jdk-17.0.x+...
$extracted = Get-ChildItem -Directory -Path $InstallDir | Where-Object { $_.Name -like "jdk-17*" } | Select-Object -First 1
if (-not $extracted) {
  throw "Could not find extracted jdk-17* directory under $InstallDir"
}

if (Test-Path $jdkDir) { Remove-Item -Recurse -Force $jdkDir }
Move-Item -Path $extracted.FullName -Destination $jdkDir

if (!(Test-Path $javaExe)) {
  throw "JDK 17 install failed; expected $javaExe"
}

Write-Host "JDK 17 installed at $jdkDir"
Write-Output $jdkDir

