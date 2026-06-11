# Updates the examples git submodule to the latest commit on the remote default branch.
# remote's default branch.
#
# Run from repo root:  powershell -File scripts/update-content-submodules.ps1
# Or from anywhere:    powershell -File C:\path\to\spec42\scripts\update-content-submodules.ps1

param(
    [switch]$SkipInit
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$env:GIT_TERMINAL_PROMPT = '0'

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
Set-Location $repoRoot

$submodules = @('examples')

function Get-OriginDefaultBranch {
    param([string]$SubmodulePath)
    $show = git -C $SubmodulePath remote show origin 2>&1 | Out-String
    if ($show -match 'HEAD branch:\s*(\S+)') {
        return $Matches[1]
    }
    return $null
}

if (-not (Test-Path (Join-Path $repoRoot '.gitmodules'))) {
    throw "No .gitmodules at repo root: $repoRoot"
}

git submodule sync @submodules
if (-not $SkipInit) {
    git submodule update --init @submodules
}

foreach ($name in $submodules) {
    $path = Join-Path $repoRoot $name
    if (-not (Test-Path $path)) {
        throw "Submodule directory missing: $path (try without -SkipInit)"
    }

    Write-Host "Updating submodule '$name' ..." -ForegroundColor Cyan
    git -C $path fetch origin --prune

    $branch = Get-OriginDefaultBranch -SubmodulePath $path
    if (-not $branch) {
        throw "Could not determine origin HEAD branch for '$name'. Fetch failed or remote has no default branch."
    }

    git -C $path checkout $branch
    git -C $path pull --ff-only origin $branch
}

Write-Host ''
Write-Host 'Submodule tips are now at the latest origin commits.' -ForegroundColor Green
Write-Host 'Superproject status (commit submodule pointers when ready):' -ForegroundColor Green
git status -sb
