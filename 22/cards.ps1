
$ErrorActionPreference = "Stop"

function Prepare-CardDeck {
    [CmdletBinding()]
    param([int]$cards)

    $deck = New-Object "int[]" $cards
    for ($index = 0; $index -lt $cards; $index++) {
        $deck[$index] = $index
    }

    return $deck
}

function Shuffle-Deck {
    [CmdletBinding()]
    param(
        [int[]]$deck,
        [string[]]$shuffleCommands
    )

    foreach ($command in $shuffleCommands) {
        switch -regex ($command) {
            "^deal into new stack$" {
                [Array]::Reverse($deck)
            }
            "^cut (-?\d+)$" {
                $n = $Matches.1
                $n = $n % ($deck.Count)
                if ($n -eq 0) {
                    continue
                }
                elseif ($n -gt 0) {
                    $deck = $deck[$n..($deck.Count - 1)] + $deck[0..($n - 1)]
                }
                else {
                    $deck = $deck[$n..-1] + $deck[0..($deck.Count + $n - 1)]
                }
            }
            "^deal with increment (\d+)$" {
                $n = $Matches.1
                if ($n -eq 0) {
                    continue
                }
                elseif ($n -le 0) {
                    throw "Cannot 'deal with increment' '$n'"
                }
                $n = $n % ($deck.Count)

                $backupDeck = New-Object "int[]" $deck.Count
                $indexNew = 0;
                for ($indexDeck = 0; $indexDeck -lt $deck.Count; $indexDeck++) {
                    $backupDeck[$indexNew] = $deck[$indexDeck]
                    $indexNew = ($indexNew + $n) % ($deck.Count)
                }
                $deck = $backupDeck
            }
            default {
                throw "Unown command '$command'"
            }
        }
    }

    return $deck
}

function Get-ResultPart1 {
    $deck = Prepare-CardDeck 10007
    $commands = Get-Content "./part1.txt"
    $newDeck = Shuffle-Deck $deck $commands
    $newDeck.IndexOf(2019)

    # 6080 - too high
    # 2564 - too high
    # 1867 - correct (index vs position)
}

function Get-CardAfterShuffle {
    [CmdletBinding()]
    param(
        [decimal]$deckSize,
        [decimal]$iterations = 1,
        [decimal]$position,
        [string[]]$shuffleCommands
    )

    for ($index = 0; $index -lt $iterations; $index++) {
        foreach ($command in $shuffleCommands) {
            switch -regex ($command) {
                "^deal into new stack$" {
                    $position = $deckSize - $position - 1
                }
                "^cut (-?\d+)$" {
                    $n = [decimal]$Matches.1
                    $position = $position - $n
                }
                "^deal with increment (\d+)$" {
                    $n = [decimal]$Matches.1
                    $position = $position * $n
                }
                default {
                    throw "Unown command '$command'"
                }
            }
            while ($position -lt 0) {
                $position += $deckSize
            }
            $position = $position % $deckSize
        }
    }

    return $position
}

# https://www.geeksforgeeks.org/multiplicative-inverse-under-modulo-m/
function Get-ModuloInverse {
    [CmdletBinding()]
    param(
        [decimal]$a,
        [decimal]$m,
        [decimal]$y
    )

    function Power {
        [CmdletBinding()]
        param(
            [decimal]$x,
            [decimal]$y,
            [decimal]$m
        )

        if ($y -eq 0) {
            return 1
        }

        [decimal]$power = Power $x ([Math]::Floor($y / 2)) $m
        $power = $power % $m
        $power = $power * $power
        $power = $power % $m

        if (($y % 2) -ne 0) {
            $power = $power * $x
            $power = $power % $m
        }

        return $power
    }

    [decimal]$inv = Power $a ($m - 2) $m
    $res = $inv * $y
    $res = $res % $m
    return $res
}

function Get-CardBeforeShuffle {
    [CmdletBinding()]
    param(
        [decimal]$deckSize,
        [decimal]$iterations = 1,
        [decimal]$position,
        [string[]]$shuffleCommands
    )

    for ($index = 0; $index -lt $iterations; $index++) {
        foreach ($command in $shuffleCommands) {
            switch -regex ($command) {
                "^deal into new stack$" {
                    $position = $deckSize - $position - 1
                }
                "^cut (-?\d+)$" {
                    $n = [decimal]$Matches.1
                    $position = $position + $n
                }
                "^deal with increment (\d+)$" {
                    $n = [decimal]$Matches.1
                    $position = Get-ModuloInverse $n $deckSize $position
                }
                default {
                    throw "Unown command '$command'"
                }
            }
            while ($position -lt 0) {
                $position += $deckSize
            }
            $position = $position % $deckSize
        }
    }

    return $position
}

function Get-ResultPart2 {
    [decimal]$deckSize = 10007
    [decimal]$iterations = 1
    [decimal]$position = 1867

    [decimal]$deckSize = 119315717514047
    [decimal]$iterations = 101741582076661
    [decimal]$position = 2020

    $commands = Get-Content "./part1.txt"
    [Array]::Reverse($commands)

    for ([decimal]$i = 0; $i -lt $iterations; $i++) {
        $position = Get-CardBeforeShuffle -deckSize $deckSize -position $position -shuffleCommands $commands
        write-Host "$i $position"
        if ($position -eq 1867) {
            break
        }
        # if ((($i + 1) % 10) -eq 0) {
        #     Write-Host "$(Get-Date) Iteration '$($i + 1)' done"
        # }
    }
    $position
}
