
. ..\09\intComp.ps1

function Out-Map {
    [CmdletBinding()]
    param($map, $pos, $direction, $min, $max)

    for ($rowIndex = $min.y; $rowIndex -le $max.y; $rowIndex++) {
        $line = New-Object "System.Text.StringBuilder"
        for ($columnIndex = $min.x; $columnIndex -le $max.x; $columnIndex++) {
            if ($rowIndex -eq $pos.y -and $columnIndex -eq $pos.x) {
                if ($map[$rowIndex] -and $map[$rowIndex][$columnIndex]) {
                    $line.Append([string]($direction + 5)) | Out-Null
                } else {
                    $line.Append([string]$direction) | Out-Null
                }
            }
            else {
                if ($map[$rowIndex] -and $map[$rowIndex][$columnIndex]) {
                    $line.Append("#") | Out-Null
                }
                else {
                    $line.Append(".") | Out-Null
                }
            }
        }
        write-host $line
    }
    write-host ""
}

function Run-Robot {
    [CmdletBinding()]
    param([int]$StartColor)

    $OpCode = Load-IntCompProgram "./program.txt"
    
    $CompInputs = @{
        OpCodeIndex = 0
        OpCodes     = $OpCode
        InputParams = @($StartColor)
    }
    $map = @{}
    $direction = 0
    $pos = @{
        x = 0
        y = 0
    }
    $min = @{
        x = 0
        y = 0
    }
    $max = @{
        x = 0
        y = 0
    }

    $i = 0

    $def = @(
        ,@(1, 0)
        ,@(0, 0)
        ,@(1, 0)
        ,@(1, 0)
        ,@(0, 1)
        ,@(1, 0)
        ,@(1, 0)
        ,@(0, 0)
        ,@(1, 0)
        ,@(0, 0)
    )

    do {
        $res = IntComp @CompInputs

        # $res = $CompInputs
        # $res.Outputs = $def[$i]
        # $res.OpCodeIndex = [decimal]5

        $i++

        if ($res.Outputs.Length -ne 2) { break }

        if ($res.Outputs[0] -eq 1) {
            if (-not $map[$pos.y]) { $map[$pos.y] = @{} }
            if (-not $map[$pos.y][$pos.x]) {
                $map[$pos.y][$pos.x] = $true
            }
        } elseif ($res.Outputs[0] -eq 0) {
            if ($map[$pos.y] -and $map[$pos.y][$pos.x]) {
                $map[$pos.y][$pos.x] = $false
            }
        } else { throw "unown paint '$($res.Outputs[0])'" }

        if ($res.Outputs[1] -eq 0) {
            $direction--
        } elseif ($res.Outputs[1] -eq 1) {
            $direction++
        } else { throw "unown durection '$($res.Outputs[1])'" }
        $direction = ($direction + 4) % 4

        switch ($direction) {
            0 { $pos.y-- }
            1 { $pos.x++ }
            2 { $pos.y++ }
            3 { $pos.x-- }
            default { throw "Unown direcition" }
        }
        
        if ($pos.y -lt $min.y) { $min.y = $pos.y }
        elseif ($pos.y -gt $max.y) { $max.y = $pos.y }

        if ($pos.x -lt $min.x) { $min.x = $pos.x }
        elseif ($pos.x -gt $max.x) { $max.x = $pos.x }
    
        $input = 0
        if ($map[$pos.y] -and $map[$pos.y][$pos.x]) { $input = 1 }

        # write-host "[$i]: $input"
        # Out-Map $map $pos $direction $min $max
        
        $CompInputs = $res
        $CompInputs.InputParams = @($input)
        $CompInputs.Remove("Outputs")
    } while ($res.OpCodeIndex -is [decimal])

    Out-Map $map $pos $direction $min $max

    return $map
}

function Get-Part1Result {
    [CmdletBinding()]
    param()

    $map = Run-Robot
    
    $count = 0
    for ($rowIndex = $min.y; $rowIndex -le $max.y; $rowIndex++) {
        for ($columnIndex = $min.x; $columnIndex -le $max.x; $columnIndex++) {
            if ($map[$rowIndex] -and $map[$rowIndex][$columnIndex] -is [bool]) {
                $count++
            }
        }
    }
    write-host "Result: $count"

    # 1307 - too low
    # 2339 - correct
    # 10351 - too high
}

function Get-Part2Result {
    [CmdletBinding()]
    param()

    Run-Robot 1 | Out-Null
    
    # .###...##..#..#.####.###..#....###..###....
    # .#..#.#..#.#..#.#....#..#.#....#..#.#..#...
    # .#..#.#....#..#.###..#..#.#....#..#.#..#...
    # .###..#.##.#..#.#....###..#....###..###....
    # .#....#..#.#..#.#....#....#....#....#.#..1.
    # .#.....###..##..####.#....####.#....#..#...
    # PGUEPLPR - correct
}
