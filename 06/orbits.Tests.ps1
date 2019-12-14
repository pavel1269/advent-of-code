
. ./orbits.ps1

$VerbosePreference = "SilentlyContinue"

function Assert {
    param($a, $b)

    if ($a -ne $b) {
        throw "'$a' differs from '$b'"
    }
}

$map = Get-Content "./input1.txt"
# $res = Calculate-OrbitMap $map
Assert (Calculate-OrbitMap $map).Center.Name "COM"

Assert (Get-NumberOfOrbits @("A)B")) 1
Assert (Get-NumberOfOrbits @("A)B", "B)C")) 3
Assert (Get-NumberOfOrbits @("A)B", "B)C", "C)D")) 6
Assert (Get-NumberOfOrbits $map) 42

$map = Get-Content "./input2.txt"
Assert (Get-NumberOfOrbitJumps $map "YOU" "SAN") 4
