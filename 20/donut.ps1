
function Parse-Map {
    [CmdletBinding()]
    param([string]$stringMap)

    $state = @{
        map = $stringMap.Split([Environment]::NewLine)
    }

    $mazeStart = $null
    $mazeLast = $null
    
    $mazeWidth = -1
    $innerPartStart = $null
    $innerPartWidth = -1
    $innerPartHeight = -1

    $possiblePortals = New-Object "System.Collections.ArrayList"

    for ($indexY = 0; $indexY -lt $state.map.Count; $indexY++) {
        for ($indexX = 0; $indexX -lt $state.map[$indexY].Length; $indexX++) {
            $field = $state.map[$indexY][$indexX]

            if ($field -eq '#') {
                if ($null -eq $mazeStart) {
                    $mazeStart = @{ x = $indexX; y = $indexY }
                    $mazeWidth = 1
                } else {
                    $mazeLast = @{ x = $indexX; y = $indexY }
                    if ($mazeStart.y -eq $indexY) {
                        $mazeWidth++
                    }
                }
            }

            if ($field -ne '#' -and $field -ne '.' -and $null -ne $mazeStart -and $indexY -gt $mazeStart.y -and $indexX -gt $mazeStart.x -and $indexX -lt $mazeStart.x + $mazeWidth) {
                if ($null -eq $innerPartStart) {
                    $innerPartStart = @{ x = $indexX; y = $indexY }
                    $innerPartWidth = 1
                    $innerPartHeight = 0
                } elseif ($indexY -eq $innerPartStart.y) {
                    $innerPartWidth++
                } elseif ($indexX -eq $innerPartStart.x) {
                    $innerPartHeight++
                }
            }

            $fieldChar = [char]$field
            if ($fieldChar -ge 'A' -and $fieldChar -le 'Z') {
                [void]$possiblePortals.Add(@{
                    x = $indexX
                    y = $indexY
                    letter = $fieldChar
                })
            }
        }
    }

    $state.start = $mazeStart
    $state.end = $mazeLast
    $state.innerStart = $innerPartStart
    $state.innerEnd = @{ x = $innerPartStart.x + $innerPartWidth - 1; y = $innerPartStart.y + $innerPartHeight - 2 }

    Write-Verbose "$(Get-Date -DisplayHint Time) Map parsed"
    
    $portals = @()
    for ($indexX = $state.start.x; $indexX -le $state.end.x; $indexX++) {
        $indexY = $state.start.y
        $field = $state.map[$indexY][$indexX]
        if ($field -eq '.') {
            $res = @($possiblePortals | Where-Object { $_.x -eq $indexX } | Where-Object { $_.y -eq $indexY - 1 -or $_.y -eq $indexY - 2 })
            if ($res.Count -eq 2) {
                $portals += @{
                    name = $state.map[$indexY - 2][$indexX] + $state.map[$indexY - 1][$indexX]
                    pos = @{ x = $indexX; y = $indexY }
                }
            } elseif ($res.Count -ne 0) {
                throw "Error parsing map x1"
            }
        }

        $indexY = $state.end.y
        $field = $state.map[$indexY][$indexX]
        if ($field -eq '.') {
            $res = @($possiblePortals | Where-Object { $_.x -eq $indexX } | Where-Object { $_.y -eq $indexY + 1 -or $_.y -eq $indexY + 2 })
            if ($res.Count -eq 2) {
                $portals += @{
                    name = $state.map[$indexY + 1][$indexX] + $state.map[$indexY + 2][$indexX]
                    pos = @{ x = $indexX; y = $indexY }
                }
            } elseif ($res.Count -ne 0) {
                throw "Error parsing map x2"
            }
        }
    }
    for ($indexX = $state.innerStart.x; $indexX -le $state.innerEnd.x - 1; $indexX++) {
        $indexY = $state.innerEnd.y + 1
        $field = $state.map[$indexY][$indexX]
        if ($field -eq '.') {
            $res = @($possiblePortals | Where-Object { $_.x -eq $indexX } | Where-Object { $_.y -eq $indexY - 1 -or $_.y -eq $indexY - 2 })
            if ($res.Count -eq 2) {
                $portals += @{
                    name = $state.map[$indexY - 2][$indexX] + $state.map[$indexY - 1][$indexX]
                    pos = @{ x = $indexX; y = $indexY }
                }
            } elseif ($res.Count -ne 0) {
                throw "Error parsing map xi1"
            }
        }

        $indexY = $state.innerStart.y - 1
        $field = $state.map[$indexY][$indexX]
        if ($field -eq '.') {
            $res = @($possiblePortals | Where-Object { $_.x -eq $indexX } | Where-Object { $_.y -eq $indexY + 1 -or $_.y -eq $indexY + 2 })
            if ($res.Count -eq 2) {
                $portals += @{
                    name = $state.map[$indexY + 1][$indexX] + $state.map[$indexY + 2][$indexX]
                    pos = @{ x = $indexX; y = $indexY }
                }
            } elseif ($res.Count -ne 0) {
                throw "Error parsing map xi2"
            }
        }
    }
    for ($indexY = $state.start.y; $indexY -le $state.end.y; $indexY++) {
        $indexX = $state.start.x
        $field = $state.map[$indexY][$indexX]
        Write-Verbose "$indexX $indexY"
        if ($field -eq '.') {
            $res = @($possiblePortals | Where-Object { $_.y -eq $indexY } | Where-Object { $_.x -eq $indexX - 1 -or $_.x -eq $indexX - 2 })
            if ($res.Count -eq 2) {
                $portals += @{
                    name = $state.map[$indexY][$indexX - 2] + $state.map[$indexY][$indexX - 1]
                    pos = @{ x = $indexX; y = $indexY }
                }
            } elseif ($res.Count -ne 0) {
                throw "Error parsing map y1"
            }
        }

        $indexX = $state.end.x
        $field = $state.map[$indexY][$indexX]
        if ($field -eq '.') {
            $res = @($possiblePortals | Where-Object { $_.y -eq $indexY } | Where-Object { $_.x -eq $indexX + 1 -or $_.x -eq $indexX + 2 })
            if ($res.Count -eq 2) {
                $portals += @{
                    name = $state.map[$indexY][$indexX + 1] + $state.map[$indexY][$indexX + 2]
                    pos = @{ x = $indexX; y = $indexY }
                }
            } elseif ($res.Count -ne 0) {
                throw "Error parsing map y2"
            }
        }
    }
    for ($indexY = $state.innerStart.y; $indexY -le $state.innerEnd.y; $indexY++) {
        $indexX = $state.innerEnd.x + 1
        $field = $state.map[$indexY][$indexX]
        Write-Verbose "$indexX $indexY"
        if ($field -eq '.') {
            $res = @($possiblePortals | Where-Object { $_.y -eq $indexY } | Where-Object { $_.x -eq $indexX - 1 -or $_.x -eq $indexX - 2 })
            if ($res.Count -eq 2) {
                $portals += @{
                    name = $state.map[$indexY][$indexX - 2] + $state.map[$indexY][$indexX - 1]
                    pos = @{ x = $indexX; y = $indexY }
                }
            } elseif ($res.Count -ne 0) {
                throw "Error parsing map yi1"
            }
        }

        $indexX = $state.innerStart.x - 1
        $field = $state.map[$indexY][$indexX]
        if ($field -eq '.') {
            $res = @($possiblePortals | Where-Object { $_.y -eq $indexY } | Where-Object { $_.x -eq $indexX + 1 -or $_.x -eq $indexX + 2 })
            if ($res.Count -eq 2) {
                $portals += @{
                    name = $state.map[$indexY][$indexX + 1] + $state.map[$indexY][$indexX + 2]
                    pos = @{ x = $indexX; y = $indexY }
                }
            } elseif ($res.Count -ne 0) {
                throw "Error parsing map yi2"
            }
        }
    }

    $resultPortals = @{}
    $portals.Name | Select-Object -Unique | Where-Object {
        $portal = $_
        $connectedPortal = @($portals | Where-Object { $_.name -eq $portal })

        if ($connectedPortal.Count -eq 2) {
            $portal1 = $connectedPortal[0]
            $portal1str = "$($portal1.pos.x)x$($portal1.pos.y)"

            $portal2 = $connectedPortal[1]
            $portal2str = "$($portal2.pos.x)x$($portal2.pos.y)"

            # write-host "$portal $portal1str $portal2str"

            $resultPortals[$portal1str] = $portal2.pos
            $resultPortals[$portal2str] = $portal1.pos
        } elseif ($portal -eq "AA") {
            $state.startPos = $connectedPortal[0].pos
        } elseif ($portal -eq "ZZ") {
            $state.endPos = $connectedPortal[0].pos
        } else {
            throw "Error parsing map portals"
        }
    }

    $state.portals = $resultPortals
    Write-Verbose "$(Get-Date -DisplayHint Time) Map portals parsed"
    
    return $state
}

function Get-AccessibleSurrounding {
    [CmdletBinding()]
    param($state, $where, $level = 0)

    if (-not ($state.map[$where.y]) -or $state.map.Count -le $where.y) {
        throw "Testing unown area Y"
    }
    if (-not ($state.map[$where.y][$where.x]) -or $state.map[0].Length -le $where.x) {
        throw "Testing unown area XY"
    }
    if ($where.x -gt $state.end.x -or $where.x -lt $state.start.x -or $where.y -lt $state.start.y -or $where.y -gt $state.end.y) {
        throw "Testing outside of maze"
    }
    if ($where.x -ge $state.innerStart.x -and $where.x -le $state.innerEnd.x -and $where.y -ge $state.innerStart.y -and $where.y -le $state.innerEnd.y) {
        throw "Testing within inside of maze"
    }

    $paths = New-Object "System.Collections.ArrayList"
    if ($state.map[$where.y - 1][$where.x] -eq '.') {
        [void]$paths.Add(@{
            pos = @{ x = $where.x; y = $where.y - 1 }
            level = $level
        })
    }

    $posStr = "$($where.x)x$($where.y + 1)"
    if ($state.map[$where.y + 1][$where.x] -eq '.') {
        [void]$paths.Add(@{
            pos = @{ x = $where.x; y = $where.y + 1 }
            level = $level
        })
    }

    $posStr = "$($where.x - 1)x$($where.y)"
    if ($state.map[$where.y][$where.x - 1] -eq '.') {
        [void]$paths.Add(@{
            pos = @{ x = $where.x - 1; y = $where.y }
            level = $level
        })
    }

    $posStr = "$($where.x + 1)x$($where.y)"
    if ($state.map[$where.y][$where.x + 1] -eq '.') {
        [void]$paths.Add(@{
            pos = @{ x = $where.x + 1; y = $where.y }
            level = $level
        })
    }
    
    $posStr = "$($where.x)x$($where.y)"
    if ($state.portals[$posStr]) {
        if ($where.x -gt $state.start.x -and $where.x -lt $state.end.x -and $where.y -gt $state.start.y -and $where.y -lt $state.end.y) {
            [void]$paths.Add(@{
                pos = $state.portals[$posStr]
                level = $level + 1
            })
        } else {
            [void]$paths.Add(@{
                pos = $state.portals[$posStr]
                level = $level - 1
            })
        }
    }

    return $paths
}

function Traverse-Map {
    [CmdletBinding()]
    param($state)

    $visited = @{}
    $queue = New-Object "System.Collections.Queue"
    
    $queue.Enqueue(@{
        pos = $state.startPos
        distance = 0
    })

    while ($queue.Count -gt 0) {
        $act = $queue.Dequeue()
        $actStr = "$($act.pos.x)x$($act.pos.y)"

        if ($visited[$actStr]) {
            continue
        }

        $visited[$actStr] = $act.distance

        if ($act.pos.x -eq $state.endPos.x -and $act.pos.y -eq $state.endPos.y) {
            return $act.distance
        }

        Get-AccessibleSurrounding $state $act.pos | ForEach-Object {
            $queue.Enqueue(@{
                pos = $_.pos
                distance = $act.distance + 1
            })
        }
    }

    return -1
}

function Traverse-MapWithLevels {
    [CmdletBinding()]
    param($state)

    $visited = @{}
    $queue = New-Object "System.Collections.Queue"
    
    $queue.Enqueue(@{
        pos = $state.startPos
        distance = 0
        level = 0
    })

    while ($queue.Count -gt 0) {
        $act = $queue.Dequeue()

        $actStr = "$($act.pos.x)x$($act.pos.y)x$($act.level)"
        if ($visited[$actStr]) {
            # Write-Verbose "$(Get-Date -DisplayHint Time) Skipping (1) '$($act.pos.x)x$($act.pos.y)' Level: '$($act.level)', distance: '$($act.distance)', $actStr"
            continue
        }
        # $visited[$actStr] = @{ level = $act.level }

        # $actStr = "$($act.pos.x)x$($act.pos.y)"
        # if ($visited[$actStr]) {
        #     if ($visited[$actStr].level -le $act.level) {
        #         Write-Verbose "$(Get-Date -DisplayHint Time) Skipping (2) '$($act.pos.x)x$($act.pos.y)' Level: '$($act.level)', distance: '$($act.distance)', $($visited[$actStr].level) $actStr"
        #         continue
        #     # } elseif ($cacheLevel -lt 0 -and $act.level -le $cacheLevel) {
        #         # Write-Verbose "$(Get-Date -DisplayHint Time) Skipping (3) '$($act.pos.x)x$($act.pos.y)' Level: '$($act.level)', distance: '$($act.distance)', $cacheLevel $actStr"
        #         # continue
        #     }
        # }
        # $visited[$actStr] = @{ level = $act.level }
        $visited[$actStr] = $true

        Write-Verbose "$(Get-Date -DisplayHint Time) Look at '$($act.pos.x)x$($act.pos.y)' Level: '$($act.level)', distance: '$($act.distance)'"

        if ($act.level -eq 0 -and $act.pos.x -eq $state.endPos.x -and $act.pos.y -eq $state.endPos.y) {
            return $act.distance
        }

        Get-AccessibleSurrounding $state $act.pos $act.level | ForEach-Object {
            if ($_.level -ge 0) {
                $queue.Enqueue(@{
                    pos = $_.pos
                    distance = $act.distance + 1
                    level = $_.level
                })
            }
        }
    }

    return -1
}

function Get-MapEntry {
    [CmdletBinding()]
    param()

    $stringMap = 
"                                         W     P     B       T           P   K                                             
                                         L     W     W       Z           S   S                                             
  #######################################.#####.#####.#######.###########.###.###########################################  
  #.........#.............#...#.#...#.#.......#.#.#.........#.#...#...#.....#...#.............#...#...#.....#...#...#.#.#  
  #####.#######.#.#######.#.###.###.#.###.#.#.#.#.###.#######.#.#####.#####.#.###.#####.#.###.###.#.###.###.###.#.###.#.#  
  #...........#.#.#.....#.#.#.#.#.#.....#.#.#.#.....#.....#.....#...#.......#.........#.#.#...#.....#.....#.#...#.....#.#  
  #.###.#.#.#####.###.#.#.#.#.#.#.#.#.#.###.#####.###.#####.###.#.###.#.#####.#########.#.#######.###.###.#####.#.#.###.#  
  #.#...#.#.#.#...#...#...........#.#.#.......#...#.....#.#.#.....#...#.#...#.....#...#.#.#.....#...#.#.#.#...#...#.#...#  
  ###.###.###.#########.###.#.###.###.#####.#####.#.#.###.###.###.###.###.#.#.###.#.#######.#####.###.#.###.#####.#####.#  
  #.#.#.#.....#.#...#.#.#...#.#...........#.#.....#.#.#.#.#...#.#.#.......#.#.#...#.#.#...#...#.....#.#.#.......#...#...#  
  #.###.###.###.###.#.#####.###.#.###.###.#####.#####.#.#.###.#.###.#######.#.#####.#.###.#.#####.###.#.#.###.###.#####.#  
  #.#.#.......#...............#.#.#.....#.#.....#...#.......#...#.#.#.....#.#...#...#.#.....#...#.#...#...#...#...#.....#  
  #.#.#.###.###.#######.#.#.#########.#####.#.###.#####.#.###.###.#####.#.#.#.#####.#.#.#####.###.#.###.#########.###.###  
  #.......#...#.#.....#.#.#...#.#...#...#...#.....#.#...#.#.........#...#...#.#.................#.......#.#.#.......#...#  
  #.###.#.#########.#####.#####.###.#.#####.#.#.###.#.#.#######.#######.#.###.#####.###.#.#######.#######.#.#####.###.###  
  #...#.#...#.#.....#.......#.....#.#...#.#.#.#.....#.#.#.#.....#.#.....#.#...#.#.....#.#.#.#.....#.......#.#.....#...#.#  
  #.###.#####.#####.#.###.#######.#.###.#.###.#.###.#.###.###.#.#.###.###.###.#.###.#######.###.#####.#####.#.#######.#.#  
  #.#.#.#.....#.#...#.#.#.#.#.....#.#.....#...#.#...#.......#.#.#.#.#...#.#.#.#.....#...#.#.......#.#.#.#.#.#...#.....#.#  
  #.#.#####.###.###.###.###.###.#.#.#.###.#########.###.#######.#.#.###.###.#.#.#.#.#.###.###.###.#.#.#.#.#.#.###.#####.#  
  #.#...#.....#.........#.....#.#.#.....#.#...#.....#.......#.....#.......#.....#.#.#.#...#.....#.#.......#.#...#.#.....#  
  #.#.#####.#.#.###.###.#####.###.#.#.#####.#######.#.#######.#######.#.###.###.#####.#.###.#.#.#.###.#.###.#.###.#.###.#  
  #.#.......#.#.#.#.#.#...#.#.......#.#.#.....#.#...#.#.....#.....#...#.#...#...#...........#.#.#.#.#.#.#...#...#.....#.#  
  #.#.#.#########.###.#.###.#########.###.#.###.#.###.#.###.###.###.#.#####.###.#.#.#####.#.###.###.#.#####.#.#####.#####  
  #.#.#.#.......#.....#.#.....#...#.....#.#...#.#.#.#...#.#.#.....#.#.....#.#...#.#.#.#.#.#...#.....#...#...#.#...#.#.#.#  
  #.###.#.###########.#.#.###.###.#####.#.#.###.#.#.#.###.#.#.#####.###.#######.#.#.#.#.###########.#.#####.#.#.###.#.#.#  
  #...#.....#...#.#.#.....#.#.#.......#...#...#.....#...#...#.#...#.#.....#.#...#.#.#.....#.#.....#.#.......#...#...#...#  
  #.###.#.#####.#.#.###.#.#.#####.#.#######.###.###.#.###.###.#.#.###.#####.###.#.#.#.###.#.#####.#.#.#.###.#.#.###.#.###  
  #.#.#.#...#...#.......#.#.#.....#.........#.#.#...#.#...#.#...#...#.#...#...#.#.#...#.#.....#...#.#.#...#.#.#.........#  
  #.#.#.#######.#####.#.###.#######.#.#####.#.#.###.#.###.#.#######.#.#.#.#.###.#.#.#.#.#########.#####.#.#.#####.###.#.#  
  #.......#...#.#.#.#.#.....#.#...#.#.....#.#...#.#.#.#.....#.....#.#.#.#...#.....#.#.......#.#.......#.#.#.#...#.#...#.#  
  ###.#.###.###.#.#.#.#####.#.#.#.#.#####.#######.#.###.#.#.#.#.###.#.###.#####.###.###.###.#.#.#.#.#.#.#####.###.#####.#  
  #...#.#.........#.#.#...#.#...#...#.......#.........#.#.#.#.#.....#.........#.#...#...#...#...#.#.#.#...#.#...#...#.#.#  
  ###.#######.#.###.#.###.###########.###########.#########.#.#########.#########.###########.#####.#.###.#.#.###.###.###  
  #.......#.#.#...#.#...#.#.....#    N           E         R T         O         F        #.#...#...#.#.......#.#...#.#.#  
  #######.#.#####.#.#.###.#####.#    O           I         I T         O         Y        #.#.#########.###.###.#.###.#.#  
  #.#.......................#.#.#                                                         #.#.#.#.#...#.#.............#.#  
  #.#.#######.#####.#.#.#.###.#.#                                                         #.#.#.#.#.#.#####.#########.#.#  
IP..#.....#.....#...#.#.#...#.#.#                                                         #.#...#.#.#...#.#.........#...#  
  #.#####.#.#.#.###.#######.#.#.#                                                         #.#.###.###.###.#####.#.#####.#  
  #.....#.#.#.#.#...#.........#..YL                                                     IH........#.#...#.#...#.#...#.#..CI
  #.#####.#########.#########.#.#                                                         #.#####.#.#.#.#.#.#.###.###.#.#  
  #.......#...#.#.#.#.#.#.#.....#                                                         #.#.........#.....#.....#.#.#.#  
  #####.###.###.#.###.#.#.#######                                                         #####################.###.#.###  
  #.#.#.#.....#.....#.........#.#                                                         #.......#...........#.#.....#.#  
  #.#.###.#.#####.#######.###.#.#                                                         #.#####.#.#####.###.###.###.#.#  
UI....#.#.#.#.#.....#.#.#...#...#                                                         #.#.#.#.....#.....#.#.....#....TB
  #.###.###.#.###.###.#.###.###.#                                                         #.#.#.###.#########.###.#.#####  
  #.#...#.........#...#...#.#....UA                                                     UI........#...#...#.....#.#.....#  
  #.###.#####.###.#.###.#.#.#.###                                                         #.#######.###.#######.#######.#  
AU............#.........#...#....FQ                                                       #.#...#.#...#.#...............#  
  ###.#.#.#.#.#####.#############                                                         #####.#.#####.###.#.#.#.#####.#  
OO..#.#.#.#.#.#.#...#.......#...#                                                         #.#.....#.......#.#.#.#.#...#.#  
  #.#####.#####.###.#.#.#.###.#.#                                                         #.#.#.###.#.#.#######.###.###.#  
  #...#...#.#.#...#.#.#.#.....#.#                                                       TZ..#.#.#...#.#...#.#.#.#...#.#.#  
  #.###.#.#.#.#.#######.#######.#                                                         #.#.#.#.###.#####.#.#####.#.###  
  #...#.#.#.........#.....#.#....IP                                                       #...#.#...#...#.#.......#...#..UA
  ###.#####.#####.#####.###.###.#                                                         #####.###.#.###.###.#.###.#.#.#  
  #.#.......#.#.#.......#.....#.#                                                         #.#.#.....#.........#.....#...#  
  #.#########.#.#############.###                                                         #.#.#########################.#  
  #...........#.........#.......#                                                       RL..#.......#...............#.#.#  
  #.###.#.###.#.#.#####.#.#####.#                                                         #.#.#####.###.#.###.#.###.#.###  
FY..#...#.#.....#.#...#.....#....AU                                                       #.#...#.......#.#.#.#.#.#.....#  
  #.#########.#######.###########                                                         #.#.#######.#.###.#####.#.#.###  
  #...#.....#.#.#.......#.......#                                                         #.#.......#.#...#.#.....#.#.#..PH
  #####.#######.#.#.#.###.#####.#                                                         #.#.#.###########.###.#.#.###.#  
  #.#...#.#...#.#.#.#...#...#....PS                                                       #...#.#.....#.#...#.#.#.#.....#  
  #.###.#.#.###.###.#.#.#.#.#####                                                         #######.#####.###.#.###.#######  
  #...#...#...#...#.#.#.#.#.....#                                                         #.....................#.......#  
  #.###.###.#.#.#.#.###.###.#####                                                         #.#####.###.#####.###.#.###.###  
FQ..........#...#.....#.....#.#.#                                                       WL....#.....#.#.#...#...#.#...#..RL
  #####################.###.#.#.#                                                         #.#####.###.#.#######.#.#.###.#  
EI....#.#.#...#.......#.#.#.#.#.#                                                         #.#.#.....#.#...#.#.....#...#.#  
  ###.#.#.#.#.#.###.#####.###.#.#                                                         ###.#.#.#####.###.###.#####.#.#  
  #.........#.....#..............TP                                                       #.#.#.#.#.#.......#.#...#.....#  
  #.#.#########.#.#.#.#.###.###.#                                                         #.#.#####.###.#####.###########  
  #.#.#.....#...#.#.#.#.#...#.#.#                                                       CI....#.........#.#.....#...#.#.#  
  #####.###.###############.#.###                                                         ###.#.###.###.#.#.#.#####.#.#.#  
RI....#.#.#.#.#.#.....#.#.#.#...#                                                         #...#.#...#.......#...#.#.#.#.#  
  #.###.#.#.#.#.#####.#.#.#####.#                                                         #.###.###.#####.#.###.#.#.#.#.#  
  #...#...#...#.#.#.#.#...#.#...#                                                         #.....#.#...#...#.#...........#  
  #.#####.###.#.#.#.#.#.###.#.#.#                                                         #.#.###.#.#######.#.#.#.#.#####  
AA..........#.................#..KS                                                       #.#...#.....#.....#.#.#.#......ZS
  #.###.###.#.###.#.###.###.###.#                                                         #.#.###.#####.###.#.###.###.###  
  #.#.....#.#.#.#.#...#.#.....#.#                                                         #.#...#.....#.#...#.#...#.....#  
  #.###.#####.#.#####.#########.#    T             Z     P         Z       E     B P      ###.#########.#.###.#.#####.#.#  
  #...#.....#.....#...#.........#    B             E     H         S       M     W W      #.#...#...#...#.#...#.#.#...#.#  
  #.#####.#.#.#.#####.###.###.#.#####.#############.#####.#########.#######.#####.#.#######.#.###.###########.###.###.###  
  #.....#.#.#.#...#...#.#.#...#...#.#.....#.....#.....#.....#.....#.#.....#.#.....#.......#.........#...............#...#  
  #.###.#.#.#.#.###.#.#.###########.###.###.#.#.###.#######.#.###.#.#.###.#.#.###.###.#########.#.#########.###.#.#.#.###  
  #.#.#.#.#.#.#...#.#...#.#.#...#.#.#...#.#.#.#.#.....#.....#.#.....#...#.#.#.#...#.....#...#.#.#.....#.#.....#.#.#.#...#  
  #.#.#.###.#######.#####.#.###.#.#.#.###.###.#.#.#######.###.#########.#.#.#.#####.#.###.###.#.###.###.#.###.#####.#.#.#  
  #...#.#...#...#.#...#.............#...#.....#.#...#.....#...#.....#...#...#.#.#.#.#.....#.#.#...#.....#...#.#.#...#.#.#  
  #.#.###.#.#.###.#######.###.#####.#.#######.#.#.###.#.#####.#.###.#.#######.#.#.###.#####.#.#######.###.#####.#######.#  
  #.#.#.#.#...........#...#...#.#.........#...#.....#.#.#.#.....#.#.#.......#...#.....#.....#.#.#...#.#.........#.#.....#  
  #.#.#.#####.#.#########.###.#.#####.#####.#.###.###.###.###.#.###.#######.###.###.#####.###.#.#.#########.#.###.#######  
  #.#.....#...#.#.#.........#.#.....#.....#.#...#.#.........#.#.#.#.#...#...#.....#...............#.#.....#.#.........#.#  
  #.#.#######.###.###.#############.#.#######.###.#######.#.#.###.#.#.#.#.###.#.#######.#.#####.###.#.#######.#.#.#.###.#  
  #.#.....#.......#.#.#.#.#.#.........#.........#.#.......#.#...#.#.#.#.....#.#.#.#.....#...#...#.#.#.....#.#.#.#.#...#.#  
  #.###.#######.###.###.#.#.#.#.###.#.#######.#######.#.#.###.#.#.#.#.#.#####.###.#.#.###.#####.#.#.#.#####.###.#######.#  
  #.#...#.#.....#.#.#.........#.#.#.#...#.......#.....#.#...#.#.#...#.#.#.........#.#.#.#.....#.......#.#.....#.#...#.#.#  
  #.#####.###.#.#.#.###.#.###.###.###.#######.#####.#.###.#######.###.#.#.###.#.#####.#.#######.#.###.#.#####.###.###.#.#  
  #.#.#.......#.#.......#.#...#.......#.........#...#...#.#...#.....#.#.#.#.#.#.#...........#...#.#...........#.#.......#  
  #.#.###.#########.###.#######.###.#.#####.#.#######.#####.#.#.#.###.#####.#.#########.#####.#.#.#######.#.###.#.#####.#  
  #.....#...#.......#.....#.....#.#.#.#.#.#.#.#.#...........#.#.#...#...#.....#...#.#.#.#...#.#.#.....#...#.......#...#.#  
  ###.#######.#.#####.#######.###.###.#.#.###.#.#######.#########.###.#####.#.#.###.#.#.#.#####.###.#.###############.###  
  #.....#.....#.#.....#.#.....#.#...#.#.#...#...#.#.....#...#...#...#...#.#.#.#.#.#.#.......#.#...#.#.#.#.............#.#  
  #.#########.#.#######.#######.###.#.###.#.#.###.###.###.#.###.#.#####.#.###.#.#.#.###.#.###.#####.#.#.#.#####.#####.#.#  
  #...#.......#.#.#.#...........#.........#.#.#.#.#.......#.#...#...#.......#.........#.#.........#.#.#.#...#.#...#.....#  
  #.#.#####.#.#.#.#.###########.###.#.#####.#.#.#.#######.###.#.###.###.#########.#####.#####.#########.#.###.#.#######.#  
  #.#.#.....#.#.#...................#.#.#...#.#.#...#.......#.#.......#.....#.....#...#.#.#.....#...........#.#.#.#.#.#.#  
  ###.###.#.#######.#.###.#########.###.#.#.#.###.#.#.#########.###.#####.###.#######.#.#.#####.#.#.#.#######.#.#.#.#.###  
  #.....#.#...#.....#.#.........#.#.#.#...#.#.#...#.#.#.....#.....#.#.......#.......#.....#...#...#.#.........#.........#  
  ###.###.###.#####.#############.###.#.#.###.#.###.#.#.#.#####.#####.#######.#######.#######.###.###########.###.###.###  
  #.#.#.#...#...#...#.#.#...#...#.......#.#.....#.#.#...#.#.......#...#.....#.#...#...........#.#.........#.#.#...#.....#  
  #.#.#.#.#.#.#######.#.###.#.###.#.###.###.#####.#.#####.#####.#####.###.#.#.#.#######.#####.#.#.#.#######.#.###.#.#.#.#  
  #...#...#.#...#.................#.#.....#.#...........#...#.......#.....#.#.......#.......#...#.#.......#.....#.#.#.#.#  
  ###################################.#######.###########.#######.#.#######.###.#######.#################################  
                                     N       Y           T       Z E       Z   T       I                                   
                                     O       L           P       Z M       E   T       H                                   "

    return $stringMap
}

function Get-Part1Result {
    [CmdletBinding()]
    param()

    $stringMap = Get-MapEntry
    $state = Parse-Map $stringMap
    $result = Traverse-Map $state

    write-host "Result: $result"

    # 608 - correct
}

function Get-Part1Result {
    [CmdletBinding()]
    param()

    $stringMap = Get-MapEntry
    $state = Parse-Map $stringMap
    $result = Traverse-MapWithLevels $state

    write-host "Result: $result"

    # 6706 - correct
}
