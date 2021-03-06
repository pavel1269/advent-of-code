
. ./intComp.ps1

$VerbosePreference = "SilentlyContinue"

function Assert {
    [CmdletBinding()]
    param($a, $b)

    if ($a.COunt -ne $b.COunt) {
        Write-Error "Arrays have different length, '$($a.Count)' vs '$($b.Count)'" -ErrorAction "stop"
    }

    if ($a -is [hashtable]) {
        for ($index = 0; $index -lt $a.Count; $index++) {
            if ($a."$index" -ne $b[$index]) {
                Write-Error "Differece at position '$index', '$($a[$index])' vs '$($b[$index])'" -ErrorAction "stop"
            }
        }
    } else {
        for ($index = 0; $index -lt $a.Count; $index++) {
            if ($a[$index] -ne $b[$index]) {
                Write-Error "Differece at position '$index', '$($a[$index])' vs '$($b[$index])'" -ErrorAction "stop"
            }
        }
    }
}

# 1,0,0,0,99 becomes 2,0,0,0,99 (1 + 1 = 2).
# 2,3,0,3,99 becomes 2,3,0,6,99 (3 * 2 = 6).
# 2,4,4,5,99,0 becomes 2,4,4,5,99,9801 (99 * 99 = 9801).
# 1,1,1,4,99,5,6,0,99 becomes 30,1,1,4,2,5,6,0,99.
try {
Assert (IntComp (@(1,0,0,0,99))).OpCodes @(2,0,0,0,99)
Assert (IntComp @(2,3,0,3,99)).OpCodes @(2,3,0,6,99)
Assert (IntComp @(2,4,4,5,99,0)).OpCodes @(2,4,4,5,99,9801)
Assert (IntComp @(1,1,1,4,99,5,6,0,99)).OpCodes @(30,1,1,4,2,5,6,0,99)
Assert (IntComp @(3,225,4,225,99) 1).Outputs @(1)


$code = @(4,0,104,50,109,5,204,0,99)
Assert (IntComp $code).Outputs @(4, 50, 5)
$code = @(1101,0,1,50,209,50,21101,1,2,1,4,50,204,1,99)
Assert (IntComp $code).Outputs @(1, 3)
$code = @(1102,1,1,50,209,50,21102,3,2,1,4,50,204,1,99)
Assert (IntComp $code).Outputs @(1, 6)
$code = @(109,50,3,52,203,1,4,51,204,2,99)
Assert (IntComp $code @(8, -5)).Outputs @(-5, 8)
$code = @(109,50,3,52,203,1,4,51,204,2,3,0,99)
Assert (IntComp $code @(0, 0)).OpCodeIndex 10
$code = @(109,50,3,52,203,1,4,51,204,2,203,0,99)
Assert (IntComp $code @(0, 0)).OpCodeIndex 10
$code = @(1005,50,6,104,1,99,104,-1,99)
Assert (IntComp $code).Outputs @(1)
$code = @(1006,50,6,104,-1,99,104,1,99)
Assert (IntComp $code).Outputs @(1)

$code = @(109,10,203,0,4,10,99)
$res = IntComp $code
$res.Remove("Outputs")
Assert (IntComp @res -InputParams @(1)).Outputs @(1)

$code = @(3,225,1,225,6,6,1100,1,238,225,104,0,1101,48,82,225,102,59,84,224,1001,224,-944,224,4,224,102,8,223,223,101,6,224,224,1,223,224,223,1101,92,58,224,101,-150,224,224,4,224,102,8,223,223,1001,224,3,224,1,224,223,223,1102,10,89,224,101,-890,224,224,4,224,1002,223,8,223,1001,224,5,224,1,224,223,223,1101,29,16,225,101,23,110,224,1001,224,-95,224,4,224,102,8,223,223,1001,224,3,224,1,223,224,223,1102,75,72,225,1102,51,8,225,1102,26,16,225,1102,8,49,225,1001,122,64,224,1001,224,-113,224,4,224,102,8,223,223,1001,224,3,224,1,224,223,223,1102,55,72,225,1002,174,28,224,101,-896,224,224,4,224,1002,223,8,223,101,4,224,224,1,224,223,223,1102,57,32,225,2,113,117,224,101,-1326,224,224,4,224,102,8,223,223,101,5,224,224,1,223,224,223,1,148,13,224,101,-120,224,224,4,224,1002,223,8,223,101,7,224,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,8,677,226,224,102,2,223,223,1006,224,329,101,1,223,223,107,677,677,224,1002,223,2,223,1006,224,344,101,1,223,223,8,226,677,224,102,2,223,223,1006,224,359,101,1,223,223,107,226,226,224,102,2,223,223,1005,224,374,1001,223,1,223,1108,677,226,224,1002,223,2,223,1006,224,389,101,1,223,223,107,677,226,224,102,2,223,223,1006,224,404,1001,223,1,223,1107,226,677,224,1002,223,2,223,1006,224,419,1001,223,1,223,108,677,677,224,102,2,223,223,1005,224,434,1001,223,1,223,1008,677,226,224,1002,223,2,223,1006,224,449,1001,223,1,223,7,226,677,224,1002,223,2,223,1006,224,464,1001,223,1,223,1007,677,677,224,102,2,223,223,1005,224,479,1001,223,1,223,1007,226,226,224,1002,223,2,223,1005,224,494,1001,223,1,223,108,226,226,224,1002,223,2,223,1005,224,509,1001,223,1,223,1007,226,677,224,1002,223,2,223,1006,224,524,101,1,223,223,1107,677,677,224,102,2,223,223,1005,224,539,101,1,223,223,1107,677,226,224,102,2,223,223,1005,224,554,1001,223,1,223,108,677,226,224,1002,223,2,223,1006,224,569,1001,223,1,223,1108,226,677,224,1002,223,2,223,1006,224,584,101,1,223,223,8,677,677,224,1002,223,2,223,1006,224,599,1001,223,1,223,1008,226,226,224,102,2,223,223,1006,224,614,101,1,223,223,7,677,677,224,1002,223,2,223,1006,224,629,101,1,223,223,1008,677,677,224,102,2,223,223,1005,224,644,101,1,223,223,7,677,226,224,1002,223,2,223,1005,224,659,101,1,223,223,1108,226,226,224,102,2,223,223,1006,224,674,1001,223,1,223,4,223,99,226)
Assert (IntComp $code 1).Outputs @(0, 0, 0, 0, 0, 0, 0, 0, 0, 13547311)

# For example, here are several programs that take one input, compare it to the value 8, and then produce one output:
# 3,9,8,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
# 3,9,7,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
# 3,3,1108,-1,8,3,4,3,99 - Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
# 3,3,1107,-1,8,3,4,3,99 - Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).

Assert (IntComp @(3,9,8,9,10,9,4,9,99,-1,8) 0).Outputs @(0)
Assert (IntComp @(3,9,8,9,10,9,4,9,99,-1,8) 7).Outputs @(0)
Assert (IntComp @(3,9,8,9,10,9,4,9,99,-1,8) 8).Outputs @(1)
Assert (IntComp @(3,9,8,9,10,9,4,9,99,-1,8) 10).Outputs @(0)

Assert (IntComp @(3,9,7,9,10,9,4,9,99,-1,8) 5).Outputs @(1)
Assert (IntComp @(3,9,7,9,10,9,4,9,99,-1,8) 8).Outputs @(0)
Assert (IntComp @(3,9,7,9,10,9,4,9,99,-1,8) 10).Outputs @(0)

Assert (IntComp @(3,3,1108,-1,8,3,4,3,99) 5).Outputs @(0)
Assert (IntComp @(3,3,1108,-1,8,3,4,3,99) 8).Outputs @(1)

Assert (IntComp @(3,3,1107,-1,8,3,4,3,99) 5).Outputs @(1)
Assert (IntComp @(3,3,1107,-1,8,3,4,3,99) 8).Outputs @(0)
Assert (IntComp @(3,3,1107,-1,8,3,4,3,99) 10).Outputs @(0)

# Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:
# 3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9 (using position mode)
# 3,3,1105,-1,9,1101,0,0,12,4,12,99,1 (using immediate mode)

Assert (IntComp @(3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9) 0).Outputs @(0)
Assert (IntComp @(3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9) -5).Outputs @(1)
Assert (IntComp @(3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9) 8).Outputs @(1)

Assert (IntComp @(3,3,1105,-1,9,1101,0,0,12,4,12,99,1) 0).Outputs @(0)
Assert (IntComp @(3,3,1105,-1,9,1101,0,0,12,4,12,99,1) -2).Outputs @(1)
Assert (IntComp @(3,3,1105,-1,9,1101,0,0,12,4,12,99,1) 7).Outputs @(1)

# Here's a larger example:
# 3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
# 1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
# 999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
# The above example program uses an input instruction to ask for a single number. The program will then output 999 if the input value is below 8, output 1000 if the input value is equal to 8, or output 1001 if the input value is greater than 8.

$code = @(3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99)
Assert (IntComp $code -5).Outputs @(999)
Assert (IntComp $code 7).Outputs @(999)
Assert (IntComp $code 8).Outputs @(1000)
Assert (IntComp $code 9).Outputs @(1001)
Assert (IntComp $code 80).Outputs @(1001)

$code = @(109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99)
Assert (IntComp $code).Outputs $code
Assert (IntComp @(1102,34915192,34915192,7,4,7,99,0)).Outputs @(1219070632396864)
Assert (IntComp @(104,1125899906842624,99)).Outputs @(1125899906842624)

$code = Get-Content ".\boost.txt"
$OpCode = $code.Split(',') | ForEach-Object { [int64]$_}
Assert (IntComp $OpCode 1).Outputs @(2745604242)

}
catch {
    $msg = $_ | Out-String
    $stacktrace = $_.ScriptStackTrace | Out-String
    Write-Error "$msg`n$stacktrace" -ErrorAction "stop"
}
