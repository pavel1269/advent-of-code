
function Test-CanSee {
    [CmdletBinding()]
    param([string[]]$map, [int]$x1, [int]$y1, [int]$x2, [int]$y2)

    if ($x1 -eq $x2 -and $y1 -eq $y2) {
        return $false
    }

    Write-Verbose "Vision from [$x1,$y1] to [$x2,$y2]?"
    $plusDirection = $true # right, down or right, up

    if ($x1 -ge $x2 -and $y1 -ge $y2) {
        $tmp = $x1
        $x1 = $x2
        $x2 = $tmp
        $tmp = $y1
        $y1 = $y2
        $y2 = $tmp
    } elseif ($x1 -lt $x2 -and $y1 -lt $y2) {
        # nothing
    } else {
        $plusDirection = $false

        if ($x1 -gt $x2) {
            $tmp = $x1
            $x1 = $x2
            $x2 = $tmp
            $tmp = $y1
            $y1 = $y2
            $y2 = $tmp
        }
    }
    Write-Verbose "[$x1,$y1] - [$x2,$y2]: $plusDirection"

    $minX = [Math]::Min($x1, $x2)
    $maxX = [Math]::Max($x1, $x2)
    $minY = [Math]::Min($y1, $y2)
    $maxY = [Math]::Max($y1, $y2)

    if ($x1 - $x2 -eq 0) {
        for ($y = $minY + 1; $y -lt $maxY; $y++) {
            if ($map[$y][$x1] -eq '#') {
                Write-Verbose "(1) Vision from [$x1,$y1] to [$x2,$y2] obsured in [$x1,$y]"
                return $false
            }
        }
    } elseif ($y1 - $y2 -eq 0) {
        for ($x = $minX + 1; $x -lt $maxX; $x++) {
            if ($map[$y1][$x] -eq '#') {
                Write-Verbose "(2) Vision from [$x1,$y1] to [$x2,$y2] obsured in [$x,$y1]"
                return $false
            }
        }
    } else {
        # 0,0 - 4,2
        # distance x = 4 - 0 = 4
        # distance y = 2 - 0 = 2
        # y 1 -> x = 2
        # x 1 -> y = 0.5
        # x 2 -> y = 1
        # x 3 -> y = 1.5

        # 1,1 - 5,3
        # distance x = 5 - 1 = 4
        # distance y = 3 - 1 = 2
        # x 1 -> y = 1.5
        # x 2 -> y = 2
        # x 3 -> y = 2.5
        $distanceY = $maxY - $minY
        $distanceX = $maxX - $minX
        Write-Verbose "(3) Scanning y from '$($y1 + 1)' to '$($y2 - 1)'"
        for ($distance = 1; $distance -lt $distanceY; $distance++) {
            if ($plusDirection) {
                $y = $y1 + $distance
            } else {
                $y = $y1 - $distance
            }
            $x = $distanceX * $distance / $distanceY + $x1

            write-Verbose "   testing [$x,$y]"
            if ($x -is [int] -and $map[$y][$x] -eq '#') {
                Write-Verbose "(3) Vision from [$x1,$y1] to [$x2,$y2] obsured in [$x,$y]"
                return $false
            }
        }

        Write-Verbose "(4) Scanning x from '$($x1 + 1)' to '$($x2 - 1)'"
        for ($distance = 1; $distance -lt $distanceX; $distance++) {
            $x = $minX + $distance
            $y = $distanceY * $distance / $distanceX
            if ($plusDirection) {
                $y += $y1
            } else {
                $y = -$y + $y1
            }

            write-Verbose "   testing [$x,$y]"
            if ($y -is [int] -and $map[$y][$x] -eq '#') {
                Write-Verbose "(4) Vision from [$x1,$y1] to [$x2,$y2] obsured in [$x,$y]"
                return $false
            }
        }
    }

    return $true
}

function Get-CountVisibleTo {
    [CmdletBinding()]
    param([string[]]$map, [int]$testX, [int]$testY)

    $count = 0
    $width = $map[0].Length
    for ($y = 0; $y -lt $map.Count; $y++) {
        for ($x = 0; $x -lt $width; $x++) {
            if ($map[$y][$x] -eq '#' -and (Test-CanSee $map $testX $testY $x $y)) {
                $count++
            }
        }
    }

    return $count
}

function Get-VisibilityMap {
    [CmdletBinding()]
    param([string[]]$map)

    $visibilityMap = New-Object "System.Text.StringBuilder"
    $width = $map[0].Length
    for ($y = 0; $y -lt $map.Count; $y++) {
        for ($x = 0; $x -lt $width; $x++) {
            if ($map[$y][$x] -eq '.') {
                $visibilityMap.Append('.') | Out-Null
            } else {
                $visibilityMap.Append((Get-CountVisibleTo $map $x $y)) | Out-Null
            }
        }
        $visibilityMap.Append("`n") | Out-Null
    }
    
    return $visibilityMap.ToString()
}

function Get-BestVisibility {
    [CmdletBinding()]
    param([string]$mapString)

    $map = $mapString.Split()
    $width = $map[0].Length
    $best = 0
    $bestX = -1
    $bestY = -1
    for ($y = 0; $y -lt $map.Count; $y++) {
        for ($x = 0; $x -lt $width; $x++) {
            if ($map[$y][$x] -eq '.') {
                continue
            }

            $sees = Get-CountVisibleTo $map $x $y
            # Write-Host "[$x,$y] sees $sees"

            if ($sees -gt $best) {
                $best = $sees
                $bestX = $x
                $bestY = $y
            }
        }
    }

    return @{
        x = $bestX
        y = $bestY
        best = $best
    }
}

function Get-Part1Result {
    [CmdletBinding()]
    param()

    $mapString = ".#....#.###.........#..##.###.#.....##...
...........##.......#.#...#...#..#....#..
...#....##..##.......#..........###..#...
....#....####......#..#.#........#.......
...............##..#....#...##..#...#..#.
..#....#....#..#.....#.#......#..#...#...
.....#.#....#.#...##.........#...#.......
#...##.#.#...#.......#....#........#.....
....##........#....#..........#.......#..
..##..........##.....#....#.........#....
...#..##......#..#.#.#...#...............
..#.##.........#...#.#.....#........#....
#.#.#.#......#.#...##...#.........##....#
.#....#..#.....#.#......##.##...#.......#
..#..##.....#..#.........#...##.....#..#.
##.#...#.#.#.#.#.#.........#..#...#.##...
.#.....#......##..#.#..#....#....#####...
........#...##...#.....#.......#....#.#.#
#......#..#..#.#.#....##..#......###.....
............#..#.#.#....#.....##..#......
...#.#.....#..#.......#..#.#............#
.#.#.....#..##.....#..#..............#...
.#.#....##.....#......##..#...#......#...
.......#..........#.###....#.#...##.#....
.....##.#..#.....#.#.#......#...##..#.#..
.#....#...#.#.#.......##.#.........#.#...
##.........#............#.#......#....#..
.#......#.............#.#......#.........
.......#...##........#...##......#....#..
#..#.....#.#...##.#.#......##...#.#..#...
#....##...#.#........#..........##.......
..#.#.....#.....###.#..#.........#......#
......##.#...#.#..#..#.##..............#.
.......##.#..#.#.............#..#.#......
...#....##.##..#..#..#.....#...##.#......
#....#..#.#....#...###...#.#.......#.....
.#..#...#......##.#..#..#........#....#..
..#.##.#...#......###.....#.#........##..
#.##.###.........#...##.....#..#....#.#..
..........#...#..##..#..##....#.........#
..#..#....###..........##..#...#...#..#.."

    $res = Get-BestVisibility $mapString
    Write-Host "Best: '$($res.best)'"

    # 340 - correct (28, 29)
}

function Get-Angle {
    [CmdletBinding()]
    param([int]$x, [int]$y, [int]$x2, [int]$y2)

    Write-Verbose "Angle from [$x,$y] to [$x2,$y2]?"

    $distanceY = [Math]::Abs($x - $x2)
    $distanceX = [Math]::Abs($y - $y2)

    # up = 0
    # right = pi / 2
    if ($x -eq $x2) {
        if ($y -eq $y2) {
            throw "wtf"
        } elseif ($y -gt $y2) {
            return 0
        } else {
            return [Math]::PI
        }
    } elseif ($y -eq $y2) {
        if ($x -gt $x2) {
            return [Math]::PI * 1.5
        } else {
            return [Math]::PI / 2
        }
    } elseif ($x -lt $x2 -and $y -gt $y2) {
        Write-Verbose "quadrant 1"
        return [Math]::Atan($distanceY / $distanceX)
    } elseif ($x -lt $x2 -and $y -lt $y2) {
        Write-Verbose "quadrant 2"
        return [Math]::Atan($distanceX / $distanceY) + [Math]::PI / 2
    } elseif ($x -gt $x2 -and $y -lt $y2) {
        Write-Verbose "quadrant 3"
        return [Math]::Atan($distanceY / $distanceX) + [Math]::PI
    } elseif ($x -gt $x2 -and $y -gt $y2) {
        Write-Verbose "quadrant 4"
        return [Math]::Atan($distanceX / $distanceY) + [Math]::PI * 1.5
    } else {
        throw "wtf 2"
    }
}

function Get-Asteroids {
    [CmdletBinding()]
    param([string[]]$map, [int]$sx, [int]$sy)

    $width = $map[0].Length
    $asteroids = New-Object System.Collections.Generic.List[System.Object]
    for ($y = 0; $y -lt $map.Count; $y++) {
        for ($x = 0; $x -lt $width; $x++) {
            if ($map[$y][$x] -eq '#') {
                $angle = Get-Angle $sx $sy $x $y
                $asteroids.Add(@{
                    x = $x
                    y = $y
                    angle = $angle
                })
            }
        }
    }

    return $asteroids
}

function Update-AsteroidsVisibility {
    [CmdletBinding()]
    param([string[]]$map, [System.Collections.Generic.List[System.Object]]$asteroids, [int]$x, [int]$y)

    $asteroids | ForEach-Object {
        if (-not $_.visible) {
            $_.visible = Test-CanSee $map $x $y $_.x $_.y
        }
    }
}

function ShootDown-Asteroids {
    [CmdletBinding()]
    param([string]$mapString, [int]$x, [int]$y)

    $map = $mapString.Split()
    $map[$y] = $map[$y].Remove($x, 1).Insert($x, 'X')
    $asteroids = [System.Collections.Generic.List[System.Object]](Get-Asteroids $map $x $y)
    
    $angle = -1
    $shot = 0

    while ($asteroids.Count -gt 0) {
        Update-AsteroidsVisibility $map $asteroids $x $y
        $visibleAsteroidAngles = $asteroids | Where-Object { $_.visible } | ForEach-Object { $_.angle } | Sort-Object
        $targetAngle = $visibleAsteroidAngles | Where-Object { $angle -lt $_ } | Select-Object -first 1
        if ($visibleAsteroidAngles.Count -eq 0) {
            throw "nothing to shoot at"
        } elseif ($targetAngle.Count -eq 0) {
            $targetAngle = $visibleAsteroidAngles | Select-Object -first 1
        }
        $target = $asteroids | Where-Object { $_.visible -and $_.angle -eq $targetAngle }
    
        $asteroids.Remove($target) | Out-Null
        $map[$target.y] = $map[$target.y].Remove($target.x, 1).Insert($target.x, '.')
        $angle = $target.angle
        $shot++
    
        #Write-Host "Target '$shot' was at [$($target.x),$($target.y)], current angle: '$angle'"
        Write-Output @{
            x = $target.x
            y = $target.y
        }
    }
}

function Get-Part2Result {
    [CmdletBinding()]
    param()

    $mapString = ".#....#.###.........#..##.###.#.....##...
...........##.......#.#...#...#..#....#..
...#....##..##.......#..........###..#...
....#....####......#..#.#........#.......
...............##..#....#...##..#...#..#.
..#....#....#..#.....#.#......#..#...#...
.....#.#....#.#...##.........#...#.......
#...##.#.#...#.......#....#........#.....
....##........#....#..........#.......#..
..##..........##.....#....#.........#....
...#..##......#..#.#.#...#...............
..#.##.........#...#.#.....#........#....
#.#.#.#......#.#...##...#.........##....#
.#....#..#.....#.#......##.##...#.......#
..#..##.....#..#.........#...##.....#..#.
##.#...#.#.#.#.#.#.........#..#...#.##...
.#.....#......##..#.#..#....#....#####...
........#...##...#.....#.......#....#.#.#
#......#..#..#.#.#....##..#......###.....
............#..#.#.#....#.....##..#......
...#.#.....#..#.......#..#.#............#
.#.#.....#..##.....#..#..............#...
.#.#....##.....#......##..#...#......#...
.......#..........#.###....#.#...##.#....
.....##.#..#.....#.#.#......#...##..#.#..
.#....#...#.#.#.......##.#.........#.#...
##.........#............#.#......#....#..
.#......#.............#.#......#.........
.......#...##........#...##......#....#..
#..#.....#.#...##.#.#......##...#.#..#...
#....##...#.#........#..........##.......
..#.#.....#.....###.#..#.........#......#
......##.#...#.#..#..#.##..............#.
.......##.#..#.#.............#..#.#......
...#....##.##..#..#..#.....#...##.#......
#....#..#.#....#...###...#.#.......#.....
.#..#...#......##.#..#..#........#....#..
..#.##.#...#......###.....#.#........##..
#.##.###.........#...##.....#..#....#.#..
..........#...#..##..#..##....#.........#
..#..#....###..........##..#...#...#..#.."

    $x = 28
    $y = 29
    $shots = ShootDown-Asteroids $mapString $x $y

    Write-Host "Result: $($shots[199].x * 100 + $shots[199].y)"

    # 2628 - correct
}
