
$ErrorActionPreference = "Stop"

function Load-IntCompProgram {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [string]
        $Filename
    )

    $program = Get-Content $Filename
    $OpCode = $program.Split(',') | ForEach-Object { [decimal]$_ }
    $OpCode = ConvertTo-HashTable $OpCode
    return $OpCode
}

function ConvertTo-HashTable {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [decimal[]]
        $Array
    )

    $HashTable = @{}
    for ([decimal]$i = 0; $i -lt $Array.Count; $i++) {
        $HashTable."$i" = $Array[$i]
    }

    return $HashTable
}

function IntComp {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        $OpCodes,

        [decimal[]]
        $InputParams = @(),
        
        [decimal]
        $OpCodeIndex = 0,

        [decimal]
        $RelativeBase = 0
    )

    function Get-Op {
        [CmdletBinding()]
        param([Hashtable]$OpCodes, [decimal]$index, $mode)

        [decimal]$op = $OpCodes."$index"
        if ($mode -eq 0) {
            $opVal = $OpCodes."$op"
        }
        elseif ($mode -eq 1) {
            $opVal = $op
        }
        elseif ($mode -eq 2) {
            $opVal = $OpCodes."$($op + $RelativeBase)"
        }
        else {
            throw "Unsupported mode '$mode'"
        }

        return [decimal]$opVal
    }

    function Get-OutOp {
        [CmdletBinding()]
        param([Hashtable]$OpCodes, [int64]$index, $mode)

        if ($mode -eq 1) {
            throw "Input / output in immediate mode"
        }
        [decimal]$op = $OpCodes."$index"
        if ($mode -eq 2) {
            $op += $relativeBase
        }

        return [int64]$op
    }

    if ($OpCodes -is [Array]) {
        $OpCodes = ConvertTo-HashTable $OpCodes
    }

    $inputParamIndex = 0
    $outputs = New-Object "System.Collections.ArrayList"
    for ([decimal]$index = $OpCodeIndex; $index -lt $OpCodes.Count; $index += $operands) {
        $operands = 1
        $func = $OpCodes."$index"

        $tmp = [Math]::Floor($func / 100)
        $modeOp1 = $tmp % 10
        $tmp = [Math]::Floor($tmp / 10)
        $modeOp2 = $tmp % 10
        $tmp = [Math]::Floor($tmp / 10)
        $modeOp3 = $tmp % 10

        Write-Verbose "$(Get-Date) Instruction '$index': '$($func % 100)'('$func') (op1 mode: '$modeOp1', op2 mode: '$modeOp2', op3 mode: '$modeOp3')"

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
                    return @{
                        OpCodes = $OpCodes
                        OpCodeIndex = $index
                        Outputs = $outputs
                        RelativeBase = $RelativeBase
                    }
                }

                $operands = 2
                $op = Get-OutOp $OpCodes ($index + 1) $modeOp1
                $OpCodes."$op" = $InputParams[$inputParamIndex]
                $inputParamIndex++
                Write-Verbose "[$op] = '$($OpCodes."$op")'"
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
                    $OpCodes."$op3" = [decimal]1
                }
                else {
                    $OpCodes."$op3" = [decimal]0
                }
                Write-Verbose "[$op3] = '$($OpCodes."$op3")'"
            }
            8 {
                $operands = 4
                $op1Val = Get-Op $OpCodes ($index + 1) $modeOp1
                $op2Val = Get-Op $OpCodes ($index + 2) $modeOp2
                $op3 = Get-OutOp $OpCodes ($index + 3) $modeOp3

                if ($op1Val -eq $op2Val) {
                    $OpCodes."$op3" = [decimal]1
                }
                else {
                    $OpCodes."$op3" = [decimal]0
                }
                Write-Verbose "[$op3] = '$($OpCodes."$op3")'"
            }
            9 {
                $operands = 2
                $op1Val = Get-Op $OpCodes ($index + 1) $modeOp1
                $RelativeBase += $op1Val
                Write-Verbose "Relative base changed to '$RelativeBase'"
            }
            99 {
                $operands = 1
                
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
