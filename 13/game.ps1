
. ..\09\intComp.ps1

function Get-Part1Result {
    
    $OpCode = Load-IntCompProgram "./program.txt"

    $CompInputs = @{
        OpCodeIndex = 0
        OpCodes     = $OpCode
        InputParams = @()
    }

    $res = IntComp @CompInputs

    $count = 0
    for ($index = 0; $index -lt $res.Outputs.length; $index += 3) {
        $x = $res.Outputs[$index]
        $y = $res.Outputs[$index + 1]
        $type = $res.Outputs[$index + 2]

        if ($type -eq 2) {
            $count++
        }
    }
}
