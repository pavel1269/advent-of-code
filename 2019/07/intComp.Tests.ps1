
. ./intComp.ps1

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

# 1,0,0,0,99 becomes 2,0,0,0,99 (1 + 1 = 2).
# 2,3,0,3,99 becomes 2,3,0,6,99 (3 * 2 = 6).
# 2,4,4,5,99,0 becomes 2,4,4,5,99,9801 (99 * 99 = 9801).
# 1,1,1,4,99,5,6,0,99 becomes 30,1,1,4,2,5,6,0,99.

Assert (IntComp (@(1,0,0,0,99))).OpCodes @(2,0,0,0,99)
Assert (IntComp @(2,3,0,3,99)).OpCodes @(2,3,0,6,99)
Assert (IntComp @(2,4,4,5,99,0)).OpCodes @(2,4,4,5,99,9801)
Assert (IntComp @(1,1,1,4,99,5,6,0,99)).OpCodes @(30,1,1,4,2,5,6,0,99)

$code = [int[]](Get-Content "./input1.txt").Split(',')
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

$code = [int[]](Get-Content "./program-amplifiers-test1.txt").Split(',')
$res = IntComp $code @(4, 0)
$res = IntComp $code @(3, $res.Outputs[0])
$res = IntComp $code @(2, $res.Outputs[0])
$res = IntComp $code @(1, $res.Outputs[0])
Assert (IntComp $code @(0, $res.Outputs[0])).Outputs @(43210)
Assert (IntComp $code 4).OpCodeIndex 2

$code = [int[]](Get-Content "./program-amplifiers-test2.txt").Split(',')
$res = IntComp $code @(0, 0)
$res = IntComp $code @(1, $res.Outputs[0])
$res = IntComp $code @(2, $res.Outputs[0])
$res = IntComp $code @(3, $res.Outputs[0])
Assert (IntComp $code @(4, $res.Outputs[0])).Outputs @(54321)

$code = [int[]](Get-Content "./program-amplifiers-test3.txt").Split(',')
$res = IntComp $code @(1, 0)
$res = IntComp $code @(0, $res.Outputs[0])
$res = IntComp $code @(4, $res.Outputs[0])
$res = IntComp $code @(3, $res.Outputs[0])
Assert (IntComp $code @(2, $res.Outputs[0])).Outputs @(65210)
