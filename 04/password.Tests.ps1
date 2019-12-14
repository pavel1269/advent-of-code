
. ./password.ps1

function Assert {
    param($a, $b)

    if ($a -ne $b) {
        throw "'$a' differs from '$b'"
    }
}

# 111111 meets these criteria (double 11, never decreases).
# 223450 does not meet these criteria (decreasing pair of digits 50).
# 123789 does not meet these criteria (no double).

Assert (Is-Valid 111111) $true
Assert (Is-Valid 223450) $false
Assert (Is-Valid 123789) $false
Assert (Is-Valid 699969) $false

# 112233 meets these criteria because the digits never decrease and all repeated digits are exactly two digits long.
# 123444 no longer meets the criteria (the repeated 44 is part of a larger group of 444).
# 111122 meets the criteria (even though 1 is repeated more than twice, it still contains a double 22).

Assert (Is-Valid2 112233) $true
Assert (Is-Valid2 123444) $false
Assert (Is-Valid2 111122) $true
