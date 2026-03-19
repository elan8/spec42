param(
  [string]$JdkHome = "C:\Dev\jdk-26",
  [string]$ElkRev = "v0.11.0",
  [switch]$EnsureVendorElk,
  [switch]$BuildVendorElk
)

$ErrorActionPreference = "Stop"
$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path

if ($EnsureVendorElk) {
  & (Join-Path $scriptRoot "vendor-elk-java.ps1") -ElkRev $ElkRev
}

if ($BuildVendorElk) {
  & (Join-Path $scriptRoot "build-elk-java.ps1") -JdkHome $JdkHome
}

Write-Host "== Running Java parity test =="
Write-Host "Command: cargo test -p elk-testkit --test java_parity"

Push-Location (Resolve-Path (Join-Path $scriptRoot ".."))
try {
  cargo test -p elk-testkit --test java_parity
  if ($LASTEXITCODE -ne 0) {
    throw "Parity test failed with exit code $LASTEXITCODE"
  }
} finally {
  Pop-Location
}

