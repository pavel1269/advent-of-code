
. .\ascii.ps1

$VerbosePreference = "SilentlyContinue"

function Assert {
    param($a, $b)

    if ($a -ne $b) {
        Write-Error "'$a' differs from '$b'" -ErrorAction "Stop"
    }
}

$map = Get-Content "map-test.txt"
Assert (Calculate-Alignment $map) 76
$map = Get-Content "map-puzzle.txt"
Assert (Calculate-Alignment $map) 6448
$map = Get-Content "map-test2.txt"
Assert ([string]::Join(",", ((Pass-Through $map) | ForEach-Object { "$($_.Direction),$($_.Moves)"}))) "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2"
