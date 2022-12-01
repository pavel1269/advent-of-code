
$ErrorActionPreference = "Stop"

function Get-BioDiversity {
    [CmdletBinding()]
    param(
        [bool[][]]$Map,
        [int]$size = 5
    )

    $rating = 0
    $ratingWorth = 1
    for ($rowIndex = 0; $rowIndex -lt $size; $rowIndex++) {
        for ($columnIndex = 0; $columnIndex -lt $size; $columnIndex++) {
            if ($Map[$rowIndex][$columnIndex]) {
                $rating += $ratingWorth
            }
            $ratingWorth *= 2
        }
    }

    return $rating
}

function Prepare-Map {
    [CmdletBinding()]
    param($size)

    $size = 5
    $map = New-Object "bool[][]" $size
    for ($index = 0; $index -lt $map.Count; $index++) {
        $map[$index] = New-Object "bool[]" $size
    }

    return $map
}

function New-MapState {
    [CmdletBinding()]
    param([bool[][]]$Map)
    
    $size = $Map.Count
    $NewMap = Prepare-Map $size
    
    for ($rowIndex = 0; $rowIndex -lt $size; $rowIndex++) {
        for ($columnIndex = 0; $columnIndex -lt $size; $columnIndex++) {
            $neighbours = 0
            if ($rowIndex -gt 0 -and $Map[$rowIndex - 1][$columnIndex]) {
                $neighbours++
            }
            if ($columnIndex -gt 0 -and $Map[$rowIndex][$columnIndex - 1]) {
                $neighbours++
            }
            if ($rowIndex -lt ($size - 1) -and $Map[$rowIndex + 1][$columnIndex]) {
                $neighbours++
            }
            if ($columnIndex -lt ($size - 1) -and $Map[$rowIndex][$columnIndex + 1]) {
                $neighbours++
            }

            if ($Map[$rowIndex][$columnIndex]) {
                $NewMap[$rowIndex][$columnIndex] = $neighbours -eq 1
            }
            else {
                $NewMap[$rowIndex][$columnIndex] = $neighbours -ge 1 -and $neighbours -le 2
            }
        }
    }

    return $NewMap
}

function Print-Map {
    [CmdletBinding()]
    param([bool[][]]$Map)

    $size = $Map.Count
    for ($rowIndex = 0; $rowIndex -lt $size; $rowIndex++) {
        Write-Host ([string]::Join(" ", ($Map[$rowIndex] | ForEach-Object { if ($_) { "#" } else { "." } })))
    }
}

function Print-Space {
    [CmdletBinding()]
    param([hashtable]$Space)

    $Space.Keys | Sort-Object | ForEach-Object {
        $spaceLevel = $_

        Write-Host "$($spaceLevel):"
        Print-Map ($Space.$spaceLevel)
    }
}

function Get-BiodiversityRating {
    [CmdletBinding()]
    param([bool[][]]$Map)

    $bioDiversivities = New-Object "System.Collections.ArrayList"
    while ($true) {
        $currentBioDiv = Get-BioDiversity $Map
        if ($bioDiversivities.IndexOf($currentBioDiv) -ne -1) {
            return $currentBioDiv
        }
        $bioDiversivities += $currentBioDiv
        $Map = New-MapState $Map
        # Print-Map $map
    }
}

function Get-Part1Result {
    [CmdletBinding()]
    param()

    $map = @(
        ,@(0, 1, 1, 0, 0)
        ,@(1, 1, 0, 1, 0)
        ,@(1, 1, 0, 1, 1)
        ,@(0, 1, 0, 0, 1)
        ,@(1, 0, 1, 1, 1)
    )

    $res = Get-BiodiversityRating $map
    $res

    # 18400817 - correct
}

function New-RecursiveMapState {
    [CmdletBinding()]
    param(
        [Hashtable]$Space,
        [int]$spaceLevel,
        [bool[][]]$Map,
        
        [int]$size = 5,
        [int]$middle = 2
    )

    $NewMap = Prepare-Map $size
    for ($rowIndex = 0; $rowIndex -lt $size; $rowIndex++) {
        for ($columnIndex = 0; $columnIndex -lt $size; $columnIndex++) {
            if ($rowIndex -eq $middle -and $columnIndex -eq $middle) {
                continue
            }

            $neighbours = 0

            # Up
            if ($rowIndex -eq 0) {
                if ($Space.$($spaceLevel - 1) -and $Space.$($spaceLevel - 1)[1][2]) {
                    $neighbours++
                }
            }
            elseif ($rowIndex -eq 3 -and $columnIndex -eq 2) {
                $SpacePlusMap = $Space.$($spaceLevel + 1)
                if ($SpacePlusMap) {
                    for ($indexWithin = 0; $indexWithin -lt $size; $indexWithin++) {
                        if ($SpacePlusMap[$size - 1][$indexWithin]) {
                            $neighbours++
                        }
                    }
                }
            }
            else {
                if ($Map[$rowIndex - 1][$columnIndex]) {
                    $neighbours++
                }
            }

            # Down
            if ($rowIndex -eq ($size - 1)) {
                if ($Space.$($spaceLevel - 1) -and $Space.$($spaceLevel - 1)[3][2]) {
                    $neighbours++
                }
            }
            elseif ($rowIndex -eq 1 -and $columnIndex -eq 2) {
                $SpacePlusMap = $Space.$($spaceLevel + 1)
                if ($SpacePlusMap) {
                    for ($indexWithin = 0; $indexWithin -lt $size; $indexWithin++) {
                        if ($SpacePlusMap[0][$indexWithin]) {
                            $neighbours++
                        }
                    }
                }
            }
            else {
                if ($Map[$rowIndex + 1][$columnIndex]) {
                    $neighbours++
                }
            }

            # Left
            if ($columnIndex -eq 0) {
                if ($Space.$($spaceLevel - 1) -and $Space.$($spaceLevel - 1)[2][1]) {
                    $neighbours++
                }
            }
            elseif ($rowIndex -eq 2 -and $columnIndex -eq 3) {
                $SpacePlusMap = $Space.$($spaceLevel + 1)
                if ($SpacePlusMap) {
                    for ($indexWithin = 0; $indexWithin -lt $size; $indexWithin++) {
                        if ($SpacePlusMap[$indexWithin][$size - 1]) {
                            $neighbours++
                        }
                    }
                }
            }
            else {
                if ($Map[$rowIndex][$columnIndex - 1]) {
                    $neighbours++
                }
            }

            # Right
            if ($columnIndex -eq ($size - 1)) {
                if ($Space.$($spaceLevel - 1) -and $Space.$($spaceLevel - 1)[2][3]) {
                    $neighbours++
                }
            }
            elseif ($rowIndex -eq 2 -and $columnIndex -eq 1) {
                $SpacePlusMap = $Space.$($spaceLevel + 1)
                if ($SpacePlusMap) {
                    for ($indexWithin = 0; $indexWithin -lt $size; $indexWithin++) {
                        if ($SpacePlusMap[$indexWithin][0]) {
                            $neighbours++
                        }
                    }
                }
            }
            else {
                if ($Map[$rowIndex][$columnIndex + 1]) {
                    $neighbours++
                }
            }

            Write-Verbose "[$spaceLevel].[$rowIndex][$columnIndex] has '$neighbours' neighbours"
            # Evaluate life
            if ($Map[$rowIndex][$columnIndex]) {
                $NewMap[$rowIndex][$columnIndex] = $neighbours -eq 1
            }
            else {
                $NewMap[$rowIndex][$columnIndex] = $neighbours -ge 1 -and $neighbours -le 2
            }
        }
    }

    return $NewMap
}

function New-RecursiveSpaceState {
    [CmdletBinding()]
    param(
        [Hashtable]$Space,
        [int]$size = 5,
        [int]$middle = 2
    )

    $spaceLevelMin = 0
    $spaceLevelMax = 0
    
    $newSpace = @{}
    $Space.Keys | ForEach-Object {
        $spaceLevel = [int]$_

        if ($spaceLevel -gt $spaceLevelMax) {
            $spaceLevelMax = $spaceLevel
        }
        if ($spaceLevel -lt $spaceLevelMin) {
            $spaceLevelMin = $spaceLevel
        }

        $map = $Space.$spaceLevel
        $newMap = New-RecursiveMapState $Space $spaceLevel $map

        $newSpace.$spaceLevel = $NewMap
    }

    # Now evaluate possible new spaces
    @(($spaceLevelMin - 1), ($spaceLevelMax + 1)) | ForEach-Object {
        $spaceLevel = $_

        $NewMap = Prepare-Map $size
        $NewMap = New-RecursiveMapState $Space $spaceLevel $NewMap
        $rating = Get-NumberOfBugsInMap $NewMap
        Write-Verbose "[$spaceLevel] has rating '$rating'"
        if ($rating -gt 0) {
            $newSpace.$spaceLevel = $NewMap
        }
    }
    
    return $newSpace
}

function Get-NumberOfBugsInMap {
    [CmdletBinding()]
    param(
        [bool[][]]$Map
    )

    $sum = $map | ForEach-Object { $_ } | Where-Object { $_ } | Measure-Object | Select-Object -ExpandProperty Count
    return $sum
}

function Get-NumberOfBugsInSpace {
    [CmdletBinding()]
    param(
        [hashtable]$space
    )

    $sum = 0
    $Space.Keys | ForEach-Object {
        $spaceLevel = [int]$_
        $map = $Space.$spaceLevel

        $sum += Get-NumberOfBugsInMap $map
    }
    return $sum
}

function Monitor-Space {
    [CmdletBinding()]
    [CmdletBinding()]
    param(
        [hashtable]$space,
        [int]$iterations
    )

    try {
        for ($index = 0; $index -lt $iterations; $index++) {
            $space = New-RecursiveSpaceState $space -Verbose:$false
            Write-Verbose "$(Get-Date) Iteration $index done"
        }
    }
    catch {
        $msg = $_ | Out-String
        $stacktrace = $_.ScriptStackTrace | Out-String
        Write-Error "$msg`n$stacktrace" -ErrorAction "stop"
    }

    return $space
}

function Get-Part2Result {
    [CmdletBinding()]
    param(
        [hashtable]$space
    )

    $map = @(
        ,@(0, 1, 1, 0, 0)
        ,@(1, 1, 0, 1, 0)
        ,@(1, 1, 0, 1, 1)
        ,@(0, 1, 0, 0, 1)
        ,@(1, 0, 1, 1, 1)
    )
    
    $space = @{}
    $space.0 = $map
    
    $newSpace = Monitor-Space $space 200 -Verbose
    # Print-Space $newSpace
    
    $res = Get-NumberOfBugsInSpace $newSpace
    $res

    # 1944 - correct
}
