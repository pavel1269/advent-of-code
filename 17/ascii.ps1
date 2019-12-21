
. ..\09\intComp.ps1

function Process-BotProgram {
    [CmdletBinding()]
    param(
        [string]
        $OutFile
    )

    $code = Get-Content ".\program.txt"
    $OpCode = $code.Split(',') | ForEach-Object { [int64]$_}
   
    $res = IntComp $OpCode

    $out = New-Object ([System.Text.StringBuilder])
    $res.Outputs | ForEach-Object {
        $out.Append([char]$_) | Out-Null
    }
    $map = $out.ToString()
    $map > $OutFile
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
                Write-Host "Intersection found at [$y][$x]: $($y * $x)"
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
 
    # $map = Get-Content "map-test.txt"
    $map = Get-Content "map-puzzle.txt"

    Calculate-Alignment $map

    # 6448 - correct
}
