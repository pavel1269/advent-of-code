
. ..\09\intComp.ps1

function Manual-RobotControl {
    [CmdletBinding()]
    param()

    $OpCode = Load-IntCompProgram "./program.txt"

    $compInputs = @{
        OpCodeIndex = 0
        OpCodes = $OpCode
        InputParams = @()
    }
    
    $map = @{}
    $pos = @{
        x = 0
        y = 0
    }
    $map."$($pos.y)" = @{}
    $min = @{
        x = 0
        y = 0
    }
    $max = @{
        x = 0
        y = 0
    }

    do {
        # Write-Host "Index: '$($compInputs.OpCodeIndex)', input: '$($hostInput)' ($([string]::Join(" ", $compInputs.InputParams)))"

        $res = IntComp @compInputs

        if ($res.Outputs.Count -gt 0) {
            # Write-Host $res.Outputs[0]
            
            # north (1), south (2), west (3), and east (4)
            switch ($compInputs.InputParams[0]) {
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
            $newPos = . $posChange $pos

            if (-not $map."$($newPos.y)") {
                $map."$($newPos.y)" = @{}
            }

            if ($newPos.y -lt $min.y) {
                $min.y = $newPos.y
            }
            elseif ($newPos.y -gt $max.y) {
                $max.y = $newPos.y
            }

            if ($newPos.x -lt $min.x) {
                $min.x = $newPos.x
            }
            elseif ($newPos.x -gt $max.x) {
                $max.x = $newPos.x
            }

            if ($res.Outputs[0] -eq 0) {
                $map."$($newPos.y)"."$($newPos.x)" = '#'
            }
            elseif ($res.Outputs[0] -eq 1) {
                $map."$($newPos.y)"."$($newPos.x)" = '.'
                $pos = $newPos
            }
            else {
                $map."$($newPos.y)"."$($newPos.x)" = 'O'
                $pos = $newPos
            }
        }

        for ($rowIndex = $min.y; $rowIndex -le $max.y; $rowIndex++) {
            $line = New-Object "System.Text.StringBuilder"
            for ($columnIndex = $min.x; $columnIndex -le $max.x; $columnIndex++) {
                if ($rowIndex -eq $pos.y -and $columnIndex -eq $pos.x) {
                    $line.Append("D") | Out-Null
                }
                else {
                    if ($map."$rowIndex"."$columnIndex") {
                        $line.Append($map."$rowIndex"."$columnIndex") | Out-Null
                    }
                    else {
                        $line.Append(" ") | Out-Null
                    }
                }
            }
            write-host $line
        }

        if ($res.OpCodeIndex -is [decimal]) {
            $compInputs = $res
            $compInputs.Remove("Outputs")

            $compInputs.InputParams = @()
            $hostInput = Read-Host
            $compInputs.InputParams += [int]$hostInput
        }
    } while ($res.OpCodeIndex -is [decimal])
}

function Get-Part1Result {
    [CmdletBinding()]
    param()

    Manual-RobotControl
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
