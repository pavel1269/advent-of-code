
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

    Write-Verbose "$(Get-Date -DisplayHint Time) Map parsed"
    return $state
}

function Parse-Map4Way {
    [CmdletBinding()]
    param([string]$stringMap)

    $state = Parse-Map $stringMap
    $pos = $state.position
    $state.map[$pos.y - 1] = $state.map[$pos.y - 1].Remove($pos.x, 1).Insert($pos.x, "#")
    $state.map[$pos.y] = $state.map[$pos.y].Remove($pos.x - 1, 3).Insert($pos.x - 1, "###")
    $state.map[$pos.y + 1] = $state.map[$pos.y + 1].Remove($pos.x, 1).Insert($pos.x, "#")

    $state.position = @(
        @{ x = $pos.x - 1; y = $pos.y - 1 }
        @{ x = $pos.x + 1; y = $pos.y - 1 }
        @{ x = $pos.x - 1; y = $pos.y + 1 }
        @{ x = $pos.x + 1; y = $pos.y + 1 }
    )
    
    Write-Verbose "$(Get-Date -DisplayHint Time) Map for 4 robots parsed"
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

# function Get-NextCollactables {
#     [CmdletBinding()]
#     param([HashTable]$state, [Hashtable]$position, [string[]]$keys)

#     $queue = New-Object "System.Collections.Queue"
#     $visited = New-Object "System.Collections.ArrayList"
#     $queue.Enqueue(@{
#         pos = $position
#         distance = 0
#     })
#     $keyResult = New-Object "System.Collections.ArrayList"
#     while ($queue.Count -gt 0) {
#         $act = $queue.Dequeue()

#         # Write-Verbose "Looking at '$($act.pos.x)x$($act.pos.y)', distance: '$($act.distance)'"
#         $visited += "$($act.pos.x)x$($act.pos.y)"
#         $field = [char]($state.map[$act.pos.y][$act.pos.x])
#         if ($field -ge [char]'a' -and $field -le [char]'z' -and $field -notin $keys) {
#             $keyResult += @{
#                 distance = $act.distance
#                 pos = $act.pos
#                 key = $field
#             }
#         } elseif ($field -in $keys -or $field -in @('@', '.')) {
#             @(Get-AccessibleSurrounding $state $act.pos).GetEnumerator().Where({
#                 $visited -notcontains "$($_.pos.x)x$($_.pos.y)"
#             }).ForEach({
#                 $queue.Enqueue(@{
#                     pos = $_.pos
#                     distance = $act.distance + 1
#                 })
#             })
#         }
#     }

#     return $keyResult
# }

# function Collect-AllKeysStepsRec {
#     [CmdletBinding()]
#     param([Hashtable]$state, [Hashtable]$position, [string[]]$keys, [int]$distance)

#     $res = New-Object "System.Collections.ArrayList"
#     @(Get-NextCollactables $state $position $keys) | ForEach-Object {
#         $act = $_
#         $newKeys = (New-Object "System.Collections.ArrayList") + $keys + $act.key
#         $newDistance = $act.Distance + $distance
#         Write-Verbose "Can get '$($act.key)', '$($newKeys.Count)' keys, distance: '$newDistance'"
#         if ($newKeys.Count -eq $state.keys.Count) {
#             Write-Verbose "all keys now"
#             $res += @{
#                 keys = $newKeys
#                 distance = $newDistance
#             }
#         } else {
#             Write-Verbose "going for rest of the keys now"
#             $res += Collect-AllKeysStepsRec $state $act.pos $newKeys $newDistance
#             Write-Verbose "back"
#         }
#     }

#     return $res
# }

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

            if (-not ($keyResult.Where({
                [string]::Join("", ($_.keys | Sort-Object)) -eq [string]::Join("", (@($act.keys) | Sort-Object))
            }))) {
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
            @(Get-AccessibleSurrounding $state $act.pos).Where({
                $actPosKey = "$($_.pos.x)x$($_.pos.y)"
                $res = $visited[""] -notcontains $actPosKey

                if (-not $res -or $act.Doors.Count -eq 0) {
                    return $res
                }

                $iter = @()
                $res = $true
                $act.Doors.ForEach({
                    if ($res) {
                        $iter += $_
                        $iterKey = [string]::Join("", ($iter | Sort-Object))
                        Write-Verbose "Checking '$actPosKey', doors: '$iterKey', '$([string]::Join(`" `", $visited[$iterKey]))'"
                        if ($visited[$iterKey] -contains $actPosKey) {
                            $res = $false
                        }
                    }
                })
                
                Write-Verbose "$res"
                return $res
            }).ForEach({
                $queue.Enqueue(@{
                    pos = $_.pos
                    distance = $act.distance + 1
                    keys = @($act.keys)
                    doors = @($act.doors)
                })
            })
        }
    }

    return $keyResult
}

# function Collect-AllKeysStepsRec2 {
#     [CmdletBinding()]
#     param([Hashtable]$state, [Hashtable]$position, [Hashtable]$paths, [string[]]$keys, [int]$distance, [System.Collections.ArrayList]$best)

#     $pos = "$($position.x)x$($position.y)"
#     $viablePaths = @($paths[$pos] | Where-Object {
#         # Not blocked
#         $path = $_
#         $plausible = -not (($path.doors | Where-Object { $_ -notin $keys } | Select-Object -First 1).Count -gt 0)
#         return $plausible
#     } | Where-Object {
#         # Can actually get a new key
#         $path = $_
#         $plausible = ($path.keys | Where-Object {
#             $_ -notin $keys
#         } | Select-Object -First 1).Count -gt 0
#         return $plausible
#     } | Sort-Object {
#         $_.Distance
#     })

#     $totalKeys = $state.keys.Count
#     for ($index = 0; $index -lt $viablePaths.Count; $index++) {
#         $path = $viablePaths[$index]

#         $newKeys = (New-Object "System.Collections.ArrayList") + $keys + $path.keys | Select-Object -Unique
#         $newDistance = $path.Distance + $distance

#         Write-Verbose "Collected: '$([string]::Join('', $newKeys))' (count: $($newKeys.Count))(viable paths: $($viablePaths.Count))(cache: $($best.Count)), at: $($path.pos.x)x$($path.pos.y)"

#         if (($best | Where-Object {
#             $cache = $_
#             -not (($newKeys | Where-Object { $_ -notin $cache.keys } | Select-Object -First 1).Count -gt 0)
#         } | Where-Object {
#             $_.distance -le $newDistance
#         } | Select-Object -First 1).Count -gt 0) {
#             continue
#         }

#         $best += @{
#             distance = $newDistance
#             keys = $newKeys
#         }

#         if ($newKeys.Count -lt $totalKeys) {
#             $best = Collect-AllKeysStepsRec2 $state $path.pos $paths $newKeys $newDistance $best
#         }
#     }

#     return $best
# }

function Test-PathInCache {
    [CmdletBinding()]
    param([Hashtable]$cache, [Hashtable]$act, [int]$positions)

    $key = Convert-PosToStrKey $act.pos $positions

    if ($cache[$key]) {
        $entry = $cache[$key].Where({
            $_.distance -le $act.distance
        }).Where({
            $cacheEntry = $_
            -not (($act.keys.Where({ $_ -notin $cacheEntry.keys }) | Select-Object -First 1).Count -gt 0)
        }) | Select-Object -First 1
        return ($entry.Count -gt 0)
    } else {
        return $false
    }
    
    # $result = (($cache.Where({
    #     $res = $true
    #     for ($index = 0; $res -and $index -lt $positions; $index++) {
    #         $res = $_.pos[$index].x -eq $act.pos[$index].x -and $_.pos[$index].y -eq $act.pos[$index].y
    #     }
    #     return $res
    # }).Where({
    #     $cache = $_
    #     -not (($act.keys.Where({ $_ -notin $cache.keys }) | Select-Object -First 1).Count -gt 0)
    # }).Where({
    #     $_.distance -le $act.distance
    # }) | Select-Object -First 1).Count -gt 0)

    # return $result
}

function Convert-PosToStrKey {
    [CmdletBinding()]
    param([Hashtable[]]$position, [int]$positions)

    $key = new-object "system.text.stringbuilder"

    for ($index = 0; $index -lt $positions; $index++) {
        if ($index -gt 0) {
            [void]$key.Append(" ")
        }

        [void]$key.Append($position[$index].x)
        [void]$key.Append("x")
        [void]$key.Append($position[$index].y)
    }

    return ($key.ToString())
}

function Collect-AllKeysSteps3 {
    [CmdletBinding()]
    param([Hashtable]$state, [Hashtable]$paths)

    $best = @{}
    $queue = New-Object "System.Collections.ArrayList"
    [void]$queue.Add(@{
        pos = @($state.position)
        keys = @()
        distance = 0
    })

    $totalKeys = $state.keys.Count
    $index = 0
    $cacheCount = 0
    $positions = $queue[0].pos.Count
    do {
        $index++
        $act = $queue[0]
        $queue.RemoveAt(0)
        $keys = $act.keys

        $key = Convert-PosToStrKey $act.pos $positions
        # Write-Verbose "'$key' '$positions'"

        if (Test-PathInCache $best $act $positions) {
            # Write-Verbose "$((Get-Date -DisplayHint Time)) [$index] Skipping: '$(if ($keys.Count -gt 0) { [string]::Join('', $keys) })', at: $key, distance: $($act.distance)"
            continue
        }

        if ($VerbosePreference -eq "Continue") {
            $msg = new-object "system.text.stringbuilder"
            [void]$msg.Append((Get-Date -DisplayHint Time))
            [void]$msg.Append(" [")
            [void]$msg.Append($index)
            [void]$msg.Append("] Collected: '")
            if ($keys.Count -gt 0) {
                [void]$msg.Append([string]::Join('', $keys))
            }
            [void]$msg.Append("' (count: ")
            [void]$msg.Append($keys.Count)
            [void]$msg.Append(")(queue: ")
            [void]$msg.Append($queue.Count)
            [void]$msg.Append(")(cache: ")
            [void]$msg.Append($cacheCount)
            [void]$msg.Append("), at: ")
            [void]$msg.Append($key)
            [void]$msg.Append(", distance: ")
            [void]$msg.Append($act.distance)

            Write-Verbose $msg

            # Write-Verbose "$(Get-Date -DisplayHint Time) [$index] Collected: '$(if ($keys.Count -eq 0) { '' } else { [string]::Join('', $keys) })' (count: $($keys.Count))(queue: $($queue.Count))(cache: $($best.Count)), at: $($act.pos.x)x$($act.pos.y), distance: $($act.distance)"
        }

        if ($keys.Count -eq $totalKeys) {
            break
        }

        if ($best[$key].Count -lt 1) {
            $best[$key] = New-Object "System.Collections.ArrayList"
        }

        $cacheCount++
        [void]$best[$key].Add(@{
            pos = $act.pos
            keys = $keys
            distance = $act.distance
        })

        for ($indexp = 0; $indexp -lt $positions; $indexp++) {
            $pos = "$($act.pos[$indexp].x)x$($act.pos[$indexp].y)"
            $paths[$pos].Where({
                # Not blocked
                $path = $_
                $plausible = -not (($path.doors.Where({ $_ -notin $keys }) | Select-Object -First 1).Count -gt 0)
                return $plausible
            }).Where({
                # Can actually get a new key
                $path = $_
                $plausible = ($path.keys.Where({ $_ -notin $keys }) | Select-Object -First 1).Count -gt 0
                return $plausible
            }).ForEach({
                $path = $_
                
                $newKeys = @($keys) + $path.keys | Select-Object -Unique
                $newDistance = $act.Distance + $path.distance
                $newPos = New-Object "System.Collections.ArrayList"
                $newPos.AddRange($act.pos)
                $newPos[$indexp] = $path.pos

                $newCache = @{
                    pos = $newPos
                    keys = $newKeys
                    distance = $newDistance
                }

                # Write-Verbose "New in queue: '$(if ($newKeys.Count -eq 0) { '' } else { [string]::Join('', $newKeys) })'"

                if (-not (Test-PathInCache $best $newCache $positions)) {
                    [void]$queue.Add($newCache)
                }
            })
        }

        $queue = [System.Collections.ArrayList]@($queue | Sort-Object {
            $_.keys.Count
        } | Sort-Object {
            $_.distance
        })
        
    } while ($keys.Count -lt $totalKeys -and $queue.Count -gt 0)

    return $act.distance
}

function Get-GraphPaths {
    [CmdletBinding()]
    param([Hashtable]$state)

    $paths = @{}
    $count = $state.KeyPos.Count + 1
    $index = 1
    @($state.Position).ForEach({
        $paths["$($_.x)x$($_.y)"] = @(Get-AllCollactables $state $_ -Verbose:$false)
    })
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

    # $position = $state.Position

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

    $res = Collect-AllKeysSteps3 $state $paths

    return $res
}

function Collect-AllKeysSteps4Way {
    [CmdletBinding()]
    param([string]$stringMap)
    
    Write-Verbose "$(Get-Date -DisplayHint Time) Startring map preparation"
    $state = Parse-Map4Way $stringMap
    $paths = Get-GraphPaths $state
    
    $res = Collect-AllKeysSteps3 $state $paths

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

function Get-Part2Result {
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
    
    $res = Collect-AllKeysSteps4Way $stringMap

    Write-Host "Result: '$res'"

    # 2004 - correct
}
