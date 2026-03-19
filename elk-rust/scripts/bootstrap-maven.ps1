param(
  [string]$MavenVersion = "3.9.9",
  [string]$InstallDir = (Join-Path $PSScriptRoot "..\\vendor\\apache-maven")
)

$ErrorActionPreference = "Stop"

$mavenHome = Join-Path $InstallDir ("apache-maven-" + $MavenVersion)
$mvnCmd = Join-Path $mavenHome "bin\\mvn.cmd"

if (Test-Path $mvnCmd) {
  Write-Host "Maven already present at $mavenHome"
  Write-Output $mvnCmd
  exit 0
}

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

$zipName = "apache-maven-$MavenVersion-bin.zip"
$zipPath = Join-Path $InstallDir $zipName
$url = "https://archive.apache.org/dist/maven/maven-3/$MavenVersion/binaries/$zipName"

Write-Host "Downloading Maven $MavenVersion..."
Invoke-WebRequest -Uri $url -OutFile $zipPath

Write-Host "Extracting Maven..."
Expand-Archive -Path $zipPath -DestinationPath $InstallDir -Force

if (!(Test-Path $mvnCmd)) {
  throw "Maven download/extract failed; expected $mvnCmd"
}

Write-Host "Maven installed at $mavenHome"
Write-Output $mvnCmd

