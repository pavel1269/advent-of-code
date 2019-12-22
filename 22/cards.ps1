
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

function Get-ModuloInverse {
    [CmdletBinding()]
    param(
        [decimal]$a,
        [decimal]$m,
        [decimal]$y
    )

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

    [decimal]$a = 1
    [decimal]$b = 0
    foreach ($command in $shuffleCommands) {
        switch -regex ($command) {
            "^deal into new stack$" {
                # $position = $deckSize - $position - 1
                $a *= -1
                $b = $deckSize - $b - 1
            }
            "^cut (-?\d+)$" {
                $n = [decimal]$Matches.1
                # $position = $position + $n
                $b += $n
            }
            "^deal with increment (\d+)$" {
                $n = [decimal]$Matches.1
                # $position = Get-ModuloInverse $n $deckSize $position
                $c = Power $n ($deckSize - 2) $deckSize
                $a *= $c
                $b *= $c
            }
            default {
                throw "Unown command '$command'"
            }
        }

        while ($a -lt 0) {
            $a += $deckSize
        }
        $a = $a % $deckSize

        while ($b -lt 0) {
            $b += $deckSize
        }
        $b = $b % $deckSize

        # while ($position -lt 0) {
        #     $position += $deckSize
        # }
        # $position = $position % $deckSize
    }

    # p1 = a p0 + b
    # p2 = a p1 + b = a(a p0 + b) + b = a^2 p0 + a b + b
    # p3 = a p2 + b = a(a^2 p0 + a b + b) + b = a^3 p0 + a^2 b + a b + b

    # $bi = $b
    # for ($i = 1; $i -lt $iterations; $i++) {
    #     $bi += (Power $a $i $deckSize) * $b
    #     $bi = $bi % $deckSize

    #     if ((($i + 1) % 1000) -eq 0) {
    #         write-host "$(Get-Date) Done i '$($i + 1)'"
    #     }
    # }

    # $a = Power $a $iterations $deckSize
    # $position = $a * $position + $bi
    # $position = $position % $deckSize

    $cache = @{
        1 = @{
            a = $a
            b = $b
        }
    }
    $lastStep = $cache.1
    for ([int64]$i = 2; $i -lt $iterations; $i *= 2) {
        $currentStep = @{
            a = ($lastStep.a * $lastStep.a) % $deckSize
            b = ($lastStep.a * $lastStep.b + $lastStep.b) % $deckSize
        }
        $cache.$i = $currentStep
        $lastStep = $currentStep
    }

    $iterationsToGo = $iterations
    $cache.Keys | Sort-Object -Descending | ForEach-Object {
        $step = $_
        $cachedStep = $cache.$_
        while ($iterationsToGo -ge $step) {
            # Write-Host "To go: '$iterationsToGo', step: '$step'"
            $position = $cachedStep.a * $position + $cachedStep.b
            $position = $position % $deckSize
            $iterationsToGo -= $step
        }
    }

    return $position
}

function Get-ResultPart2 {
    # deal into new stack => $position = $deckSize - $position - 1
    # cut N => $position = $position % $deckSize + $N % $deckSize
    # deal with increment N => $position = $N ^ ($deckSize - 2) % $deckSize * $position % $deckSize
    
    # cut 2175
    # $position = ($position + 2175) % $deckSize
    # deal with increment 62
    # $position = 62 ^ ($deckSize - 2) % $deckSize * ($position + 2175) % $deckSize
    # cut -9109
    # $position = 62 ^ ($deckSize - 2) % $deckSize * ($position + 2175) % $deckSize - 9109 % $deckSize
    # deal with increment 17
    # $position = 17 ^ ($deckSize - 2) % $deckSize * (62 ^ ($deckSize - 2) % $deckSize * ($position + 2175) % $deckSize - 9109 % $deckSize)
    # cut -2565
    # $position = 17 ^ ($deckSize - 2) % $deckSize * 62 ^ ($deckSize - 2) % $deckSize * ($position + 2175 % $deckSize - 9109 % $deckSize) - 2565 % $deckSize
    # deal with increment 20
    # $position = 20 ^ ($deckSize - 2) % $deckSize * $position % $deckSize
    
    # cut 2175
    
    # $position = $position + 2175
    # $c1 = 2175 % $deckSize
    # $position = $position + $c1
    
    # deal with increment 62
    
    # $position = 62 ^ ($deckSize - 2) % $deckSize * ($position + $c1) % $deckSize
    # $c2 = 62 ^ ($deckSize - 2) % $deckSize
    # $position = $c2 * ($position + $c1) % $deckSize
    # $position = $c2 * $position % $deckSize + $c2 * $c1 % $deckSize
    
    # cut -9109
    
    # $position = $c2 * $position % $deckSize + $c2 * $c1 % $deckSize - 9109 % $deckSize
    # $c3 = $c2 * $c1 % $deckSize - 9109 % $deckSize
    # $position = $c2 * $position % $deckSize + $c3
    
    # deal with increment 17
    
    # $position = 17 ^ ($deckSize - 2) % $deckSize * $position % $deckSize
    # $c4 = 17 ^ ($deckSize - 2) % $deckSize
    # $position = $c4 * ($c2 * $position % $deckSize + $c2 * $c1 % $deckSize - $c3)
    # $position = $c4 * $c2 * $position % $deckSize + $c4 * $c3 % $deckSize
    
    # deal into new stack
    
    # $position = $deckSize - ($c4 * $c2 * $position % $deckSize + $c4 * $c3 % $deckSize) - 1
    # $position = $deckSize - $c4 * $c2 * $position % $deckSize - $c4 * $c3 % $deckSize - 1
    # $position =  -1 * $c4 * $c2 * $position % $deckSize + $deckSize - $c4 * $c3 % $deckSize - 1

    [decimal]$deckSize = 10007
    [decimal]$iterations = 1
    [decimal]$position = 1867

    [decimal]$deckSize = 119315717514047
    [decimal]$iterations = 101741582076661
    [decimal]$position = 2020

    $commands = Get-Content "./part1.txt"
    [Array]::Reverse($commands)

    $position = Get-CardBeforeShuffle -deckSize $deckSize -position $position -shuffleCommands $commands -iterations $iterations
    $position

    # 18741272659228 - too low
    # 71047285772808 - correct
}
