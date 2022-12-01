
. ..\09\intComp.ps1

function Process-BotProgram {
    [CmdletBinding()]
    param(
        [string]
        $OutFile,

        [switch]
        $MoveMode = $false,

        [int[]]
        $inputProgram = @()
    )

    $code = Get-Content ".\program.txt"
    $OpCode = $code.Split(',') | ForEach-Object { [int64]$_}
    if ($MoveMode) {
        $OpCode[0] = 2
    }
   
    $res = IntComp $OpCode $inputProgram

    if ($OutFile) {
        $out = New-Object ([System.Text.StringBuilder])
        $res.Outputs | ForEach-Object {
            if ($_ -gt 255) {
                $out = $out.Append([string]$_)
            }
            else {
                $out = $out.Append([char]$_)
            }
        }
        $map = $out.ToString()
        $map > $OutFile
    }

    if ($MoveMode) {
        return $res
    }
}

function Calculate-Alignment {
    [CmdletBinding()]
    param(
        [string[]]
        $map
    )

    $width = $map[0].Length
    $height = $map.Count

    [int64]$sum = 0
    for ($y = 1; $y -lt $height; $y++) {
        for ($x = 1; $x -lt $map[$y].Length; $x++) {
            if ($map[$y][$x] -ne '#') {
                $x++
                continue
            }

            if ($map[$y][$x + 1] -ne '#') {
                $x += 2
                continue
            }

            if (($map[$y][$x - 1] -eq '#') -and ($map[$y - 1][$x] -eq '#') -and ($map[$y + 1][$x] -eq '#')) {
                # Write-Host "Intersection found at [$y][$x]: $($y * $x)"
                $sum += $y * $x
            }
        }
    }

    return $sum
}

function Get-Part1Result {
    [CmdletBinding()]
    param()

    Process-BotProgram "map-puzzle.txt"
    $map = Get-Content "map-puzzle.txt"
 
    # $map = Get-Content "map-test.txt"

    Calculate-Alignment $map

    # 6448 - correct
}

function Get-Movements {
    [CmdletBinding()]
    param(
        [int]$direction
    )

    $movement = @{
        left = $null
        right = $null
    }
    
    switch ($direction) {
        0 {
            $movement.left = {
                [CmdletBinding()]
                param($coords)
                return @{ x = $coords.x - 1; y = $coords.y }
            }
            $movement.right = {
                [CmdletBinding()]
                param($coords)
                return @{ x = $coords.x + 1; y = $coords.y }
            }
        }
        1 {
            $movement.left = {
                [CmdletBinding()]
                param($coords)
                return @{ x = $coords.x; y = $coords.y - 1 }
            }
            $movement.right = {
                [CmdletBinding()]
                param($coords)
                return @{ x = $coords.x; y = $coords.y + 1 }
            }
        }
        2 {
            $movement.left = {
                [CmdletBinding()]
                param($coords)
                return @{ x = $coords.x + 1; y = $coords.y }
            }
            $movement.right = {
                [CmdletBinding()]
                param($coords)
                return @{ x = $coords.x - 1; y = $coords.y }
            }
        }
        3 {
            $movement.left = {
                [CmdletBinding()]
                param($coords)
                return @{ x = $coords.x; y = $coords.y + 1 }
            }
            $movement.right = {
                [CmdletBinding()]
                param($coords)
                return @{ x = $coords.x; y = $coords.y - 1 }
            }
        }
        default {
            throw "direction fail"
        }
    }

    return $movement
}

function Pass-Through {
    [CmdletBinding()]
    param(
        [string[]]
        $map
    )

    $y = 0
    $me = $map | ForEach-Object {
        @{
            y = $y++
            x = $_.IndexOf('^')
        }
    } | Where-Object { $_.x -ne -1 }
    $map[$me.y] = $map[$me.y].Remove($me.x, 1).Insert($me.x, '#')
    
    $width = $map[0].Length
    $height = $map.Count

    $direction = 0
    # 0 - up
    # 1 - right
    # 2 - down
    # 3 - left

    $orders = @()
    function Is-Viable {
        [CmdletBinding()]
        param(
            [HashTable]
            $coords
        )

        return ($coords.x -ge 0 -and $coords.x -lt $width -and $coords.y -ge 0 -and $coords.y -lt $height -and $map[$coords.y][$coords.x] -eq '#')
    }

    while ($true) {
        $movements = Get-Movements $direction
        $left = & $movements.left $me
        $right = & $movements.right $me

        $command = @{
            Direction = $null
            Moves = 0
        }
        $nextDirectionMove = $null
        if (Is-Viable $right) {
            $command.Direction = 'R'
            $direction = $direction + 1
            $nextDirectionMove = $movements.right
        }
        elseif (Is-Viable $left) {
            $command.Direction = 'L'
            $direction = $direction - 1
            $nextDirectionMove = $movements.left
        }
        else {
            # write-host "Finish at [$($me.y)][$($me.x)]"
            break
        }

        while ($direction -lt 0) { $direction += 4 }
        $direction = $direction % 4

        $moves = 0
        while (Is-Viable $me) {
            # write-host "$($map[$me.y][$me.x]) at [$($me.y)][$($me.x)]"
            $lastLoc = $me
            $me = & $nextDirectionMove $me
            $moves++
        }
        $me = $lastLoc
        $command.Moves = $moves - 1
        $orders += $command

        # Write-Host "Moving '$($command.Moves)' x '$($command.Direction)', at [$($me.y)][$($me.x)]"
    }

    return $orders
}

function Get-Part2Result {
    [CmdletBinding()]
    param()

    Process-BotProgram "map-puzzle.txt"
    $map = Get-Content "map-puzzle.txt" | Where-Object { $_ }

    $map = Get-Content "map-test2.txt"

    $orders = (Pass-Through $map) | ForEach-Object { "$($_.Direction),$($_.Moves)"}
    [string]::Join(",", $orders)
    # R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2

    # L,4,L,6,L,8,L,12,L,8,R,12,L,12,L,8,R,12,L,12,L,4,L,6,L,8,L,12,L,8,R,12,L,12,R,12,L,6,L,6,L,8,L,4,L,6,L,8,L,12,R,12,L,6,L,6,L,8,L,8,R,12,L,12,R,12,L,6,L,6,L,8

    # A = L,8,R,12,L,12
    # L,4,L,6,L,8,L,12,A,A,L,4,L,6,L,8,L,12,A,R,12,L,6,L,6,L,8,L,4,L,6,L,8,L,12,R,12,L,6,L,6,L,8,A,R,12,L,6,L,6,L,8
    # B = R,12,L,6,L,6,L,8
    # L,4,L,6,L,8,L,12,A,A,L,4,L,6,L,8,L,12,A,B,L,4,L,6,L,8,L,12,B,A,B
    # C = L,4,L,6,L,8,L,12
    # C,A,A,C,A,B,C,B,A,B
    
    $program = @(
        "C,A,A,C,A,B,C,B,A,B",
        10,
        "L,8,R,12,L,12",
        10,
        "R,12,L,6,L,6,L,8",
        10,
        "L,4,L,6,L,8,L,12",
        10,
        'n',
        10
    ) | ForEach-Object { [int[]][char[]]$_ }

    # $program = @(
    #     "A",
    #     10,
    #     "L,4",
    #     10,
    #     "L,4",
    #     10,
    #     "L,4",
    #     10,
    #     'n',
    #     10
    # ) | ForEach-Object { [int[]][char[]]$_ }

    # [string]::Join(" ", $program)
    $res = Process-BotProgram -MoveMode -inputProgram $program -OutFile "result.txt"
    # $res
    $res.Outputs[$res.Outputs.Count - 1]

    # 914900 - correct
}
