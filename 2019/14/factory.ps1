
$ErrorActionPreference = "Stop"

function Process-Reactions {
    [CmdletBinding()]
    param(
        [string[]]
        $Reactions
    )

    $ReactionData = @{}
    $Reactions | ForEach-Object {
        $reaction = $_
        $reactionSplit = $reaction.Split(", ") | Where-Object { $_ }

        $reactionInputs = @()
        $output = $false
        for ($index = 0; $index -lt $reactionSplit.Count; $index += 2) {
            if ($reactionSplit[$index] -eq "=>") {
                $index++
                $output = $true
            }

            $reactionIngerient = @{
                Amount = [int]$reactionSplit[$index]
                Type = $reactionSplit[$index + 1]
            }

            if ($output) {
                $ReactionData.$($reactionIngerient.Type) = @{
                    Amount = $reactionIngerient.Amount
                    Needs = $reactionInputs
                }
            }
            else {
                $reactionInputs += $reactionIngerient
            }
        }
    }

    return $ReactionData
}

function Produce-OreCost {
    [CmdletBinding()]
    param(
        [string[]]
        $Reactions,

        [int64]
        $OreLimit = -1
    )

    $ReactionData = Process-Reactions $Reactions

    $ReactionPriorities = @{}
    $ReactionData.Keys | ForEach-Object { $ReactionPriorities.$_ = 0 }
    $ReactionData.Keys | ForEach-Object {
        $reaction = $ReactionData[$_]
        $reaction.Needs.Type | ForEach-Object {
            $ReactionPriorities.$_++
        }
    }

    $Costs = @{
        "ORE" = 0
    }
    $ReactionData.Keys | ForEach-Object { $Costs.$_ = 0 }

    function Run-Reactions {
        while ([bool]($needs = $Costs.Keys | Where-Object { $_ -ne "ORE" -and $Costs.$_ -lt 0 })) {
            $produce = $needs | Sort-Object { $ReactionPriorities.$_ } | Select-Object -First 1
            $reaction = $ReactionData.$produce
            $timesRaw = [Math]::Abs($Costs.$produce) / $reaction.Amount
            $times = [Math]::Floor($timesRaw)
            if ($times -ne $timesRaw) {
                $times++
            }
            Write-Verbose "Producing '$($produce)' ($times)"
            $Costs.$produce += ($reaction.Amount * $times)
            $reaction.Needs | ForEach-Object {
                $cost = $_
                $Costs.$($cost.Type) -= ($cost.Amount * $times)
            }
        }
    }

    $Costs."FUEL" = -1

    Run-Reactions

    $produced = 1
    Write-Verbose "Produced '1' FUEL"
    $maxCostPerOne = -($Costs."ORE")

    while (($OreLimit + $Costs."ORE") -gt 0) {
        $shouldMake = [Math]::Floor(($OreLimit + $Costs."ORE") / $maxCostPerOne)
        if ($shouldMake -le 0) {
            break
        }

        $Costs."FUEL" = -$shouldMake
        Run-Reactions
        $produced += $shouldMake
        Write-Verbose "Produced '$shouldMake' FUEL"
    }

    return @{
        Fuel = $produced
        Cost = -($Costs."ORE")
    }
}

function Get-Part1 {
    $reactions = Get-Content "./part1.txt"
    $res = Produce-OreCost $reactions
    $res

    # 579797 - correct
}

function Get-Part2 {
    $reactions = Get-Content "./part1.txt"
    $res = Produce-OreCost $reactions 1000000000000
    $res

    # 1724741 - low
    # 2521844 - correct
}
