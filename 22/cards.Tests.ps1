
. .\cards.ps1

$VerbosePreference = "SilentlyCOntinue"

function Assert {
    [CmdletBinding()]
    param($a, $b)

    if ($a.COunt -ne $b.COunt) {
        Write-Error "Arrays have different length, '$($a.Count)' vs '$($b.Count)'" -ErrorAction "stop"
    }

    for ($index = 0; $index -lt $a.Count; $index++) {
        if ($a[$index] -ne $b[$index]) {
            Write-Error "Differece at position '$index', '$($a[$index])' vs '$($b[$index])'" -ErrorAction "stop"
        }
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
