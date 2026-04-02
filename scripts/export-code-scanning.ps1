param(
  [string]$Repo = "",
  [string]$OutDir = ".\code-scanning-export"
)

$ErrorActionPreference = "Stop"

function Require-Command([string]$Name) {
  if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
    throw "Missing required command: $Name"
  }
}

Require-Command "gh"

# Validate auth early so we fail with a clear message.
try {
  gh auth status | Out-Null
} catch {
  throw "GitHub CLI is not authenticated. Run 'gh auth login' and retry."
}

# Resolve repo if not provided (works when run in a cloned repo with gh configured)
if ([string]::IsNullOrWhiteSpace($Repo)) {
  $Repo = (gh repo view --json nameWithOwner -q .nameWithOwner).Trim()
}
if ([string]::IsNullOrWhiteSpace($Repo)) {
  throw "Could not resolve repo. Pass -Repo owner/name"
}

$resolvedOutDir = if ([System.IO.Path]::IsPathRooted($OutDir)) {
  $OutDir
} else {
  Join-Path $PSScriptRoot $OutDir
}
New-Item -ItemType Directory -Force -Path $resolvedOutDir | Out-Null

$ts = Get-Date -Format "yyyyMMdd-HHmmss"

$allAlertsPath = Join-Path $resolvedOutDir "code-scanning-alerts-$ts.json"

Write-Host "Exporting GitHub code scanning alerts for $Repo -> $allAlertsPath"
$rawJson = gh api "repos/$Repo/code-scanning/alerts" --paginate --slurp

try {
  $pagedAlerts = $rawJson | ConvertFrom-Json
} catch {
  throw "Export failed: GitHub API output is not valid JSON. Ensure '--paginate --slurp' is used and GitHub CLI auth is valid (gh auth login)."
}

$alerts = @()
if ($null -ne $pagedAlerts) {
  foreach ($page in @($pagedAlerts)) {
    if ($page -is [System.Array]) {
      $alerts += @($page)
    } elseif ($null -ne $page) {
      $alerts += $page
    }
  }
}

$alertsJson = $alerts | ConvertTo-Json -Depth 100
[System.IO.File]::WriteAllText($allAlertsPath, $alertsJson, [System.Text.Encoding]::UTF8)
if (-not (Test-Path -LiteralPath $allAlertsPath)) {
  throw "Export failed: alert JSON file was not created."
}
$fileInfo = Get-Item -LiteralPath $allAlertsPath
if ($fileInfo.Length -le 0) {
  throw "Export failed: alert JSON file is empty."
}

Write-Host "Done."
Write-Host "Code scanning JSON: $allAlertsPath"
Write-Host "Count -> scanning: $(@($alerts).Count)"
