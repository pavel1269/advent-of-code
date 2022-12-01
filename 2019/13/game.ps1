
. ..\09\intComp.ps1

function Get-BlockCount {
    [CmdletBinding()]
    param($res)

    $count = 0
    for ($index = 0; $index -lt $res.Outputs.length; $index += 3) {
        $x = $res.Outputs[$index]
        $y = $res.Outputs[$index + 1]
        $type = $res.Outputs[$index + 2]

        if (($x -ne -1 -or $y -ne 0) -and $type -eq 2) {
            $count++
        }
    }

    return $count
}

function Get-Part1Result {
    [CmdletBinding()]
    param()
    
    $OpCode = Load-IntCompProgram "./program.txt"

    $CompInputs = @{
        OpCodeIndex = 0
        OpCodes     = $OpCode
        InputParams = @()
    }

    $res = IntComp @CompInputs
    $count = Get-BlockCount $res
    Write-Host "Result: '$count'"

    # 270 - correct
}

function Get-Part2Result {
    [CmdletBinding()]
    param()

    $OpCode = Load-IntCompProgram "./program.txt"
    $OpCode["0"] = 2

    $CompInputs = @{
        OpCodeIndex = 0
        OpCodes     = $OpCode
        InputParams = @()
    }

    $res = IntComp @CompInputs
    $blocks = Get-BlockCount $res
    $score = -1
    while ($blocks -ge 0) {
        $paddleX = -1
        $paddleY = -1
        $ballX = -1
        $ballY = -1

        for ($index = 0; $index -lt $res.Outputs.length; $index += 3) {
            $x = $res.Outputs[$index]
            $y = $res.Outputs[$index + 1]
            $type = $res.Outputs[$index + 2]

            if ($x -eq -1 -and $y -eq 0) {
                if ($type -gt 0 -and $type -gt $score) {
                    $blocks--
                }
                $score = $type
            } elseif ($type -eq 4) {
                $ballX = $x
                $ballY = $y
            } elseif ($type -eq 3) {
                $paddleX = $x
                $paddleY = $y
            }
        }

        $movement = 0
        if ($ballX -gt $paddleX) {
            $movement = 1
        } elseif ($ballX -lt $paddleX) {
            $movement = -1
        }

        Write-Host "Blocks: '$blocks' '$(Get-BlockCount $res)'($($res.Outputs.length)), ball: '[$ballX,$ballY]', paddle: '[$paddleX,$paddleY]', score: '$score', input: '$movement'"
        $CompInputs = $res
        $CompInputs.InputParams = @($movement)
        $CompInputs.Remove("Outputs")
        $res = IntComp @CompInputs
    }

    Write-Host "Result score: '$score'"

    # 12535 - correct
}
