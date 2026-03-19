param(
  [string]$JdkHome = "C:\Dev\jdk-26",
  [string]$ElkDir = (Join-Path $PSScriptRoot "..\\vendor\\elk-java"),
  [string]$MavenRepoLocal = (Join-Path $ElkDir "build\\mvnrepo")
)

$ErrorActionPreference = "Stop"

if (!(Test-Path $JdkHome)) {
  throw "JDK not found at JdkHome=$JdkHome"
}
if (!(Test-Path $ElkDir)) {
  throw "ELK repo not found at ElkDir=$ElkDir. Run scripts/vendor-elk-java.ps1 first."
}

$mvn = "mvn"
$javaHome = $JdkHome

function Get-JavaMajorVersion([string]$JavaHome) {
  $java = Join-Path $JavaHome "bin\\java.exe"
  if (!(Test-Path $java)) { return $null }
  $oldEap = $ErrorActionPreference
  $ErrorActionPreference = "SilentlyContinue"
  try {
    $out = (& $java "-version" 2>&1 | Out-String)
  } finally {
    $ErrorActionPreference = $oldEap
  }
  if ($out -match 'version\s+"(\d+)\.') { return [int]$Matches[1] }
  if ($out -match 'version\s+"(\d+)"') { return [int]$Matches[1] }
  return $null
}

$major = Get-JavaMajorVersion $javaHome
if ($major -and $major -ge 22) {
  Write-Host "Detected JDK $major at $javaHome; ELK build requires Java 17-compatible toolchain. Bootstrapping JDK 17."
  $javaHome = & (Join-Path $PSScriptRoot "bootstrap-jdk17.ps1")
}

Push-Location (Join-Path $ElkDir "build")
try {
  Write-Host "Building ELK Java from $(Get-Location)"
  Write-Host "JAVA_HOME=$javaHome"
  Write-Host "Local Maven repo: $MavenRepoLocal"

  $env:JAVA_HOME = $javaHome
  $env:PATH = (Join-Path $javaHome "bin") + ";" + $env:PATH
  # Tycho may consume large Eclipse p2 metadata; JDK 26 XML limits can be too strict.
  # Override JAXP limits for this build invocation.
  $xmlLimits = @(
    "-Djdk.xml.maxGeneralEntitySizeLimit=0",
    "-Djdk.xml.maxParameterEntitySizeLimit=0",
    "-Djdk.xml.totalEntitySizeLimit=0",
    "-Djdk.xml.entityExpansionLimit=0"
  ) -join " "
  $env:MAVEN_OPTS = ($env:MAVEN_OPTS + " " + $xmlLimits).Trim()

  if (!(Get-Command $mvn -ErrorAction SilentlyContinue)) {
    $bootstrap = Join-Path $PSScriptRoot "bootstrap-maven.ps1"
    $mvn = & $bootstrap
  }

  & $mvn `
    "--define" "elk.metadata.documentation.outputPath=$ElkDir\docs" `
    "-Dmaven.repo.local=$MavenRepoLocal" `
    "clean" `
    "package"

  if ($LASTEXITCODE -ne 0) {
    throw "Maven build failed with exit code $LASTEXITCODE"
  }
} finally {
  Pop-Location
}

Write-Host "ELK Java build complete."

