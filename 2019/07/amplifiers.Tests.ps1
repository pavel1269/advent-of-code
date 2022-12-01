
. ./amplifiers.ps1

$VerbosePreference = "SilentlyContinue"

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

$code = [int[]](Get-Content "./program-amplifiers-test1.txt").Split(',')
Assert (Amplify-Signal $code) @(43210)

$code = [int[]](Get-Content "./program-amplifiers-test2.txt").Split(',')
Assert (Amplify-Signal $code) @(54321)

$code = [int[]](Get-Content "./program-amplifiers-test3.txt").Split(',')
Assert (Amplify-Signal $code) @(65210)

$code = [int[]](Get-Content "./program-amplifiersFeed-test1.txt").Split(',')
Assert (Amplify-SignalWithFeedback $code) @(139629729)

$code = [int[]](Get-Content "./program-amplifiersFeed-test2.txt").Split(',')
Assert (Amplify-SignalWithFeedback $code) @(18216)
