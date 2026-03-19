param(
  [string]$ElkRepoUrl = "https://github.com/eclipse-elk/elk",
  [string]$ElkDir = (Join-Path $PSScriptRoot "..\\vendor\\elk-java"),
  [string]$ElkRev = "v0.11.0"
)

$ErrorActionPreference = "Stop"

if (!(Test-Path $ElkDir)) {
  New-Item -ItemType Directory -Force -Path $ElkDir | Out-Null
}

if (!(Test-Path (Join-Path $ElkDir ".git"))) {
  Write-Host "Cloning ELK Java into $ElkDir"
  git clone $ElkRepoUrl $ElkDir
}

Push-Location $ElkDir
try {
  Write-Host "Fetching + checking out ELK revision $ElkRev"
  git fetch --tags --force
  git checkout $ElkRev
} finally {
  Pop-Location
}

Write-Host "ELK Java repo ready at $ElkDir"

