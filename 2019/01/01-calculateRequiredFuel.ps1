
$ErrorActionPreference = "Stop"

function Get-FuelRequirement {
    [CmdletBinding()]
    param(
        [Parameter(ValueFromPipeline = $true)]
        [int]
        $Mass
    )

    process {
        $a = [int][Math]::Floor($Mass / 3)
        $res = $a - 2
        if ($res -lt 0) {
            $res = 0
        }

        Write-Verbose "Mass '$Mass', divided: '$a', res: '$res'"
        return $res
    }
}

function Get-FuelRequirementRecursive {
    [CmdletBinding()]
    param(
        [Parameter(ValueFromPipeline = $true)]
        [int]
        $Mass
    )

    process {
        Write-Verbose "Calculating for '$Mass'"
        $sum = 0
        while ($Mass -gt 0) {
            $Mass = Get-FuelRequirement $Mass
            $sum += $Mass
        }
        Write-Verbose "Result: '$sum'"
        return $sum
    }
}

function Get-Result01 {
    [CmdletBinding()]
    param()

    $in = Get-Content -Path "./01-input.txt"
    $in | Get-FuelRequirement | Measure-Object -Sum

    # 34152 - low
    # 3167282 - correct
}

function Get-Result01-part2 {
    [CmdletBinding()]
    param()

    $in = Get-Content -Path "./01-input.txt"
    $in | Get-FuelRequirementRecursive | Measure-Object -Sum

    # 4747944 - low
    # 4748063 - correct
}
