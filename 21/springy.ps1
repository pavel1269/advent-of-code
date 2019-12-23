
. ..\09\intComp.ps1

function Walk-Springy {
    [CmdletBinding()]
    param(
        [HashTable]
        $OpCode,

        [string[]]
        $commands
    )

    $codedCommands = New-Object "System.Collections.ArrayList"
    for ($index = 0; $index -lt $commands.Count; $index++) {
        $codedCommands.AddRange([int[]][char[]]$commands[$index])
        $codedCommands.Add([int][char]"`n") | Out-Null
    }
    
    $res = IntComp -OpCodes ($OpCode.Clone()) -InputParams $codedCommands

    $printableOut = [string][char[]][int[]]$res.Outputs
    write-host $printableOut
}

function Get-Part1Result {
    [CmdletBinding()]
    param()

    $OpCode = Load-IntCompProgram "./program.txt"

    $commands = @(
        "OR A T"
        "AND B T"
        "AND C T" # T = ABC
        "NOT T J" # T = !(ABC)
        "AND D J" # T = !(ABC)D
        "WALK"
    )

    Walk-Springy $OpCode $commands

    # Cannot convert value "19350375" to type "System.Char". Error: "Value was either too large or too small for a character." - ugly but works
    # 19350375 - correct
}
