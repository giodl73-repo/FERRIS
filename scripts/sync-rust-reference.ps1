[CmdletBinding()]
param(
    [string]$MaximRoot,
    [switch]$Check
)

$ErrorActionPreference = "Stop"

$FerrisRoot = Split-Path -Parent $PSScriptRoot
if (-not $MaximRoot) {
    $MaximRoot = Join-Path $FerrisRoot "..\TRACKER\repos\knowledge-systems\maxim"
}

$MaximRoot = [IO.Path]::GetFullPath($MaximRoot)
$DestinationRoot = Join-Path $FerrisRoot "docs\reference\rust-reference"

$Sets = @(
    @{
        Source = Join-Path $MaximRoot "rust-language"
        Destination = Join-Path $DestinationRoot "rust-language"
        Pattern = "*.md"
    },
    @{
        Source = Join-Path $MaximRoot "rust-architecture"
        Destination = Join-Path $DestinationRoot "rust-architecture"
        Pattern = "*.md"
    },
    @{
        Source = Join-Path $MaximRoot "rust-application-blueprints"
        Destination = Join-Path $DestinationRoot "rust-application-blueprints"
        Pattern = "*.md"
    },
    @{
        Source = Join-Path $MaximRoot "rust-production-engineering"
        Destination = Join-Path $DestinationRoot "rust-production-engineering"
        Pattern = "*.md"
    },
    @{
        Source = Join-Path $MaximRoot "rust-crate-ecosystem"
        Destination = Join-Path $DestinationRoot "rust-crate-ecosystem"
        Pattern = "*.md"
    },
    @{
        Source = Join-Path $MaximRoot "rust-interop-migration"
        Destination = Join-Path $DestinationRoot "rust-interop-migration"
        Pattern = "*.md"
    },
    @{
        Source = Join-Path $MaximRoot "rust-security-assurance"
        Destination = Join-Path $DestinationRoot "rust-security-assurance"
        Pattern = "*.md"
    },
    @{
        Source = Join-Path $MaximRoot "rust-performance"
        Destination = Join-Path $DestinationRoot "rust-performance"
        Pattern = "*.md"
    },
    @{
        Source = Join-Path $MaximRoot "languages"
        Destination = Join-Path $DestinationRoot "languages"
        Pattern = "09-RUST.md"
    }
)

function Get-RelativePath {
    param(
        [string]$BasePath,
        [string]$Path
    )

    $baseUri = [Uri]::new(($BasePath.TrimEnd("\") + "\"))
    $pathUri = [Uri]::new($Path)
    return [Uri]::UnescapeDataString($baseUri.MakeRelativeUri($pathUri).ToString()).Replace("/", "\")
}

$ExpectedFiles = [System.Collections.Generic.HashSet[string]]::new(
    [StringComparer]::OrdinalIgnoreCase
)
$ManifestFiles = @()
$Failures = @()

foreach ($Set in $Sets) {
    if (-not (Test-Path -LiteralPath $Set.Source -PathType Container)) {
        throw "MAXIM source directory not found: $($Set.Source)"
    }

    $SourceFiles = @(Get-ChildItem -LiteralPath $Set.Source -File -Filter $Set.Pattern |
        Sort-Object Name)
    if ($SourceFiles.Count -eq 0) {
        throw "No MAXIM source files matched $($Set.Source)\$($Set.Pattern)"
    }

    if (-not $Check) {
        New-Item -ItemType Directory -Path $Set.Destination -Force | Out-Null
    }

    foreach ($SourceFile in $SourceFiles) {
        $DestinationFile = Join-Path $Set.Destination $SourceFile.Name
        $RelativeDestination = Get-RelativePath $FerrisRoot $DestinationFile
        [void]$ExpectedFiles.Add($DestinationFile)

        if ($Check) {
            if (-not (Test-Path -LiteralPath $DestinationFile -PathType Leaf)) {
                $Failures += "missing: $RelativeDestination"
                continue
            }

            $SourceHash = (Get-FileHash -LiteralPath $SourceFile.FullName -Algorithm SHA256).Hash
            $DestinationHash = (Get-FileHash -LiteralPath $DestinationFile -Algorithm SHA256).Hash
            if ($SourceHash -ne $DestinationHash) {
                $Failures += "drifted: $RelativeDestination"
            }
        }
        else {
            Copy-Item -LiteralPath $SourceFile.FullName -Destination $DestinationFile -Force
            $SourceHash = (Get-FileHash -LiteralPath $SourceFile.FullName -Algorithm SHA256).Hash
            $ManifestFiles += [ordered]@{
                source = Get-RelativePath $MaximRoot $SourceFile.FullName
                mirror = Get-RelativePath $FerrisRoot $DestinationFile
                sha256 = $SourceHash.ToLowerInvariant()
            }
        }
    }

    if (Test-Path -LiteralPath $Set.Destination -PathType Container) {
        foreach ($DestinationFile in Get-ChildItem -LiteralPath $Set.Destination -File -Filter "*.md") {
            if (-not $ExpectedFiles.Contains($DestinationFile.FullName)) {
                $RelativeDestination = Get-RelativePath $FerrisRoot $DestinationFile.FullName
                if ($Check) {
                    $Failures += "unexpected: $RelativeDestination"
                }
                else {
                    Remove-Item -LiteralPath $DestinationFile.FullName
                }
            }
        }
    }
}

if ($Check) {
    if ($Failures.Count -gt 0) {
        $Failures | ForEach-Object { Write-Error $_ }
        exit 1
    }

    Write-Host "Rust reference mirror is current ($($ExpectedFiles.Count) files)."
    exit 0
}

$SourceCommit = (& git -C $MaximRoot rev-parse HEAD).Trim()
$SourceDirty = [bool](& git -C $MaximRoot status --porcelain)
$Manifest = [ordered]@{
    schema_version = "ferris.maxim-rust-mirror.v1"
    canonical_repository = "MAXIM"
    canonical_root = $MaximRoot
    source_commit = $SourceCommit
    source_dirty = $SourceDirty
    files = $ManifestFiles
}

$ManifestPath = Join-Path $DestinationRoot "MIRROR-MANIFEST.json"
$Json = $Manifest | ConvertTo-Json -Depth 5
[IO.File]::WriteAllText($ManifestPath, ($Json + [Environment]::NewLine), [Text.UTF8Encoding]::new($false))

Write-Host "Synchronized $($ManifestFiles.Count) Rust reference files into FERRIS."
Write-Host "Manifest: $ManifestPath"
