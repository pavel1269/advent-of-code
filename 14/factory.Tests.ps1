
. ./factory.ps1

$VerbosePreference = "SilentlyContinue"

function Assert {
    param($a, $b)

    if ($a -ne $b) {
        throw "'$a' differs from '$b'"
    }
}

$reactions = Get-Content "./input1.txt"
$ret = Process-Reactions $reactions
Assert $ret.B.Amount 1
Assert $ret.B.Needs.Amount 1
Assert $ret.B.Needs.Type "ORE"

Assert (Produce-OreCost $reactions) 31

$reactions = Get-Content "./input2.txt"
Assert (Produce-OreCost $reactions) 165

$reactions = Get-Content "./input3.txt"
Assert (Produce-OreCost $reactions) 13312

$reactions = Get-Content "./input4.txt"
Assert (Produce-OreCost $reactions) 180697

$reactions = Get-Content "./input5.txt"
Assert (Produce-OreCost $reactions) 2210736
