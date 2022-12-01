
. ./intComp.ps1

function Assert {
    [CmdletBinding()]
    param($a, $b)

    if ($a.COunt -ne $b.COunt) {
        throw "Arrays have different length, '$($a.Count)' vs '$($b.Count)'"
    }

    for ($index = 0; $index -lt $a.Count; $index++) {
        if ($a[$index] -ne $b[$index]) {
            throw "Differece at position '$index', '$($a[$index])' vs '$($b[$index])'"
        }
    }
}

# 1,0,0,0,99 becomes 2,0,0,0,99 (1 + 1 = 2).
# 2,3,0,3,99 becomes 2,3,0,6,99 (3 * 2 = 6).
# 2,4,4,5,99,0 becomes 2,4,4,5,99,9801 (99 * 99 = 9801).
# 1,1,1,4,99,5,6,0,99 becomes 30,1,1,4,2,5,6,0,99.

Assert (IntComp @(1,0,0,0,99) -Verbose) @(2,0,0,0,99)
Assert (IntComp @(2,3,0,3,99) -Verbose) @(2,3,0,6,99)
Assert (IntComp @(2,4,4,5,99,0) -Verbose) @(2,4,4,5,99,9801)
Assert (IntComp @(1,1,1,4,99,5,6,0,99) -Verbose) @(30,1,1,4,2,5,6,0,99)
