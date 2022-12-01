
. ..\09\intComp.ps1

function Print-Beam {
    [CmdletBinding()]
    param($map, $startX = 0, $startY = 0, $countX = $map.Count, $countY = $map.Count)
    
    for ($y = $startY; $y -lt $countY; $y++) {
        $line = New-Object ([System.Text.Stringbuilder])
        $default = "."
        for ($x = $startX; $x -lt $countX; $x++) {
            if (($x -eq 695) -or ($x -eq 795)) {
                $line = $line.Append(" ")
            }
            switch ($map."$y"."$x") {
                1 {
                    $line = $line.Append("#")
                    $default = "-"
                }
                2 {
                    $line = $line.Append("=")
                    $default = "-"
                }
                -1 {
                    $line = $line.Append(",")
                    $default = "."
                }
                -2 {
                    $line = $line.Append("_")
                    $default = "."
                }
                default {
                    $line = $line.Append($default)
                }
            }
        }
        
        # Write-Host ($line.ToString())
        Write-Output ($line.ToString())
    }
}

function Get-Part1Res {
    [CmdletBinding()]
    param()

    $code = Get-Content ".\beamProgram.txt"
    $OpCode = $code.Split(',') | ForEach-Object { [int64]$_}
    
    $toScan = 50
    $map = New-Object "int[][]" $toScan
    for ($index = 0; $index -lt $toScan; $index++) {
        $map[$index] = New-Object "int[]" $toScan
    }

    for ($y = 0; $y -lt $toScan; $y++) {
        for ($x = 0; $x -lt $toScan; $x++) {
            $res = IntComp $OpCode @($x, $y)
            $lastResult = $res.Outputs[0]
            Write-Host "$(Get-Date) Scanned [$y][$x], res: '$lastResult'"
            
            if ($lastResult -eq 1) {
                $map[$y][$x] = 1
            }
            else {
                $map[$y][$x] = -1
            }
        }
    }

    Print-Beam $map

    (($map | ForEach-Object { $_ } ) | Where-Object { $_ -eq 1 }).Coun
    # 229 - correct

}

function Get-Part2Result {
    [CmdletBinding()]
    param()

    $code = Get-Content ".\beamProgram.txt"
    $OpCode = $code.Split(',') | ForEach-Object { [int64]$_}
    
    $map = @{}
    $y = 50
    $map."$y" = @{}

    # Start somewhere, based on previous result

    $found = $false
    $bottom = -1 # or left
    $top = -1 # or right
    for ($x = 0; $x -le $y; $x++) {
        $res = IntComp $OpCode @($x, $y)
        $lastResult = $res.Outputs[0]
        Write-Host "$(Get-Date) Scanned [$y][$x], res: '$lastResult'"

        if ($lastResult -eq 1) {
            $map."$y"."$x" = 1
            if (-not $found) {
                $found = $true
                $bottom = $x
            }
        }
        else {
            $map."$y"."$x" = -1
            if ($found) {
                $top = $x - 1
                break
            }
        }
    }

    # Print-Beam $map -CountX 51 -CountY 51
    # ,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,#########,......

    # Backup for backtraking from previous part :) 
    if ($false) {
        # $bakup = $map
        $map = $bakup.Clone()
        
        $y = 50
        $top = 43
        $bottom = 35
    }

    # Backup after finding first $size of 100
    if ($false) {
        # $bakup2 = $map
        $map = $bakup2.Clone()
        
        $y = 545
        $top = 479
        $bottom = 378
    }

    # Follow beam outline only
    $santaSize = 100
    $possibleLocations = @()
    while ($true) {
        $y++
        $map."$y" = @{}
        
        $res = IntComp $OpCode @($bottom, $y)
        $lastResult = $res.Outputs[0]
        Write-Host "$(Get-Date) Scanned [$y][$bottom], res: '$lastResult'"

        if ($lastResult -eq 1) {
            $map."$y"."$bottom" = 1
        }
        else {
            $map."$y"."$bottom" = -1
            $bottom++
            $map."$y"."$bottom" = 2

            $possibleLocations | ForEach-Object {
                $_.width--
                $_.x++
            }
        }
        
        $res = IntComp $OpCode @(($top + 1), $y)
        $lastResult = $res.Outputs[0]
        Write-Host "$(Get-Date) Scanned [$y][$($top + 1)], res: '$lastResult'"

        if ($lastResult -eq 1) {
            $top++
            $map."$y"."$top" = 1
            $map."$y"."$($top + 1)" = -2
        }
        else {
            $map."$y"."$($top + 1)" = -1
            $map."$y"."$top" = 2
        }
        
        $size = $top - $bottom

        if ($size -lt $santaSize) {
            $possibleLocations = @()
            Write-host "'$y': '$size' (possibles: '$($possibleLocations.Count)')"
            continue
        }
        # break

        $possibleLocations = @($possibleLocations | Where-Object { $_.width -ge $santaSize })
        Write-host "'$y': '$size' (possibles: '$($possibleLocations.Count)')"
        $possibleLocations | ForEach-Object {
            $_.heigh++
        }

        $match = $possibleLocations | Where-Object { $_.heigh -ge $santaSize }
        if ($match) {
            Write-Host "Closest santa loc: [$($match.y)][$($match.x)], dist: '$($match.x * 10000 + $match.y)'"
            break
        }

        $possibleLocations += @{
            x = $bottom
            y = $y
            width = $size + 1
            heigh = 1
        }
    }

    # Print-Beam $map -StartY 50 -StartX 32 -CountX ($top + 2) -CountY ($y + 1) > out1.txt
    # Print-Beam $map -StartY 520 -StartX 360 -CountX ($top + 2) -CountY ($y + 1) > out2.txt
    Print-Beam $map -StartY 893 -StartX 600 -CountX 900 -CountY 1010 > out3.txt

    # 6970906 - too high
    # 6950903 - correct
}
