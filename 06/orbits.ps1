
$ErrorActionPreference = "Stop"

function Calculate-OrbitMap {
    [CmdletBinding()]
    param(
        [string[]]
        $Map
    )

    Write-Verbose "Contsructing star map"

    $Center = $null
    $MapData = @{}

    $Map | ForEach-Object {
        $rawCoord = $_

        if (-not ($rawCoord -match "^(\w+)\)(\w+)$")) {
            throw "Failed to parse '$rawCoord'"
        }

        $orbit = $Matches.1
        $orbitee = $Matches.2

        if (-not $MapData.$orbit) {
            $MapData.$orbit = @{
                Name = $orbit
                InOrbit = @()
                Orbits = $null
            }
        }
        if (-not $MapData.$orbitee) {
            $MapData.$orbitee = @{
                Name = $orbitee
                InOrbit = @()
                Orbits = $null
            }
        }

        $MapData.$orbit.InOrbit += $MapData.$orbitee
        $MapData.$orbitee.Orbits = $MapData.$orbit

        if (-not $Center) {
            $Center = $MapData.$orbit
        }
        elseif ($MapData.$orbitee.Name -eq $Center.Name) {
            $Center = $MapData.$orbit
        }
    }

    while ($Center.Orbits) {
        $Center = $Center.Orbits
    }
    
    return $Center
}

function Get-NumberOfOrbits {
    [CmdletBinding()]
    param(
        [string[]]
        $Map
    )

    $MapCenter = Calculate-OrbitMap $Map

    Write-Verbose "Calculating orbitees"
    $res = Get-NumberOfOrbitees $MapCenter
    return $res
}

function Get-NumberOfOrbitees {
    [CmdletBinding()]
    param(
        [hashtable]
        $MapPoint,

        [int]
        $Level = 0
    )

    Write-Verbose "Calculating orbitees of '$($MapPoint.Name)' ('$Level')"
    #$sum = $MapPoint.InOrbit.Count
    $sum = 0
    $sumChilds = $MapPoint.InOrbit | ForEach-Object { Get-NumberOfOrbitees $_ ($Level + 1) } | Measure-Object -Sum | Select-Object -ExpandProperty Sum
    $sum += $sumChilds
    if ($Level -ge 1) {
        $sum += $Level 
    }

    Write-Verbose "'$($MapPoint.Name)' has '$sum'"
    return $sum

}

function Get-Part1Result {
    $map = Get-Content "./part1.txt"
    $res = Get-NumberOfOrbits $map
    $res
}
