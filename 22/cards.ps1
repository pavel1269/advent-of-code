
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
    # 2564 - too high (index vs position)
    # 1867 - correct
}
