
. ./fft.ps1

$VerbosePreference = "SilentlyContinue"

function Assert {
    param($a, $b)

    if ($a -ne $b) {
        throw "'$a' differs from '$b'"
    }
}

Assert (Clear-Signal "12345678") "48226158"
Assert (Clear-Signal "48226158") "34040438"
Assert (Clear-Signal "34040438") "03415518"
Assert (Clear-Signal "03415518") "01029498"
Assert (Clear-SignalPhases "12345678" 4) "01029498"

Assert (Clear-SignalPhases "80871224585914546619083218645595").SubString(0, 8) "24176176"
Assert (Clear-SignalPhases "19617804207202209144916044189917").SubString(0, 8) "73745418"
Assert (Clear-SignalPhases "69317163492948606335995924319873").SubString(0, 8) "52432133"
