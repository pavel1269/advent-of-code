
. ./fft.ps1

$VerbosePreference = "SilentlyContinue"

function Assert {
    param($a, $b)

    if ($a -ne $b) {
        Write-Error "'$a' differs from '$b'"
    }
}

# Assert (Clear-Signal "12345678") "48226158"
# Assert (Clear-Signal "48226158") "34040438"
# Assert (Clear-Signal "34040438") "03415518"
# Assert (Clear-Signal "03415518") "01029498"
# Assert (Clear-SignalPhases "12345678" 4) "01029498"

# Assert (Clear-SignalPhases "80871224585914546619083218645595").SubString(0, 8) "24176176"
# Assert (Clear-SignalPhases "19617804207202209144916044189917").SubString(0, 8) "73745418"
# Assert (Clear-SignalPhases "69317163492948606335995924319873").SubString(0, 8) "52432133"

Assert (Get-BinonExpModPrime 5 0 13) 1
Assert (Get-BinonExpModPrime 5 1 13) 5
Assert (Get-BinonExpModPrime 5 2 13) 10
Assert (Get-BinonExpModPrime 5 3 13) 10
Assert (Get-BinonExpModPrime 5 4 13) 5
Assert (Get-BinonExpModPrime 5 5 13) 1
Assert (Get-BinonExpModPrime 5 0 7) 1
Assert (Get-BinonExpModPrime 5 1 7) 5
Assert (Get-BinonExpModPrime 5 2 7) 3
Assert (Get-BinonExpModPrime 5 3 7) 3
Assert (Get-BinonExpModPrime 5 4 7) 5
Assert (Get-BinonExpModPrime 5 5 7) 1

# 25
# 1        25      300      2300    12650    53130    177100   480700  1081575  2042975  3268760  4457400  5200300  5200300

Assert (Get-BinonExpModPrime 25 0 8191) 1
Assert (Get-BinonExpModPrime 25 1 8191) 25
Assert (Get-BinonExpModPrime 25 2 8191) 300
Assert (Get-BinonExpModPrime 25 3 8191) 2300
Assert (Get-BinonExpModPrime 25 4 8191) 4459 # 12650
Assert (Get-BinonExpModPrime 25 5 8191) 3984 # 53130
Assert (Get-BinonExpModPrime 25 6 8191) 5089 # 177100
Assert (Get-BinonExpModPrime 25 7 8191) 5622 # 480700
Assert (Get-BinonExpModPrime 25 8 8191) 363 # 1081575
Assert (Get-BinonExpModPrime 25 9 8191) 3416 # 2042975
Assert (Get-BinonExpModPrime 25 10 8191) 551 # 3268760
Assert (Get-BinonExpModPrime 25 11 8191) 1496 # 4457400
Assert (Get-BinonExpModPrime 25 12 8191) 7206 # 5200300
Assert (Get-BinonExpModPrime 25 12 8191) (5200300 % 8191) # 5200300

# 13
# 1287     715      286       78       13       1

Assert (Get-BinonExpModPrime 13 0 2) 1
Assert (Get-BinonExpModPrime 13 1 2) 1
Assert (Get-BinonExpModPrime 13 2 8191) 78
Assert (Get-BinonExpModPrime 13 2 2) 0
Assert (Get-BinonExpModPrime 13 3 2) 0
Assert (Get-BinonExpModPrime 13 4 2) 1
Assert (Get-BinonExpModPrime 13 5 2) 1

Assert (Get-BinonExpMod10 25 0) 1
Assert (Get-BinonExpMod10 25 1) 5
Assert (Get-BinonExpMod10 25 2) 0
Assert (Get-BinonExpMod10 25 3) 0
Assert (Get-BinonExpMod10 25 4) 0
Assert (Get-BinonExpMod10 25 5) 0
Assert (Get-BinonExpMod10 25 6) 0
Assert (Get-BinonExpMod10 25 7) 0
Assert (Get-BinonExpMod10 25 8) 5
Assert (Get-BinonExpMod10 25 9) 5
Assert (Get-BinonExpMod10 25 10) 0
Assert (Get-BinonExpMod10 25 11) 0
Assert (Get-BinonExpMod10 25 12) 0

Assert (Get-BinonExpMod10 13 0) 1
Assert (Get-BinonExpMod10 13 1) 3
Assert (Get-BinonExpMod10 13 2) 8
Assert (Get-BinonExpMod10 13 3) 6
Assert (Get-BinonExpMod10 13 4) 5
Assert (Get-BinonExpMod10 13 5) 7

# "examples" from part 1
Assert (Clear-RepeatedSignalPhases "000001105678" 1 1 1) "8"
Assert (Clear-RepeatedSignalPhases "000000805678" 1 1 4) "6158"
Assert (Clear-RepeatedSignalPhases "000000805678" 1 4 4) "9498"

Assert (Clear-RepeatedSignalPhases "03081770884921959731165446850517" 10000 100 8 -Verbose) "53553731"
Assert (Clear-RepeatedSignalPhases "03036732577212944063491565474664" 10000 100 8 -Verbose) "84462026"
Assert (Clear-RepeatedSignalPhases "02935109699940807407585447034323" 10000 100 8 -Verbose) "78725270"
