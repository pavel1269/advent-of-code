
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

    $damage = $res.Outputs[$res.Outputs.Count - 1]
    if ($damage -gt 255) {
        $res.Outputs[$res.Outputs.Count - 1] = [int][char]' '
    }

    $printableOut = [string][char[]][int[]]$res.Outputs
    write-host $printableOut
    write-host "Damage: $damage"
}

function Get-Part1Result {
    [CmdletBinding()]
    param()

    $OpCode = Load-IntCompProgram "./program.txt"

    $commands = @(
        "OR A T"
        "AND B T"
        "AND C T" # T = ABC
        "NOT T J" # J = !(ABC)
        "AND D J" # J = !(ABC)D
        "WALK"
    )

    Walk-Springy $OpCode $commands

    # Cannot convert value "19350375" to type "System.Char". Error: "Value was either too large or too small for a character." - ugly but works
    # 19350375 - correct
}

function Get-Part2Result {
    [CmdletBinding()]
    param()

    $OpCode = Load-IntCompProgram "./program.txt"

    $commands = @(
        "OR E T"
        "AND F T" # T = EF

        "OR E J"
        "AND I J"
        "OR H J"
        "OR T J" # J = H or EI or EF

        "NOT A T"
        "NOT T T" # T = A
        "AND B T"
        "AND C T" # T = ABC
        "NOT T T" # T = !(ABC)
        "AND D T" # T = !(ABC)D

        "AND T J"

        "RUN"
    )

    Walk-Springy $OpCode $commands

    # 1143990055 - correct
}
