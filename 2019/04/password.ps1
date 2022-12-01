
function Is-Valid {
    param([string]$Password)

    if ($Password.Length -ne 6) {
        return $false
    }

    $lastChar = $Password[0]
    $sameInSequence = $false
    for ($index = 1; $index -lt 6; $index++) {
        $actualChar = $Password[$index]

        if ($lastChar -gt $actualChar) {
            return $false
        }

        if ($lastChar -eq $actualChar) {
            $sameInSequence = $true
        }

        $lastChar = $actualChar
    }

    if (-not $sameInSequence) {
        return $false
    }

    return $true
}

function Get-PasswordCount {
    param()

    # How many different passwords within the range given in your puzzle input meet these criteria?
    # Your puzzle input is 231832-767346.

    $count = 0
    for ($index = 231832; $index -le 767346; $index++) {
        if (Is-Valid $index) {
            $count++
            Write-Verbose "'$index' matches"
        }
    }

    $count

    # 28429 - too high
    # 1330 - correct
}

function Is-Valid2 {
    param([string]$Password)

    if ($Password.Length -ne 6) {
        return $false
    }

    $lastChar = $Password[0]
    $sequenceLength = 0
    $sameInSequence = $false
    for ($index = 1; $index -lt 6; $index++) {
        $actualChar = $Password[$index]

        if ($lastChar -gt $actualChar) {
            return $false
        }

        if ($lastChar -eq $actualChar) {
            $sequenceLength++
        }
        else {
            if ($sequenceLength -eq 1) {
                $sameInSequence = $true
            }
            $sequenceLength = 0
        }

        $lastChar = $actualChar
    }

    if ($sequenceLength -ne 1 -and -not $sameInSequence) {
        return $false
    }

    return $true
}

function Get-PasswordCount2 {
    param()

    # How many different passwords within the range given in your puzzle input meet these criteria?
    # Your puzzle input is 231832-767346.

    $count = 0
    for ($index = 231832; $index -le 767346; $index++) {
        if (Is-Valid2 $index) {
            $count++
            Write-Verbose "'$index' matches"
        }
    }

    $count

    # 876 - correct
}
