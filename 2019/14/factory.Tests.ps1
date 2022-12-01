
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

Assert (Produce-OreCost $reactions).Cost 31

$reactions = Get-Content "./input2.txt"
Assert (Produce-OreCost $reactions).Cost 165

$reactions = Get-Content "./input3.txt"
Assert (Produce-OreCost $reactions).Cost 13312

$reactions = Get-Content "./input4.txt"
Assert (Produce-OreCost $reactions).Cost 180697

$reactions = Get-Content "./input5.txt"
Assert (Produce-OreCost $reactions).Cost 2210736

# The 13312 ORE-per-FUEL example could produce 82892753 FUEL.
# The 180697 ORE-per-FUEL example could produce 5586022 FUEL.
# The 2210736 ORE-per-FUEL example could produce 460664 FUEL.

$reactions = Get-Content "./input3.txt"
Assert (Produce-OreCost $reactions 1000000000000).Fuel 82892753

$reactions = Get-Content "./input4.txt"
Assert (Produce-OreCost $reactions 1000000000000).Fuel 5586022

$reactions = Get-Content "./input5.txt"
Assert (Produce-OreCost $reactions 1000000000000).Fuel 460664
