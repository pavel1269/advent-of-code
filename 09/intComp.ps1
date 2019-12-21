
$ErrorActionPreference = "Stop"

function IntComp {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [int64[]]
        $OpCodesArray,

        [int64[]]
        $InputParams = @(),
        
        [int64]
        $OpCodeIndex = 0
    )

    function Get-Op {
        [CmdletBinding()]
        param([Hashtable]$OpCodes, [int64]$index, $mode)

        $op = $OpCodes."$index"
        if ($mode -eq 0) {
            $opVal = $OpCodes."$op"
        }
        elseif ($mode -eq 1) {
            $opVal = $op
        }
        elseif ($mode -eq 2) {
            $opVal = $OpCodes."$($op + $relativeBase)"
        }
        else {
            throw "Unsupported mode '$mode'"
        }

        return [int64]$opVal
    }

    function Get-OutOp {
        [CmdletBinding()]
        param([Hashtable]$OpCodes, [int64]$index, $mode)

        if ($mode -eq 1) {
            throw "Input / output in immediate mode"
        }
        $op = $OpCodes."$index"
        if ($mode -eq 2) {
            $op += $relativeBase
        }

        return [int64]$op
    }

    function Get-OutOpCodes {
        [CmdletBinding()]
        param([Hashtable]$OpCodes)
        
        $MaxOpCode = $OpCodes.Keys | ForEach-Object { [int64]$_ } | Sort-Object -Descending | Select-Object -First 1
        $ResOpCodes = New-Object "int64[]" ($MaxOpCode + 1)
        $MaxOpCode = $OpCodes.Keys | ForEach-Object { $ResOpCodes[[int64]$_] = $OpCodes.$_ }

        return $ResOpCodes
    }

    $OpCodes = @{}
    for ([int64]$i = 0; $i -lt $OpCodesArray.Count; $i++) {
        $OpCodes."$i" = $OpCodesArray[$i]
    }

    $inputParamIndex = 0
    $outputs = @()
    $relativeBase = 0
    for ([int64]$index = 0; $index -lt $OpCodes.Count; $index += $operands) {
        $operands = 1
        $func = $OpCodes."$index"

        $tmp = [Math]::Floor($func / 100)
        $modeOp1 = $tmp % 10
        $tmp = [Math]::Floor($tmp / 10)
        $modeOp2 = $tmp % 10
        $tmp = [Math]::Floor($tmp / 10)
        $modeOp3 = $tmp % 10

        Write-Verbose "Instruction '$index': '$($func % 100)'('$func') (op1 mode: '$modeOp1', op2 mode: '$modeOp2', op3 mode: '$modeOp3')"

        switch ($func % 100) {
            1 {
                $operands = 4
                $op1Val = Get-Op $OpCodes ($index + 1) $modeOp1
                $op2Val = Get-Op $OpCodes ($index + 2) $modeOp2
                $op3 = Get-OutOp $OpCodes ($index + 3) $modeOp3
                $OpCodes."$op3" = $op1Val + $op2Val
                Write-Verbose "[$op3] = '$op1Val' + '$op2Val' = '$($op1Val + $op2Val)'"
            }
            2 {
                $operands = 4
                $op1Val = Get-Op $OpCodes ($index + 1) $modeOp1
                $op2Val = Get-Op $OpCodes ($index + 2) $modeOp2
                $op3 = Get-OutOp $OpCodes ($index + 3) $modeOp3
                $OpCodes."$op3" = $op1Val * $op2Val
                Write-Verbose "[$op3] = '$op1Val' * '$op2Val' = '$($op1Val * $op2Val)'"
            }
            3 {
                if ($inputParamIndex -ge $InputParams.Count) {
                    Write-Verbose "[$input] = ?"
                    $ResOpCodes = Get-OutOpCodes $OpCodes
                    return @{
                        OpCodes = $ResOpCodes
                        OpCodeIndex = $index
                        Outputs = $outputs
                    }
                }

                $operands = 2
                $op = Get-OutOp $OpCodes ($index + 1) $modeOp1
                $OpCodes."$op" = $InputParams[$inputParamIndex]
                $inputParamIndex++
                Write-Verbose "[$op] = '$InputParam'"
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
                $operands = 4
                $op1Val = Get-Op $OpCodes ($index + 1) $modeOp1
                $op2Val = Get-Op $OpCodes ($index + 2) $modeOp2
                $op3 = Get-OutOp $OpCodes ($index + 3) $modeOp3

                if ($op1Val -lt $op2Val) {
                    $OpCodes."$op3" = [int64]1
                }
                else {
                    $OpCodes."$op3" = [int64]0
                }
                Write-Verbose "[$op3] = '$($OpCodes."$op3")'"
            }
            8 {
                $operands = 4
                $op1Val = Get-Op $OpCodes ($index + 1) $modeOp1
                $op2Val = Get-Op $OpCodes ($index + 2) $modeOp2
                $op3 = Get-OutOp $OpCodes ($index + 3) $modeOp3

                if ($op1Val -eq $op2Val) {
                    $OpCodes."$op3" = [int64]1
                }
                else {
                    $OpCodes."$op3" = [int64]0
                }
                Write-Verbose "[$op3] = '$($OpCodes."$op3")'"
            }
            9 {
                $operands = 2
                $op1Val = Get-Op $OpCodes ($index + 1) $modeOp1
                $relativeBase += $op1Val
                Write-Verbose "Relative base changed to '$relativeBase'"
            }
            99 {
                $operands = 1
                $ResOpCodes = Get-OutOpCodes $OpCodes
                
                return @{
                    OpCodes = $ResOpCodes
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

function Get-ResultPart1 {
    $code = Get-Content ".\boost.txt"
    $OpCode = $code.Split(',') | ForEach-Object { [int64]$_}

    $res = IntComp $OpCode 1 -Verbose
    $res

    # 2745604242 - correct

}

function Get-ResultPart2 {

    $code = Get-Content ".\boost.txt"
    $OpCode = $code.Split(',') | ForEach-Object { [int64]$_}

    $res = IntComp $OpCode 2 -Verbose
    $res

    # 51135 - correct
}
