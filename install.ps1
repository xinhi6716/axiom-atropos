$ErrorActionPreference = 'Stop'
$runtime = Join-Path $PSScriptRoot 'bin/atropos.exe'
if (-not (Test-Path -LiteralPath $runtime -PathType Leaf)) {
    throw 'Extract the Windows release archive first, or build with cargo build --release --locked.'
}
& $runtime install @args
exit $LASTEXITCODE
