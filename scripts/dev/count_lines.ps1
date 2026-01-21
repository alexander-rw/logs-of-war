# Count total lines in all .rs files throughout the repository
$repoRoot = git rev-parse --show-toplevel
$totalLines = Get-ChildItem -Path $repoRoot -Filter "*.rs" -Recurse |
    Get-Content |
    Measure-Object -Line |
    Select-Object -ExpandProperty Lines

Write-Output "Total lines in .rs files: $totalLines"
