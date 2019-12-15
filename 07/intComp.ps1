
$ErrorActionPreference = "Stop"

function IntComp {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [int[]]
        $OpCodes,

        [int[]]
        $InputParams = @()
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

    $inputParamIndex = 0
    $outputs = [int[]]@()
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

                if ($inputParamIndex -ge $InputParams.Count) {
                    throw "Missing input parameter"
                }

                $operands = 2
                $input = $OpCodes[$index + 1]
                $OpCodes[$input] = $InputParams[$inputParamIndex]
                $inputParamIndex++
                Write-Verbose "[$input] = '$($OpCodes[$input])'"
            }
            4 {
                $operands = 2
                $outputVal = Get-Op $OpCodes ($index + 1) $modeOp1
                $outputs += [int]$outputVal
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
