
$ErrorActionPreference = "Stop"

function IntComp {
    [CmdletBinding()]
    param(
        [int[]]
        $OpCodes
    )

    for ($index = 0; $index -lt $OpCodes.Count; $index += 4) {
        $func = $OpCodes[$index]
        Write-Verbose "Instruction 1: '$func'"

        switch ($func) {
            1 {
                $op1 = $OpCodes[$index + 1]
                $op1Val = $OpCodes[$op1]
                $op2 = $OpCodes[$index + 2]
                $op2Val = $OpCodes[$op2]
                $op3 = $OpCodes[$index + 3]
                $OpCodes[$op3] = $op1Val + $op2Val
                Write-Verbose "[$op3] = '$op1Val'($op1) + '$op2Val'($op2) = '$($op1Val + $op2Val)'"
            }
            2 {
                $op1 = $OpCodes[$index + 1]
                $op1Val = $OpCodes[$op1]
                $op2 = $OpCodes[$index + 2]
                $op2Val = $OpCodes[$op2]
                $op3 = $OpCodes[$index + 3]
                $OpCodes[$op3] = $op1Val * $op2Val
                Write-Verbose "[$op3] = '$op1Val'($op1) * '$op2Val'($op2) = '$($op1Val * $op2Val)'"
            }
            99 {
                $index = $OpCodes.Count
                return $OpCodes
            }
            default {
                throw "Op code '$func' encountered"
            }
        }
    }

    throw "Missing halt"
}

function Get-Result {
    $code = @(1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,6,19,1,9,19,23,2,23,10,27,1,27,5,31,1,31,6,35,1,6,35,39,2,39,13,43,1,9,43,47,2,9,47,51,1,51,6,55,2,55,10,59,1,59,5,63,2,10,63,67,2,9,67,71,1,71,5,75,2,10,75,79,1,79,6,83,2,10,83,87,1,5,87,91,2,9,91,95,1,95,5,99,1,99,2,103,1,103,13,0,99,2,14,0,0)
    $code[1] = 12
    $code[2] = 2
    $res = IntComp $code
    $res[0]

    # 3931283 - correct
}

function Get-Result-part2 {

    for ($noun = 0; $noun -lt 100; $noun++) {
        for ($verb = 0; $verb -lt 100; $verb++) {
            $code = @(1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,6,19,1,9,19,23,2,23,10,27,1,27,5,31,1,31,6,35,1,6,35,39,2,39,13,43,1,9,43,47,2,9,47,51,1,51,6,55,2,55,10,59,1,59,5,63,2,10,63,67,2,9,67,71,1,71,5,75,2,10,75,79,1,79,6,83,2,10,83,87,1,5,87,91,2,9,91,95,1,95,5,99,1,99,2,103,1,103,13,0,99,2,14,0,0)
            $code[1] = $noun
            $code[2] = $verb
            $res = IntComp $code
            Write-Host "'$noun', '$verb': '$($res[0])'"
            
            if ($res[0] -eq 19690720) {
                Write-Host "WooHoo ($(100 * $noun + $verb))"
                return
            }
        }
    }

    # 6979 - correct

}
