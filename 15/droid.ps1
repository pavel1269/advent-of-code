
. ..\09\intComp.ps1

function Manual-RobotControl {
    [CmdletBinding()]
    param()

    # Initialization
    $state = Init
    Resolve-Map $state
    Out-Map $state

    while ($state.compInputs.OpCodeIndex -is [decimal]) {
        # Write-Host "Index: '$($compInputs.OpCodeIndex)', input: '$($hostInput)' ($([string]::Join(" ", $compInputs.InputParams)))"

        $state.compInputs.InputParams = @()
        Write-Host "Enter input: "
        $hostInput = Read-Host
        $state.compInputs.InputParams += [int]$hostInput

        Step-InMap $state
        Set-NewState $state
        
        Out-Map $state
    }
}

function Init {

    $OpCode = Load-IntCompProgram "./program.txt"

    $state = @{
        compInputs = @{
            OpCodeIndex = 0
            OpCodes     = $OpCode
            InputParams = @()
        }
        map        = @{
            "0" = @{
                "0" = '.'
            }
        }
        pos        = @{
            x = 0
            y = 0
        }
        min        = @{
            x = 0
            y = 0
        }
        max        = @{
            x = 0
            y = 0
        }
    }

    Step-InMap $state

    return $state
}

function Step-InMap {
    [CmdletBinding()]
    param($state)
    
    $state.compInputs.Remove("Outputs")
    $inputs = $state.compInputs
    $res = IntComp @inputs
    $res.InputParams = $state.compInputs.InputParams
    $state.compInputs = $res
}

function Resolve-Map {
    [CmdletBinding()]
    param($state)

    function Get-UnknownSurrounding {
        [CmdletBinding()]
        param($state, $where)

        if (-not ($state.map."$($where.y)")) {
            Out-Map $state
            throw "Testing unown area Y '$($where.y)'"
        }
        if (-not ($state.map."$($where.y)"."$($where.x)")) {
            Out-Map $state
            throw "Testing unown area X '$($where.x)' Y '$($where.y)'"
        }

        # north (1), south (2), west (3), and east (4)
        $unownSurrounding = @()
        
        if ($where.y -le $state.min.y) {
            $unownSurrounding += 1
        } elseif (-not ($state.map."$($where.y - 1)"."$($where.x)")) {
            $unownSurrounding += 1
        }

        if ($where.y -ge $state.max.y) {
            $unownSurrounding += 2
        } elseif (-not ($state.map."$($where.y + 1)"."$($where.x)")) {
            $unownSurrounding += 2
        }

        if ($where.x -le $state.min.x) {
            $unownSurrounding += 3
        } elseif (-not ($state.map."$($where.y)"."$($where.x - 1)")) {
            $unownSurrounding += 3
        }

        if ($where.x -ge $state.max.x) {
            $unownSurrounding += 4
        } elseif (-not ($state.map."$($where.y)"."$($where.x + 1)")) {
            $unownSurrounding += 4
        }
        
        return $unownSurrounding
    }
    
    function Get-PathToNearestUnknownSurrounding {
        [CmdletBinding()]
        param($state)

        $queue = New-Object "System.Collections.Queue"
        $visited = @("$($state.pos.x)x$($state.pos.y)")
        @(Get-AccessibleSurrounding $state $state.pos) | ForEach-Object {
            $queue.Enqueue($_)
        }

        while ($queue.Count -gt 0) {
            $act = $queue.Dequeue()
            $visited += "$($act.pos.x)x$($act.pos.y)"

            if ($state.map."$($act.pos.y)"."$($act.pos.x)" -eq 'O') {
                return "$($act.path)"
            }
            $paths = @(Get-UnknownSurrounding $state $act.pos)
            if ($paths.Count -gt 0) {
                return "$($act.path)"
            }
            @(Get-AccessibleSurrounding $state $act.pos) | Where-Object { $visited -notcontains "$($_.pos.x)x$($_.pos.y)" } | ForEach-Object {
                $_.path = "$($act.path)$($_.path)"
                $queue.Enqueue($_)
            }
        }
    }

    while ($true) {
        if ($state.map."$($state.pos.y)"."$($act.state.x)" -eq 'O') {
            return
        }

        while (($paths = @(Get-UnknownSurrounding $state $state.pos)).Count -gt 0) {
            $state.compInputs.InputParams = @([int]($paths[0]))
            Step-InMap $state
            Set-NewState $state
        }
    
        $res = Get-PathToNearestUnknownSurrounding $state
        if ($res) {
            for ($index = 0; $index -lt $res.length; $index++) {
                $state.compInputs.InputParams = @([int]("$($res[$index])"))
                Step-InMap $state
                Set-NewState $state
            }
        }
        else {
            return
        }
    }
}

function Get-AccessibleSurrounding {
    [CmdletBinding()]
    param($state, $where)

    if (-not ($state.map."$($where.y)")) {
        Out-Map $state
        throw "Testing unown area Y"
    }
    if (-not ($state.map."$($where.y)"."$($where.x)")) {
        Out-Map $state
        throw "Testing unown area XY"
    }

    # north (1), south (2), west (3), and east (4)
    $paths = @()
    $good = @('.', 'O')
    if ($where.y -gt $state.min.y -and ($state.map."$($where.y - 1)"."$($where.x)") -in $good) {
        $paths += @{
            path = 1
            pos = @{
                y = $where.y - 1
                x = $where.x
            }
        }
    }

    if ($where.y -lt $state.max.y -and ($state.map."$($where.y + 1)"."$($where.x)") -in $good) {
        $paths += @{
            path = 2
            pos = @{
                y = $where.y + 1
                x = $where.x
            }
        }
    }

    if ($where.x -gt $state.min.x -and ($state.map."$($where.y)"."$($where.x - 1)") -in $good) {
        $paths += @{
            path = 3
            pos = @{
                y = $where.y
                x = $where.x - 1
            }
        }
    }

    if ($where.x -lt $state.max.x -and ($state.map."$($where.y)"."$($where.x + 1)") -in $good) {
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

function Set-NewState {
    [CmdletBinding()]
    param($state)

    if ($state.compInputs.Outputs.Count -gt 0) {
        # north (1), south (2), west (3), and east (4)
        switch ($state.compInputs.InputParams[0]) {
            1 {
                $posChange = { param($pos); return @{ x = $pos.x; y = $pos.y - 1 } }
            }
            2 {
                $posChange = { param($pos); return @{ x = $pos.x; y = $pos.y + 1 } }
            }
            3 {
                $posChange = { param($pos); return @{ x = $pos.x - 1; y = $pos.y } }
            }
            4 {
                $posChange = { param($pos); return @{ x = $pos.x + 1; y = $pos.y } }
            }
            default { throw "sth went wrong" }
        }
        $newPos = . $posChange $state.pos

        if (-not ($state.map."$($newPos.y)")) {
            $state.map."$($newPos.y)" = @{ }
        }

        if ($newPos.y -lt $state.min.y) {
            $state.min.y = $newPos.y
        }
        elseif ($newPos.y -gt $state.max.y) {
            $state.max.y = $newPos.y
        }

        if ($newPos.x -lt $state.min.x) {
            $state.min.x = $newPos.x
        }
        elseif ($newPos.x -gt $state.max.x) {
            $state.max.x = $newPos.x
        }

        if ($state.compInputs.Outputs[0] -eq 0) {
            $state.map."$($newPos.y)"."$($newPos.x)" = '#'
        }
        elseif ($state.compInputs.Outputs[0] -eq 1) {
            $state.map."$($newPos.y)"."$($newPos.x)" = '.'
            $state.pos = $newPos
        }
        else {
            $state.map."$($newPos.y)"."$($newPos.x)" = 'O'
            $state.pos = $newPos
        }
    }
}

function Out-Map {
    [CmdletBinding()]
    param($state)

    for ($rowIndex = $state.min.y; $rowIndex -le $state.max.y; $rowIndex++) {
        $line = New-Object "System.Text.StringBuilder"
        for ($columnIndex = $state.min.x; $columnIndex -le $state.max.x; $columnIndex++) {
            if ($rowIndex -eq $state.pos.y -and $columnIndex -eq $state.pos.x) {
                $line.Append("D") | Out-Null
            }
            else {
                if ($state.map."$rowIndex"."$columnIndex") {
                    $line.Append($state.map."$rowIndex"."$columnIndex") | Out-Null
                }
                else {
                    $line.Append(" ") | Out-Null
                }
            }
        }
        write-host $line
    }
}

function Get-PathToOxygen {
    [CmdletBinding()]
    param($state, $from)

    $queue = New-Object "System.Collections.Queue"
    $visited = @("$($from.pos.x)x$($from.pos.y)")
    @(Get-AccessibleSurrounding $state $from) | ForEach-Object {
        $queue.Enqueue($_)
    }

    while ($queue.Count -gt 0) {
        $act = $queue.Dequeue()
        $visited += "$($act.pos.x)x$($act.pos.y)"

        if ($state.map."$($act.pos.y)"."$($act.pos.x)" -eq 'O') {
            return "$($act.path)"
        }
        @(Get-AccessibleSurrounding $state $act.pos) | Where-Object { $visited -notcontains "$($_.pos.x)x$($_.pos.y)" } | ForEach-Object {
            $_.path = "$($act.path)$($_.path)"
            $queue.Enqueue($_)
        }
    }

    throw "not found"
}

function Get-Part1Result {
    [CmdletBinding()]
    param()

    try {
        # Manual-RobotControl
        
        $state = Init
        Resolve-Map $state
        Write-Host "Map Explored"
        Out-Map $state
        # return $state
        $path = Get-PathToOxygen $state (@{ x = 0; y = 0 })
        Write-Host "Path: '$($path.length)' '$path'"
    }
    catch {
        "$($_ | Out-String)`n$($_.ScriptStackTrace | Out-String)" | Write-Host
    }

    # 248 - correct

    #  ####### ### ##### ######### ########### 
    # #.......#...#.....#.........#...........#
    # #.###.###.#.#.###.#.#####.#.###########.#
    # #.#...#...#...#.#.......#.#.#...#.......#
    # #.#.###.#######.#########.#.#.#.#.#####.#
    # #.#.#...#...#.......#.....#...#.#.#...#.#
    # #.#.#.###.#.#.#.#####.#.#######.#.#.#.#.#
    # #.#.#...#.#...#.....#.#.#...#.....#.#...#
    # #.#.###.###########.#.###.#.#######.#### 
    # #O#...#.#.........#.#.....#...#.#...#...#
    #  ##.#.#.#.#######.#.#########.#.#.###.## 
    # #...#.#...#.....#.#.....#.....#.#.#.....#
    # #.###.#####.###.#.###.###.#####.#.#####.#
    # #...#.#.......#.#...#...#.#.....#.#.....#
    #  ##.#.#.#####.#####.###.#.###.###.#.#### 
    # #...#.#.#...#...........#...#.....#.....#
    # #.#####.#.#############.###.###.###.###.#
    # #.........#...#.....#.....#...#.#.....#.#
    #  ######.###.#.#.###.#########.#.#####.#.#
    # #.....#.#...#...#.............#...#...#.#
    # #.###.###.#####.#################.#####.#
    # #.#.#.#...#...#.#.....#.....#...#.#.....#
    # #.#.#.#.#.#.#.###.#####.###.#.#.#.#.#### 
    # #.#.....#.#D#.....#.....#.#.#.#...#.....#
    # #.#######.#.#######.#####.#.#.#####.###.#
    # #.......#.#.#...#.....#.....#.#...#...#.#
    #  ######.###.#.###.###.#######.#.#.#####.#
    # #.....#.#...#.#...#.#...#.....#.#.....#.#
    # #.#.###.#.###.#.###.###.#.#####.#####.#.#
    # #.#...#...#...........#.#.......#.#...#.#
    # #.###.#####.###########.#########.#.###.#
    # #...#.....#...#.#.....#.......#.....#...#
    #  ##.###.#.###.#.#.###.#.#######.#####.##
    # #.#.#...#...#...#...#...#.....#...#.....#
    # #.#.#.#########.###.###.#.###.###.#####.#
    # #...#.........#.#...#...#.#.#.....#...#.#
    # #.## ########.#.#.###.###.#.#######.#.#.#
    # #...#.......#...#...#...#...#...#...#.#.#
    #  ##.###.###.#######.#######.#.#.#.###.#.#
    # #.......#.........#...........#...#.....#
    #  ####### ######### ########### ### #####

    # #   #   # # #   # #     #
    # #.....  #...#.....#.......
    #  . # . # . . .   . .     .
    #  .#...#...#...  #...    #.
    #  . . # . #   #     # ####.#
    #  . .#...            #.....#
    #  . . . #            #.#.####
    #  . .#...#           #.#.#...#
    #  D . # . #       #  #.###.#.##
    # #O#.   .#.........# #.....#...#
    #  # .   . .       .   ########.#
    # #...# #...#      .      #.....#
    #  . #   #     #   . #    #.####
    # #...# #.......  #...#   #.#
    #  # .   .     .   # .    #.##
    # #...#  .    #.......    #...#
    #  .     .   #########     ##.##
    # #.......  #...#.....#     #...#
    #  ######.###.#.#.###.#########.#
    # #.....#.#...#...#.............#
    # #.###.###.#####.##############
    # #.#.#.#...#...#.#.....#
    # #.#.#.#.#.#.#.###.####
    # #.#.....#.#.#.....#
    # #.#######.#.######
    # #.......#.#.#
    #  ######.###.#
    #       #.#...#
    #       #.#.##
    #       #...#
    #        ###
}

function Get-LongestPath {
    [CmdletBinding()]
    param($state, $from)

    $queue = New-Object "System.Collections.Queue"
    $visited = @("$($from.pos.x)x$($from.pos.y)")
    @(Get-AccessibleSurrounding $state $from) | ForEach-Object {
        $queue.Enqueue($_)
    }

    while ($queue.Count -gt 0) {
        $act = $queue.Dequeue()
        $visited += "$($act.pos.x)x$($act.pos.y)"

        @(Get-AccessibleSurrounding $state $act.pos) | Where-Object { $visited -notcontains "$($_.pos.x)x$($_.pos.y)" } | ForEach-Object {
            $_.path = "$($act.path)$($_.path)"
            $queue.Enqueue($_)
        }
    }

    return "$($act.path)"
}

function Get-Part2Result {
    [CmdletBinding()]
    param()

    try {
        $state = Init
        Resolve-Map $state
        Write-Host "Map Explored"
        Out-Map $state
        # return $state
        
        for ($rowIndex = $state.min.y; $rowIndex -le $state.max.y; $rowIndex++) {
            for ($columnIndex = $state.min.x; $columnIndex -le $state.max.x; $columnIndex++) {
                if ($state.map."$rowIndex"."$columnIndex" -eq 'O') {
                    $start = @{
                        x = $columnIndex
                        y = $rowIndex
                    }
                }
            }
        }
        if (-not $start) {
            throw "Start not found"
        }

        $path = Get-LongestPath $state $start
        Write-Host "Path: '$($path.length)' '$path'"
    }
    catch {
        "$($_ | Out-String)`n$($_.ScriptStackTrace | Out-String)" | Write-Host
    }

    # 382 - correct
    # 390 - too high. Curiously, it's the right answer for someone else; you might be logged in to the wrong account or just unlucky
}
