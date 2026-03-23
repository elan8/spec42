param(
  [Parameter(Mandatory = $true)][string]$InputJson,
  [string]$JdkHome = "C:\Dev\jdk-26",
  [string]$ElkDir = ""
)

$ErrorActionPreference = "Stop"

if (!(Test-Path $InputJson)) { throw "InputJson not found: $InputJson" }
if (!(Test-Path $JdkHome)) { throw "JDK not found: $JdkHome" }

$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path

$env:JAVA_HOME = $JdkHome
$env:PATH = (Join-Path $JdkHome "bin") + ";" + $env:PATH

$root = Resolve-Path (Join-Path $scriptRoot "..")
$moduleDir = Join-Path (Join-Path $root "java") "elk-json-runner"
$pom = Join-Path $moduleDir "pom.xml"
$targetDir = Join-Path $root "target"
$mavenRepoDir = Join-Path $targetDir "m2"
$runToken = [guid]::NewGuid().ToString("N")
$mavenSettings = Join-Path $targetDir ("maven-settings-" + $runToken + ".xml")

New-Item -ItemType Directory -Force -Path $targetDir | Out-Null
New-Item -ItemType Directory -Force -Path $mavenRepoDir | Out-Null
@"
<settings>
  <localRepository>$($mavenRepoDir -replace '\\','/')</localRepository>
</settings>
"@ | Set-Content -Encoding UTF8 -Path $mavenSettings

if (!(Get-Command "mvn" -ErrorAction SilentlyContinue)) {
  [Console]::Error.WriteLine("Maven not found; bootstrapping...")
  $mvnCmd = & (Join-Path $scriptRoot "bootstrap-maven.ps1")
} else {
  $mvnCmd = "mvn"
}

# Keep stdout reserved for runner JSON; script diagnostics go to stderr.
[Console]::Error.WriteLine("Running Java runner via Maven exec:java...")
& $mvnCmd -q --batch-mode -f $pom `
  "-s" $mavenSettings `
  compile `
  "exec:java" `
  "-Dexec.args=$InputJson" `
  "-DskipTests=true" `
  "-e"
