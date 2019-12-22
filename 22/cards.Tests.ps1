
. .\cards.ps1

$VerbosePreference = "SilentlyCOntinue"

function Assert {
    [CmdletBinding()]
    param($a, $b)

    if ($a.Count -ne $b.Count) {
        Write-Error "Arrays have different length, '$($a.Count)' vs '$($b.Count)'" -ErrorAction "stop"
    }

    for ($index = 0; $index -lt $a.Count; $index++) {
        if ($a[$index] -ne $b[$index]) {
            Write-Error "Differece at position '$index', '$($a[$index])' vs '$($b[$index])'" -ErrorAction "stop"
        }
    }
}

function AssertScalar {
    param($a, $b)

    if ($a -ne $b) {
        Write-Error "'$a' differs from '$b'" -ErrorAction "Stop"
    }
}

$deck = Prepare-CardDeck 10
Assert $deck @(0,1,2,3,4,5,6,7,8,9)
Assert (Shuffle-Deck $deck "deal into new stack") @(9,8,7,6,5,4,3,2,1,0)
Assert (Shuffle-Deck $deck "cut 3") @(3,4,5,6,7,8,9,0,1,2)
Assert (Shuffle-Deck $deck "cut -4") @(6,7,8,9,0,1,2,3,4,5)
Assert (Shuffle-Deck $deck "deal with increment 3") @(0,7,4,1,8,5,2,9,6,3)

$commands = @(
    "deal with increment 7"
    "deal into new stack"
    "deal into new stack"
)
Assert (Shuffle-Deck $deck $commands) @(0,3,6,9,2,5,8,1,4,7)

$commands = @(
    "cut 6"
    "deal with increment 7"
    "deal into new stack"
)
Assert (Shuffle-Deck $deck $commands) @(3,0,7,4,1,8,5,2,9,6)

$commands = @(
    "deal with increment 7"
    "deal with increment 9"
    "cut -2"
)
Assert (Shuffle-Deck $deck $commands) @(6,3,0,7,4,1,8,5,2,9)

$commands = @(
    "deal into new stack"
    "cut -2"
    "deal with increment 7"
    "cut 8"
    "cut -4"
    "deal with increment 7"
    "cut 3"
    "deal with increment 9"
    "deal with increment 3"
    "cut -1"
)
Assert (Shuffle-Deck $deck $commands) @(9,2,5,8,1,4,7,0,3,6)

$commands = Get-Content "./part1.txt"
AssertScalar (Get-CardAfterShuffle -deckSize 10007 -position 2019 -shuffleCommands $commands) 1867
AssertScalar (Get-CardAfterShuffle -deckSize 10007 -position 1867 -shuffleCommands $commands) 6250
AssertScalar (Get-CardAfterShuffle -deckSize 10007 -position 6250 -shuffleCommands $commands) 8309
[Array]::Reverse($commands)
AssertScalar (Get-CardBeforeShuffle -deckSize 10007 -position 1867 -shuffleCommands $commands) 2019
AssertScalar (Get-CardBeforeShuffle -deckSize 10007 -position 6250 -shuffleCommands $commands -iterations 2) 2019
AssertScalar (Get-CardBeforeShuffle -deckSize 10007 -position 8309 -shuffleCommands $commands -iterations 2) 1867
AssertScalar (Get-CardBeforeShuffle -deckSize 10007 -position 8309 -shuffleCommands $commands -iterations 3) 2019
