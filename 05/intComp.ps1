
$ErrorActionPreference = "Stop"

function IntComp {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [int[]]
        $OpCodes,

        [int]
        $InputParam = -1
    )

    function Get-Op {
        param($OpCodes, $index, $mode)

        $op = $OpCodes[$index]
        if ($mode -eq 0) {
            $opVal = $OpCodes[$op]
        }
        else {
            $opVal = $op
        }

        return $opVal
    }

    $outputs = @()
    for ($index = 0; $index -lt $OpCodes.Count; $index += $operands) {
        $operands = 1
        $func = $OpCodes[$index]

        $tmp = [Math]::Floor($func / 100)
        $modeOp1 = $tmp -band 1
        $tmp = [Math]::Floor($tmp / 10)
        $modeOp2 = $tmp -band 1
        $tmp = [Math]::Floor($tmp / 10)
        $modeOp3 = $tmp -band 1

        Write-Verbose "Instruction 1: '$($func % 100)'('$func') (op1 mode: '$modeOp1', op2 mode: '$modeOp2', op3 mode: '$modeOp3')"

        switch ($func % 100) {
            1 {
                if ($modeOp3 -ne 0) {
                    throw "Output in immediate mode"
                }

                $operands = 4
                $op1Val = Get-Op $OpCodes ($index + 1) $modeOp1
                $op2Val = Get-Op $OpCodes ($index + 2) $modeOp2

                $op3 = $OpCodes[$index + 3]
                $OpCodes[$op3] = $op1Val + $op2Val
                Write-Verbose "[$op3] = '$op1Val' + '$op2Val' = '$($op1Val + $op2Val)'"
            }
            2 {
                if ($modeOp3 -ne 0) {
                    throw "Output in immediate mode"
                }

                $operands = 4
                $op1Val = Get-Op $OpCodes ($index + 1) $modeOp1
                $op2Val = Get-Op $OpCodes ($index + 2) $modeOp2

                $op3 = $OpCodes[$index + 3]
                $OpCodes[$op3] = $op1Val * $op2Val
                Write-Verbose "[$op3] = '$op1Val' * '$op2Val' = '$($op1Val * $op2Val)'"
            }
            3 {
                if ($modeOp1 -ne 0) {
                    throw "Input in immediate mode"
                }

                $operands = 2
                $input = $OpCodes[$index + 1]
                $OpCodes[$input] = $InputParam
                Write-Verbose "[$input] = '$InputParam'"
            }
            4 {
                $operands = 2
                $outputVal = Get-Op $OpCodes ($index + 1) $modeOp1
                $outputs += $outputVal
                Write-Verbose "Output: '$outputVal'"
            }
            5 {
                $operands = 3
                $op1Val = Get-Op $OpCodes ($index + 1) $modeOp1
                $op2Val = Get-Op $OpCodes ($index + 2) $modeOp2

                if ($op1Val -ne 0) {
                    $operands = 0
                    $index = $op2Val
                    Write-Verbose "Jump to '$index'"
                }
            }
            6 {
                $operands = 3
                $op1Val = Get-Op $OpCodes ($index + 1) $modeOp1
                $op2Val = Get-Op $OpCodes ($index + 2) $modeOp2

                if ($op1Val -eq 0) {
                    $operands = 0
                    $index = $op2Val
                    Write-Verbose "Jump to '$index'"
                }
            }
            7 {
                if ($modeOp3 -ne 0) {
                    throw "Output in immediate mode"
                }

                $operands = 4
                $op1Val = Get-Op $OpCodes ($index + 1) $modeOp1
                $op2Val = Get-Op $OpCodes ($index + 2) $modeOp2
                $op3 = $OpCodes[$index + 3]

                if ($op1Val -lt $op2Val) {
                    $OpCodes[$op3] = 1
                }
                else {
                    $OpCodes[$op3] = 0
                }
                Write-Verbose "[$op3] = '$($OpCodes[$op3])'"
            }
            8 {
                if ($modeOp3 -ne 0) {
                    throw "Output in immediate mode"
                }

                $operands = 4
                $op1Val = Get-Op $OpCodes ($index + 1) $modeOp1
                $op2Val = Get-Op $OpCodes ($index + 2) $modeOp2
                $op3 = $OpCodes[$index + 3]

                if ($op1Val -eq $op2Val) {
                    $OpCodes[$op3] = 1
                }
                else {
                    $OpCodes[$op3] = 0
                }
                Write-Verbose "[$op3] = '$($OpCodes[$op3])'"
            }
            99 {
                $operands = 1
                $index = $OpCodes.Count
                return @{
                    OpCodes = $OpCodes
                    Outputs = $outputs
                }
            }
            default {
                throw "Op code '$func' encountered"
            }
        }
    }

    throw "Missing halt"
}

function Get-Result {
    $code = @(3,225,1,225,6,6,1100,1,238,225,104,0,1101,48,82,225,102,59,84,224,1001,224,-944,224,4,224,102,8,223,223,101,6,224,224,1,223,224,223,1101,92,58,224,101,-150,224,224,4,224,102,8,223,223,1001,224,3,224,1,224,223,223,1102,10,89,224,101,-890,224,224,4,224,1002,223,8,223,1001,224,5,224,1,224,223,223,1101,29,16,225,101,23,110,224,1001,224,-95,224,4,224,102,8,223,223,1001,224,3,224,1,223,224,223,1102,75,72,225,1102,51,8,225,1102,26,16,225,1102,8,49,225,1001,122,64,224,1001,224,-113,224,4,224,102,8,223,223,1001,224,3,224,1,224,223,223,1102,55,72,225,1002,174,28,224,101,-896,224,224,4,224,1002,223,8,223,101,4,224,224,1,224,223,223,1102,57,32,225,2,113,117,224,101,-1326,224,224,4,224,102,8,223,223,101,5,224,224,1,223,224,223,1,148,13,224,101,-120,224,224,4,224,1002,223,8,223,101,7,224,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,8,677,226,224,102,2,223,223,1006,224,329,101,1,223,223,107,677,677,224,1002,223,2,223,1006,224,344,101,1,223,223,8,226,677,224,102,2,223,223,1006,224,359,101,1,223,223,107,226,226,224,102,2,223,223,1005,224,374,1001,223,1,223,1108,677,226,224,1002,223,2,223,1006,224,389,101,1,223,223,107,677,226,224,102,2,223,223,1006,224,404,1001,223,1,223,1107,226,677,224,1002,223,2,223,1006,224,419,1001,223,1,223,108,677,677,224,102,2,223,223,1005,224,434,1001,223,1,223,1008,677,226,224,1002,223,2,223,1006,224,449,1001,223,1,223,7,226,677,224,1002,223,2,223,1006,224,464,1001,223,1,223,1007,677,677,224,102,2,223,223,1005,224,479,1001,223,1,223,1007,226,226,224,1002,223,2,223,1005,224,494,1001,223,1,223,108,226,226,224,1002,223,2,223,1005,224,509,1001,223,1,223,1007,226,677,224,1002,223,2,223,1006,224,524,101,1,223,223,1107,677,677,224,102,2,223,223,1005,224,539,101,1,223,223,1107,677,226,224,102,2,223,223,1005,224,554,1001,223,1,223,108,677,226,224,1002,223,2,223,1006,224,569,1001,223,1,223,1108,226,677,224,1002,223,2,223,1006,224,584,101,1,223,223,8,677,677,224,1002,223,2,223,1006,224,599,1001,223,1,223,1008,226,226,224,102,2,223,223,1006,224,614,101,1,223,223,7,677,677,224,1002,223,2,223,1006,224,629,101,1,223,223,1008,677,677,224,102,2,223,223,1005,224,644,101,1,223,223,7,677,226,224,1002,223,2,223,1005,224,659,101,1,223,223,1108,226,226,224,102,2,223,223,1006,224,674,1001,223,1,223,4,223,99,226)
    $res = IntComp $code
    $res.Outputs

    # correct - 13547311

}

function Get-Result-part2 {
    $code = @(3,225,1,225,6,6,1100,1,238,225,104,0,1101,48,82,225,102,59,84,224,1001,224,-944,224,4,224,102,8,223,223,101,6,224,224,1,223,224,223,1101,92,58,224,101,-150,224,224,4,224,102,8,223,223,1001,224,3,224,1,224,223,223,1102,10,89,224,101,-890,224,224,4,224,1002,223,8,223,1001,224,5,224,1,224,223,223,1101,29,16,225,101,23,110,224,1001,224,-95,224,4,224,102,8,223,223,1001,224,3,224,1,223,224,223,1102,75,72,225,1102,51,8,225,1102,26,16,225,1102,8,49,225,1001,122,64,224,1001,224,-113,224,4,224,102,8,223,223,1001,224,3,224,1,224,223,223,1102,55,72,225,1002,174,28,224,101,-896,224,224,4,224,1002,223,8,223,101,4,224,224,1,224,223,223,1102,57,32,225,2,113,117,224,101,-1326,224,224,4,224,102,8,223,223,101,5,224,224,1,223,224,223,1,148,13,224,101,-120,224,224,4,224,1002,223,8,223,101,7,224,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,8,677,226,224,102,2,223,223,1006,224,329,101,1,223,223,107,677,677,224,1002,223,2,223,1006,224,344,101,1,223,223,8,226,677,224,102,2,223,223,1006,224,359,101,1,223,223,107,226,226,224,102,2,223,223,1005,224,374,1001,223,1,223,1108,677,226,224,1002,223,2,223,1006,224,389,101,1,223,223,107,677,226,224,102,2,223,223,1006,224,404,1001,223,1,223,1107,226,677,224,1002,223,2,223,1006,224,419,1001,223,1,223,108,677,677,224,102,2,223,223,1005,224,434,1001,223,1,223,1008,677,226,224,1002,223,2,223,1006,224,449,1001,223,1,223,7,226,677,224,1002,223,2,223,1006,224,464,1001,223,1,223,1007,677,677,224,102,2,223,223,1005,224,479,1001,223,1,223,1007,226,226,224,1002,223,2,223,1005,224,494,1001,223,1,223,108,226,226,224,1002,223,2,223,1005,224,509,1001,223,1,223,1007,226,677,224,1002,223,2,223,1006,224,524,101,1,223,223,1107,677,677,224,102,2,223,223,1005,224,539,101,1,223,223,1107,677,226,224,102,2,223,223,1005,224,554,1001,223,1,223,108,677,226,224,1002,223,2,223,1006,224,569,1001,223,1,223,1108,226,677,224,1002,223,2,223,1006,224,584,101,1,223,223,8,677,677,224,1002,223,2,223,1006,224,599,1001,223,1,223,1008,226,226,224,102,2,223,223,1006,224,614,101,1,223,223,7,677,677,224,1002,223,2,223,1006,224,629,101,1,223,223,1008,677,677,224,102,2,223,223,1005,224,644,101,1,223,223,7,677,226,224,1002,223,2,223,1005,224,659,101,1,223,223,1108,226,226,224,102,2,223,223,1006,224,674,1001,223,1,223,4,223,99,226)
    $res = IntComp $code 5
    $res.Outputs
}
