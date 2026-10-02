param(
    [int]$Requirements = 400,
    [int[]]$Connections = @(0, 25, 100, 400),
    [int]$Iterations = 3,
    [switch]$ReuseLibrary
)

# Report-only scaling probe through the canonical publication benchmark.
$ErrorActionPreference = 'Stop'
if ($Requirements -lt 2 -or $Iterations -lt 1 -or ($Connections | Where-Object { $_ -lt 0 })) {
    throw 'Requirements must be >= 2, iterations >= 1, and connection counts >= 0.'
}
$repoRoot = Split-Path $PSScriptRoot -Parent
$probeRoot = Join-Path $repoRoot ('target/connection-scaling-' + [guid]::NewGuid().ToString('N'))
$snapshotRoot = Join-Path $probeRoot 'tests/snapshots'
New-Item -ItemType Directory -Force $snapshotRoot | Out-Null
Copy-Item -LiteralPath (Join-Path $repoRoot 'tests/snapshots/sysml.library') -Destination $snapshotRoot -Recurse
Push-Location $repoRoot
try {
    & cargo build --offline --release -p spec42-semantic-benchmark
    if ($LASTEXITCODE -ne 0) { throw 'Benchmark build failed.' }
    $benchmark = Join-Path $repoRoot 'target/release/spec42-semantic-benchmark.exe'
    foreach ($style in @('plain', 'derivation')) {
        foreach ($count in $Connections) {
            $lines = [System.Collections.Generic.List[string]]::new()
            $lines.Add('package Scaling { private import RequirementDerivation::*;')
            for ($i = 0; $i -lt $Requirements; $i++) { $lines.Add("requirement r$i;") }
            for ($i = 0; $i -lt $count; $i++) {
                $original = $i % $Requirements
                $derived = ($i + 1) % $Requirements
                if ($style -eq 'derivation') {
                    $lines.Add("#derivation connection d$i { end #original ::> r$original; end #derive ::> r$derived; }")
                } else {
                    $lines.Add("connection c$i { end ::> r$original; end ::> r$derived; }")
                }
            }
            $lines.Add('}')
            $name = "$style-$count"
            $fixture = "# SOURCE`n~~~sysml`n" + ($lines -join "`n") + "`n~~~`n"
            [System.IO.File]::WriteAllText((Join-Path $snapshotRoot "$name.md"), $fixture)
            $reportPath = Join-Path $probeRoot "$name.json"
            $arguments = @('--repo-root', $probeRoot, '--filter', "$name.md", '--libraries', 'standard',
                '--iterations', $Iterations, '--output', $reportPath)
            if ($ReuseLibrary) { $arguments += '--reuse-library' }
            & $benchmark @arguments
            if ($LASTEXITCODE -ne 0) { throw "Benchmark failed: $name" }
            $report = Get-Content -Raw -LiteralPath $reportPath | ConvertFrom-Json
            [pscustomobject]@{
                Style = $style
                Requirements = $Requirements
                Connections = $count
                BuildMs = $report.summary.build_wall_time_ns.median / 1e6
                ParseMs = $report.summary.parse_ns.median / 1e6
                LoweringMs = $report.summary.lowering_ns.median / 1e6
                ResolutionMs = $report.summary.resolution_ns.median / 1e6
                Report = $reportPath
            }
        }
    }
} finally {
    Pop-Location
}
