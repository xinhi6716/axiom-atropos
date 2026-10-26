param(
    [string]$Agent = 'codex',
    [switch]$Update
)
$ErrorActionPreference = 'Stop'
$pythonCommand = Get-Command python3,python,py -ErrorAction SilentlyContinue | Select-Object -First 1
if (-not $pythonCommand) { throw 'Python 3.10+ is required. Install Python, then rerun this command.' }
$installerFile = Join-Path ([System.IO.Path]::GetTempPath()) ('axiom-forge-' + [guid]::NewGuid() + '.py')
try {
    Invoke-WebRequest 'https://raw.githubusercontent.com/xinhi6716/axiom-forge/main/scripts/install.py' -OutFile $installerFile
    $installerArgs = @($installerFile, '--host', $Agent)
    if ($Update) { $installerArgs += '--update' }
    & $pythonCommand.Source @installerArgs
    if ($LASTEXITCODE -ne 0) { throw "Axiom Forge installer failed ($LASTEXITCODE)." }
} finally {
    Remove-Item -LiteralPath $installerFile -ErrorAction SilentlyContinue
}
