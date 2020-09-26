
function Parse-Map {
    [CmdletBinding()]
    param([string]$stringMap)

    $state = @{
        map = $stringMap.Split()
        keys = @{}
        keyPos = @()
        doors = @{}
        position = $null
    }

    for ($indexY = 0; $indexY -lt $state.map.Count; $indexY++) {
        for ($indexX = 0; $indexX -lt $state.map[$indexY].Length; $indexX++) {
            $field = [char]($state.map[$indexY][$indexX])
            if ($field -eq '#' -or $field -eq '.') {
                continue
            }
            elseif ($field -eq '@') {
                if ($state.position) {
                    throw "duplicit position"
                }
                $state.position = @{
                    x = $indexX
                    y = $indexY
                }
            } elseif ($field -ge 'a' -and $field -le 'z') {
                if ($state.keys.$field) {
                    throw "duplicit key '$field'"
                }
                $pos = @{
                    x = $indexX
                    y = $indexY
                }
                $state.keys.$field = $pos
                $state.keyPos += $pos
            } elseif ($field -ge 'A' -and $field -le 'Z') {
                $field = [char]("$field".ToLower())
                if ($state.doors.$field) {
                    throw "duplicit door '$field'"
                }
                $state.doors.$field = @{
                    x = $indexX
                    y = $indexY
                }
            } else {
                throw "Could not determine '$field'"
            }
        }
    }

    return $state
}

function Get-AccessibleSurrounding {
    [CmdletBinding()]
    param($state, $where)

    if (-not ($state.map[$where.y]) -or $state.map.Count -le $where.y) {
        throw "Testing unown area Y"
    }
    if (-not ($state.map[$where.y][$where.x]) -or $state.map[0].Length -le $where.x) {
        throw "Testing unown area XY"
    }

    # north (1), south (2), west (3), and east (4)
    $paths = New-Object "System.Collections.ArrayList"
    if ($where.y -gt 0 -and (($state.map[$where.y - 1][$where.x]) -ne '#')) {
        $paths += @{
            path = 1
            pos = @{
                y = $where.y - 1
                x = $where.x
            }
        }
    }

    if ($where.y -lt $state.map.Count - 1 -and ($state.map[$where.y + 1][$where.x]) -ne '#') {
        $paths += @{
            path = 2
            pos = @{
                y = $where.y + 1
                x = $where.x
            }
        }
    }

    if ($where.x -gt 0 -and $state.map[$where.y][$where.x - 1] -ne '#') {
        $paths += @{
            path = 3
            pos = @{
                y = $where.y
                x = $where.x - 1
            }
        }
    }

    if ($where.x -lt $state.map[0].Length - 1 -and ($state.map[$where.y][$where.x + 1]) -ne '#') {
        $paths += @{
            path = 4
            pos = @{
                y = $where.y
                x = $where.x + 1
            }
        }
    }
    
    return $paths
}

function Get-NextCollactables {
    [CmdletBinding()]
    param([HashTable]$state, [Hashtable]$position, [string[]]$keys)

    $queue = New-Object "System.Collections.Queue"
    $visited = New-Object "System.Collections.ArrayList"
    $queue.Enqueue(@{
        pos = $position
        distance = 0
    })
    $keyResult = New-Object "System.Collections.ArrayList"
    while ($queue.Count -gt 0) {
        $act = $queue.Dequeue()

        # Write-Verbose "Looking at '$($act.pos.x)x$($act.pos.y)', distance: '$($act.distance)'"
        $visited += "$($act.pos.x)x$($act.pos.y)"
        $field = [char]($state.map[$act.pos.y][$act.pos.x])
        if ($field -ge [char]'a' -and $field -le [char]'z' -and $field -notin $keys) {
            $keyResult += @{
                distance = $act.distance
                pos = $act.pos
                key = $field
            }
        } elseif ($field -in $keys -or $field -in @('@', '.')) {
            @(Get-AccessibleSurrounding $state $act.pos).GetEnumerator().Where({
                $visited -notcontains "$($_.pos.x)x$($_.pos.y)"
            }).ForEach({
                $queue.Enqueue(@{
                    pos = $_.pos
                    distance = $act.distance + 1
                })
            })
        }
    }

    return $keyResult
}

function Collect-AllKeysStepsRec {
    [CmdletBinding()]
    param([Hashtable]$state, [Hashtable]$position, [string[]]$keys, [int]$distance)

    $res = New-Object "System.Collections.ArrayList"
    @(Get-NextCollactables $state $position $keys) | ForEach-Object {
        $act = $_
        $newKeys = (New-Object "System.Collections.ArrayList") + $keys + $act.key
        $newDistance = $act.Distance + $distance
        Write-Verbose "Can get '$($act.key)', '$($newKeys.Count)' keys, distance: '$newDistance'"
        if ($newKeys.Count -eq $state.keys.Count) {
            Write-Verbose "all keys now"
            $res += @{
                keys = $newKeys
                distance = $newDistance
            }
        } else {
            Write-Verbose "going for rest of the keys now"
            $res += Collect-AllKeysStepsRec $state $act.pos $newKeys $newDistance
            Write-Verbose "back"
        }
    }

    return $res
}

function Get-AllCollactables {
    [CmdletBinding()]
    param([HashTable]$state, [Hashtable]$position)

    $queue = New-Object "System.Collections.Queue"
    $visited = @{}
    $visited[""] = New-Object "System.Collections.ArrayList"
    $queue.Enqueue(@{
        pos = $position
        distance = 0
        doors = @()
        keys = @()
    })
    $keyResult = New-Object "System.Collections.ArrayList"
    while ($queue.Count -gt 0) {
        $act = $queue.Dequeue()

        $field = [char]($state.map[$act.pos.y][$act.pos.x])
        Write-Verbose "Looking at '$($act.pos.x)x$($act.pos.y)' '$field', distance: '$($act.distance)', keys: '$([string]::Join(`"`", ($act.keys)))', doors: '$([string]::Join(`"`", ($act.doors)))'"
        $walkable = $field -in @('@', '.')
        if ($field -ge [char]'a' -and $field -le [char]'z') {
            Write-Verbose "Found key '$field'"
            if ($field -notin $act.keys) {
                $act.keys += $field
            }
            $walkable = $true

            if (-not ($keyResult | Where-Object {
                [string]::Join("", ($_.keys | Sort-Object)) -eq [string]::Join("", (@($act.keys) | Sort-Object))
            })) {
                [void]$keyResult.Add(@{
                    distance = $act.distance
                    pos = $act.pos
                    keys = @($act.keys)
                    doors = @($act.doors)
                })
            }
        }
        if ($field -ge [char]'A' -and $field -le [char]'Z') {
            if ($field -notin $act.keys -and $field -notin $act.doors) {
                $act.doors += $field
            }
            $walkable = $true
        }

        $key = ""
        if ($act.doors.Count -gt 0) {
            $key = [string]::Join("", ($act.doors | Sort-Object))
        }
        if (-not $visited[$key]) {
            $visited[$key] = New-Object "System.Collections.ArrayList"
        }
        [void]$visited[$key].Add("$($act.pos.x)x$($act.pos.y)")

        if ($walkable) {
            @(Get-AccessibleSurrounding $state $act.pos) | Where-Object {
                $actPosKey = "$($_.pos.x)x$($_.pos.y)"
                $res = $visited[""] -notcontains $actPosKey

                if (-not $res -or $act.Doors.Count -eq 0) {
                    return $res
                }

                $iter = @()
                $res = $true
                $act.Doors | ForEach-Object {
                    if ($res) {
                        $iter += $_
                        $iterKey = [string]::Join("", ($iter | Sort-Object))
                        Write-Verbose "Checking '$actPosKey', doors: '$iterKey', '$([string]::Join(`" `", $visited[$iterKey]))'"
                        if ($visited[$iterKey] -contains $actPosKey) {
                            $res = $false
                        }
                    }
                }
                
                Write-Verbose "$res"
                return $res
            } | ForEach-Object {
                $queue.Enqueue(@{
                    pos = $_.pos
                    distance = $act.distance + 1
                    keys = @($act.keys)
                    doors = @($act.doors)
                })
            }
        }
    }

    return $keyResult
}

function Collect-AllKeysStepsRec2 {
    [CmdletBinding()]
    param([Hashtable]$state, [Hashtable]$position, [Hashtable]$paths, [string[]]$keys, [int]$distance, [System.Collections.ArrayList]$best)

    $pos = "$($position.x)x$($position.y)"
    $viablePaths = @($paths[$pos] | Where-Object {
        # Not blocked
        $path = $_
        $plausible = -not (($path.doors | Where-Object { $_ -notin $keys } | Select-Object -First 1).Count -gt 0)
        return $plausible
    } | Where-Object {
        # Can actually get a new key
        $path = $_
        $plausible = ($path.keys | Where-Object {
            $_ -notin $keys
        } | Select-Object -First 1).Count -gt 0
        return $plausible
    } | Sort-Object {
        $_.Distance
    })

    $totalKeys = $state.keys.Count
    for ($index = 0; $index -lt $viablePaths.Count; $index++) {
        $path = $viablePaths[$index]

        $newKeys = (New-Object "System.Collections.ArrayList") + $keys + $path.keys | Select-Object -Unique
        $newDistance = $path.Distance + $distance

        Write-Verbose "Collected: '$([string]::Join('', $newKeys))' (count: $($newKeys.Count))(viable paths: $($viablePaths.Count))(cache: $($best.Count)), at: $($path.pos.x)x$($path.pos.y)"

        if (($best | Where-Object {
            $cache = $_
            -not (($newKeys | Where-Object { $_ -notin $cache.keys } | Select-Object -First 1).Count -gt 0)
        } | Where-Object {
            $_.distance -le $newDistance
        } | Select-Object -First 1).Count -gt 0) {
            continue
        }

        $best += @{
            distance = $newDistance
            keys = $newKeys
        }

        if ($newKeys.Count -lt $totalKeys) {
            $best = Collect-AllKeysStepsRec2 $state $path.pos $paths $newKeys $newDistance $best
        }
    }

    return $best
}

function Collect-AllKeysSteps3 {
    [CmdletBinding()]
    param([Hashtable]$state, [Hashtable]$position, [Hashtable]$paths)

    $best = New-Object "System.Collections.ArrayList"

    $queue = New-Object "System.Collections.ArrayList"
    [void]$queue.Add(@{
        distance = 0
        keys = @()
        pos = $position
    })

    $totalKeys = $state.keys.Count
    $index = 0
    do {
        $index++
        $act = $queue[0]
        $queue.RemoveAt(0)
        $keys = $act.keys

        if (($best | Where-Object {
            $_.pos.x -eq $act.pos.x -and $_.pos.y -eq $act.pos.y
        } | Where-Object {
            $cache = $_
            -not (($keys | Where-Object { $_ -notin $cache.keys } | Select-Object -First 1).Count -gt 0)
        } | Where-Object {
            $_.distance -le $act.distance
        } | Select-Object -First 1).Count -gt 0) {
            continue
        }

        [void]$best.Add(@{
            pos = $act.pos
            keys = $keys
            distance = $act.distance
        })

        Write-Verbose "$(Get-Date -DisplayHint Time) [$index] Collected: '$(if ($keys.Count -eq 0) { '' } else { [string]::Join('', $keys) })' (count: $($keys.Count))(queue: $($queue.Count))(cache: $($best.Count)), at: $($act.pos.x)x$($act.pos.y), distance: $($act.distance)"

        if ($keys.Count -eq $totalKeys) {
            break
        }

        $pos = "$($act.pos.x)x$($act.pos.y)"
        $paths[$pos] | Where-Object {
            # Not blocked
            $path = $_
            $plausible = -not (($path.doors | Where-Object { $_ -notin $keys } | Select-Object -First 1).Count -gt 0)
            return $plausible
        } | Where-Object {
            # Can actually get a new key
            $path = $_
            $plausible = ($path.keys | Where-Object { $_ -notin $keys } | Select-Object -First 1).Count -gt 0
            return $plausible
        } | ForEach-Object {
            $path = $_
            
            $newKeys = @($keys) + $path.keys | Select-Object -Unique
            $newDistance = $act.Distance + $path.distance

            [void]$queue.Add(@{
                distance = $newDistance
                keys = $newKeys
                pos = $path.pos
            })
        }

        $queue = [System.Collections.ArrayList]@($queue | Sort-Object {
            $_.distance
        })
        
    } while ($keys.Count -lt $totalKeys -and $queue.Count -gt 0)

    return $act.distance
}

function Get-GraphPaths {
    [CmdletBinding()]
    param([Hashtable]$state)

    Write-Verbose "$(Get-Date -DisplayHint Time) Map parsed"
    $paths = @{}
    $count = $state.KeyPos.Count + 1
    $index = 1
    $paths["$($state.Position.x)x$($state.Position.y)"] = @(Get-AllCollactables $state $state.position -Verbose:$false)
    Write-Verbose "$(Get-Date -DisplayHint Time)    [$index/$count] Analyzed"
    $state.KeyPos | ForEach-Object {
        $key = $_
        $paths["$($key.x)x$($key.y)"] = @(Get-AllCollactables $state $key -Verbose:$false)
        $index++
        Write-Verbose "$(Get-Date -DisplayHint Time)    [$index/$count] Analyzed"
    }
    Write-Verbose "$(Get-Date -DisplayHint Time) All paths gathered"

    return $paths
}

function Collect-AllKeysSteps {
    [CmdletBinding()]
    param([string]$stringMap)

    Write-Verbose "$(Get-Date -DisplayHint Time) Startring map preparation"
    $state = Parse-Map $stringMap
    $position = $state.Position

    # Brute force with paths
    # $keys = @()
    # $paths = @(Collect-AllKeysStepsRec $state $state.position @() 0)
    # Write-Verbose "All paths resolved"
    # $paths = $paths | Sort-Object { $_.Distance }
    # $winner = $paths | Select-Object -First 1
    # $winner = $winner.Distance

    $paths = Get-GraphPaths $state

    # $best = Collect-AllKeysStepsRec2 $state $state.position $paths @() 0 @()
    # $totalKeys = $state.keys.Count
    # $res = $best | Where-Object {
    #     $_.Keys.Count -eq $totalKeys
    # } | Sort-Object {
    #     $_.Distance
    # } | Select-Object -First 1
    # $res = $res.Distance

    $res = Collect-AllKeysSteps3 $state $state.position $paths

    return $res
}

function Get-Part1Result {
    [CmdletBinding()]
    param()

    $stringMap =
"#################################################################################
#.............#...#...O.#.#...........#.#...#.........#.......#.....#.......#.Z.#
#####.#######.#H#.#.###.#.#.#####.###.#.#.###.#####.#.#.#####.#.###.#.###.###.#.#
#.....#.#...#.#.#...#.#.#.#.#.#...#.....#.#...#...#.#.#...#w..#...#...#.#.....#.#
#.#####.#.#.#.#.#####.#.#.#B#.#.#########.#.###.#.#.#####.#.#####.#####.#######.#
#.#...#...#.....#...#.#.#.....#.........#.#.#...#.#.....#.#...#..y..#...#.....#.#
#.#.#.#.#########.#.#.#.###########.###.#.#.#.#######.###.###.#####.#.#.#.###.#.#
#.#.#...#.........#.#.......#.....#...#.#.#.#.#.....#.....#.#.#...#...#.#.#...#.#
#.###.###.#########.#######.#.###.#####.#.#.#.#.###.#######.#.#.#.#####.#.#.###.#
#...#.#.....#x....#.#..f..#...#...#...#.#.#.#.....#.....#i..#...#.#...#.#.#.#...#
#.#.#.#.#####.###.#.#A###.#####.###.#.#.#.#.#########.#.#.#.#####.#.#.###.#.#.#.#
#.#.#.#.#...#.#.#.#...#.#.#...#.#.T.#...#.#...#.....#.#...#.#.#..e#.#.....#.#.#.#
###.###.#X#.#.#.#.#.###.#.###.#.###.###.#.###.#.###.#.#####.#.#.###.#######.#.#.#
#...#d..#.#...#.#.#.....#...#.#...#...#.#...#.#.#.....#.......#.#.....#.#...#.#.#
#.###.###.#####R#.#####.###.#.###.###.#.#.#.#.#.###########.###.#.###.#.#.#####.#
#...#.#.#...#.......#...#...#...#.....#.#.#...#.....#.....#.#...#...#.#...#...#.#
###E#.#.###.#######.#####.###.#.#######.#.#####.###N#.###.###.###.###.#.###.#.#.#
#.#...#...#..c..#...#r..F.#...#.....#.#.#...#.#.#.#.#...#.#...#...#...#.....#.#.#
#.#####.#######.###.#.#######.#####.#.#.###.#.#.#.#.###.#.#.###.###.#########.#.#
#z....#.......#...#...#.....#s#.......#.#.#.#.....#...#.#...#v..#.#...#.....#...#
#.#.###.#.#######.#####G###.#.#######.#.#.#.#####.#.###.#######.#.###.#.#######.#
#.#.#...#.......#...#...#.#.#...#...#.#.#.#.....#.#...#.D.....#.....#.#.......#.#
###.#.#########.###C#####.#.#.#.#.#.###.#.#####.#####.###############.#.#####.#.#
#...#...#.....#...#....g....#.#.#.#.....#.....#.....#...........#.....#.....#...#
#.#####.#.###.#.#############.#.#.#######.#########.#.#######.#.#.#########.#####
#.....#...#...#.#.........#...#.#.#.....#.........#.#.#.....#.#...#.......#.#..u#
#Q#.#######.###.#.#.#.#####.###.#.#####.#.#######.#.###.###.#.#####.###.#.#.#.###
#.#.......#.#...#.#.#.#...#.#.#...#...#.#.#.....#.#.#...#.#.#...#.....#.#.#.#...#
#.#####.###.#.###.#.###.#.#.#.#####.#.#.#.#.###.#.#.#.###.#.#.#.#######.#.#.#.#.#
#...#...#...#.....#.....#...#.......#...#.#.#.#.K.#...#.#...#.#.#.......#.#.#.#.#
###.#.###.#####################.###.#####.#.#.#.#######.#.#####.#.#####.###.#.#.#
#...#.....#...#.....#.........#...#.#...#.#.#...#.......#...#...#...#...#...#.#.#
#.#########.###.###.#.###.###.###.#.#.#.###.#####.#.###.###.#.###.#.#.###.#####.#
#.#.....#.........#...#...#.#.#.#.#.#.#.#...#.....#.#l..#.#.#...#.#.#...#...#...#
#.###.#.###.###########.###.#.#.#.#.#.#.#.###.#####.#.###.#.###.#.#.#.#####.#.#.#
#.#...#...#...#...#.....#...#.#...#...#.#...#.#.#...#.....#.....#.#.#.#...#.#.#.#
#.#.#####.#.###.#.#.#####.###.#########.#.#.#.#.#.#################.###.#.#P#.#.#
#.#.#.....#.#...#.#...#.#...#.#.....#...#.#.#.#.#.#.................#...#...#.#.#
#.#.#.#######.###.###.#.#.#.#.#.###.#.#.#.#.#.#.#.#.#########.#######.#######.#.#
#.U.#p........#....j..#...#.....#.....#...#.....#...........#...........J.....#.#
#######################################.@.#######################################
#...#.....#...#.......#.........#.........#.....#.....#.....#.....#.............#
#.#.#.###.###.#.#L###.#.#####.###.#.###.#.#.#.###.#.###.#.#.#.#.#.#.#########.#.#
#.#...#.....#...#...#.#.#...#.....#...#.#...#.....#.....#.#.#.#.#.#.#.......#q#.#
#.#.#######.#.#####.#.###.#.#########.#.#.###############.###.#.#.###.#####.###.#
#.#.#.....#.#...#...#.....#.#.........#.#.#.....#.....#...#...#.#.....#...#...#.#
#.#.#.###.#.#####.#########.#.#########.#.#.#.#.#.###.#.###.###.#.#####.#.###.#.#
#.#.#...#.#.....#.....#...#...#.......#.#.#.#.#...#...#.#.....#.#.#...#.#...#...#
#.#####.#.#####.#####.#.#######.###.###.#.#.#.#####.###.#.#####.#.#.#.#.###.###.#
#.....#.#...#...#...#.#...........#...#.#k#.#...#.....#.#.#.....#.#.#.#...#...#.#
#####.#.###.###.#.#.#.#.#############.#.#.#.###.#.#####.#.#.#######.#.###.#.###.#
#.....#...#...#...#...#.#...........#...#.#...#.#...#...#.#...#...#.#...#.#.....#
#.#######.###.#####.#####.#########.#.###.#####.###.#.###.###.#.#.#.###.#.#######
#.......#...#.....#.#.....#.......#.#...#.......#.#.#.#...#.#...#.....#.#.#.....#
#.###.#####.#####.###.#####.#.#.###.#############.#.#.#.###.###########.#.#.###.#
#...#.#.........#...#.#.....#.#.#...#...#...#.....#.#...#...#.......#...#.#.#m..#
#.###.#.###########.#.#######.###.###.#.#.#.#.#.###.#####.#.#######.#.###.#.#.#.#
#.#...#...#.#.......#...#...#.W...#...#.#.#.#.#.....#.....#...#.....#...#.#.#.#.#
###.#.###.#.#.#########.#.#.#####.###.#.#.###.#######.#######.#.###.###.#.###.#.#
#...#...#...#...#.....#...#.....#...#.#.#.#...#.........#.......#.#.#...#.....#.#
#.#########.###.#.#.###########.###.#.#.#.#.###.#######.#.#######.#.#.#########.#
#.........#.#...#.#.#....a......#.#...#.#.#...#.#.......#.#...#.....#.#...#.....#
#.#######.#.#.###.#.#############.#####.#.###.#.#.#######.#.###.#####.###.#.#####
#.#.....#...#.#...#.....#...#.......#h..#.....#.#.#.......#.#...#...#...#.#.#...#
#.###.#.#####.#.#####.#.#.#.#.#####.#.###.#######.#######.#.#####.#.###.#.#.#.#.#
#...#.#...#...#.#...#.#.#.#..o#...#...#.#.......#.......#.#.....#.#.#...#.....#.#
###.#####.#.#.#.###.#.###.#######.#####.#######.#.#####.#######.#.#.#.###########
#.#...#...#.#.#.....#.....#.#.......#...#...#...#.....#.......#.#.#.#...#.......#
#.###.#.###.#.#####.#######.#.#I###.###.#.#.#.###########.###.#.#.#.###.#.#####.#
#...#...#...#.....#.#.#.....#.#...#t..#.#.#...#.........#.#.#.#...#...#...#.....#
#.#####.#.#########.#.#.###.#.###.###.#.#.#####.#######.#.#.#.#######.#####.#####
#...M...#.......#...#...#.#...#.#...#...#.......#.....#.#.#.........#.....#.#...#
#.#############.#.###.###.#####.###.#############.#####.#.#.#######.#####.#.###.#
#.#.....Y...#.#.#.#.#.#..b..#...#.#.#...#...#.........#.#.#.#...#.#.#...#...#...#
#.#.#######.#.#.#.#.#.###.#.#.#.#.#.#.#.#.#.#.#######.#.#.###.#.#.#.#.#.#####.#.#
#...#n....#.#.#...#.......#...#...#...#.#.#.#.#.....#...#.....#.#...#.#.......#.#
#####.###.#.#.###################.#####.#.###.#.###.#####V#####.#.###.###.#####.#
#.....#...#...#.....#...#...#.....#.....#.....#...#...........#.#...#.#...#.....#
#.###########.###.#.#.#.#.#.#######.###.#.#####################.###.###.###.#####
#.................#...#...#.........#.S.#.......................#.......#.......#
#################################################################################"
    
    $res = Collect-AllKeysSteps $stringMap

    Write-Host "Result: '$res'"

    # 3918 - correct
}
