
$ErrorActionPreference = "Stop"

function Clear-Signal {
    [CmdletBinding()]
    param(
        [string]
        $Signal,

        [int[]]
        $Pattern = @(0, 1, 0, -1)
    )

    $signalArray = @()
    for ($index = 0; $index -lt $Signal.Length; $index++) {
        $signalArray += [int]($Signal[$index]) - 48
    }

    $outputSignal = ""
    for ($outIndex = 1; $outIndex -le $signalArray.Count; $outIndex++) {
        $output = 0
        $amp = 0
        $patternIndex = 0
        for ($inIndex = $outIndex; $inIndex -le $signalArray.Count; $inIndex++) {
            # out index, in index => pattern index
            # 1, 1 => 1
            # 1, 2 => 2
            # 1, 3 => 3
            # 2, 1 => 0
            # 2, 2 => 1
            # 2, 3 => 1
            # 2, 4 => 2
            # 3, 1 => 0
            # 3, 2 => 0
            # 3, 3 => 1
            # 3, 4 => 1
            if (($inIndex % $outIndex) -eq 0) {
                $patternIndex++
                $amp = $Pattern[$patternIndex % $Pattern.Count]
            }

            if ($amp -ne 0) {
                $output += $signalArray[$inIndex - 1] * $amp
                # Write-Verbose "'$signalArray[$inIndex - 1]' * '$amp'"
            }
            else {
                $inIndex += $outIndex - 1
            }
        }

        $output = [Math]::Abs($output) % 10
        Write-Verbose "['$outIndex'] = '$output'"
        $outputSignal += [string]$output
    }

    return $outputSignal
}

function Clear-SignalPhases {
    [CmdletBinding()]
    param(
        [string]
        $Signal,

        [int]
        $Phases = 100
    )

    for ($phaseIndex = 0; $phaseIndex -lt $Phases; $phaseIndex++) {
        $Signal = Clear-Signal $Signal -Verbose:$false
        Write-Verbose "$(Get-Date) Phase '$phaseIndex' done"
    }

    return $Signal
}

function Get-Part1Result {
    $signal = "59728776137831964407973962002190906766322659303479564518502254685706025795824872901465838782474078135479504351754597318603898249365886373257507600323820091333924823533976723324070520961217627430323336204524247721593859226704485849491418129908885940064664115882392043975997862502832791753443475733972832341211432322108298512512553114533929906718683734211778737511609226184538973092804715035096933160826733751936056316586618837326144846607181591957802127283758478256860673616576061374687104534470102346796536051507583471850382678959394486801952841777641763547422116981527264877636892414006855332078225310912793451227305425976335026620670455240087933409"
    $res = Clear-SignalPhases $signal
    $res

    # 76795888404007217241605692440918498059033714253979089993002955679090168750389144227458767776558545236205965951036679611045715214238516252742883014311380453273266162184152735284960510020587167360654219625714751770628887184931560418510784287651985352512009407202993191429377186986198532073214863698298729009954422014762659675106425133228975889427297331548411074196473339074755662125302823926771230622614104200541059260334003753144379140185961760424909360397065651629209754099810121613397856245727768704069155124284548951631901161329920760247692674139420900628332608933707037124032270530049632333829017565201097202225955151647895576675625905795037983409
    # 76795888 - correct
}

function Clear-RepeatedSignalPhases {
    [CmdletBinding()]
    param(
        [string]
        $Signal,

        [int]
        $Repeats = 10000,

        [int]
        $Phases = 100,

        [int]
        $Length = 8
    )


    # $finalSignal = ""
    # Write-Verbose "$(Get-Date) Preparing signal"
    # for ($index = 0; $index -lt $Repeats; $index++) {
    #     $finalSignal += $signal
    # }
    # Write-Verbose "$(Get-Date) Multiplied signal, now it has '$($finalSignal.Length)' characters"

    # $res = Clear-SignalPhases $finalSignal
    # return $res

    # "12345678" -> "48226158" -> "34040438" -> "03415518" -> "01029498"
    # 0)  1  0 -1  0  1  0 -1  0
    # 1)  0  1  1  0  0 -1 -1  0
    # 2)  0  0  1  1  1  0  0  0
    # 3)  0  0  0  1  1  1  1  0
    # 4)  0  0  0  0  1  1  1  1
    # 5)  0  0  0  0  0  1  1  1
    # 6)  0  0  0  0  0  0  1  1
    # 7)  0  0  0  0  0  0  0  1
    #
    # M - iteraction, M >= 0
    # N - Length, N > 0, e.g. N = 8
    # R - Result, R[N]
    #
    # RM[7] = R0[7]
    #
    # R1[6] = R0[6] + R0[7]
    # R2[6] = R1[6] + R1[7] = R0[6] + 2 * R0[7]
    # R3[6] = R2[6] + R2[7] = R0[6] + 3 * R0[7]
    # RM[6] = R0[6] + M * R0[7]
    #
    # R1[5] = R0[5] + R0[6] + R0[7]
    # R2[5] = R1[5] + R1[6] + R1[7] = (R0[5] + R0[6] + R0[7]) + (R0[6] + R0[7]) + R0[7] = R0[5] + 2 * R0[6] + 3 * R0[7]
    # R3[5] = R2[5] + R2[6] + R2[7] = (R0[5] + 2 * R0[6] + 3 * R0[7]) + (R0[6] + 2 * R0[7]) + R0[7] = R0[5] + 3 * R0[6] + 6 * R0[7]
    # R4[5] = R3[5] + R3[6] + R3[7] = (R0[5] + 3 * R0[6] + 6 * R0[7]) + (R0[6] + 3 * R0[7]) + R0[7] = R0[5] + 4 * R0[6] + 10 * R0[7]
    # R5[5] = R4[5] + R4[6] + R4[7] = (R0[5] + 4 * R0[6] + 10 * R0[7]) + (R0[6] + 4 * R0[7]) + R0[7] = R0[5] + 5 * R0[6] + 15 * R0[7]
    # F(0) = 0
    # F(1) = 1
    # F(2) = F(1) + 2 = 3
    # F(3) = F(2) + 3 = 6
    # F(X) = F(X - 1) + X = X * ((X + 1) / 2)
    # RM[5] = R0[5] + M * R0[6] + F(M) * R0[7]
    #
    # R1[4] = R0[4] + R0[5] + R0[6] + R0[7]
    # R2[4] = R1[4] + R1[5] + R1[6] + R1[7] = (R0[4] + R0[5] + R0[6] + R0[7]) + (R0[5] + R0[6] + R0[7]) + (R0[6] + R0[7]) + R0[7] = R0[4] + 2 * R0[5] + 3 * R0[6] + 4 * R0[7]
    # R3[4] = R0[4] + 3 * R0[5] + 6 * R0[6] + 10 * R0[7]
    # R4[4] = R0[4] + 4 * R0[5] + 10 * R0[6] + 20 * R0[7]
    # R5[4] = R0[4] + 5 * R0[5] + 15 * R0[6] + 35 * R0[7]
    # R6[4] = R0[4] + 6 * R0[5] + 21 * R0[6] + 56 * R0[7]
    # RM[4] = R0[4] + M * R0[5] + F(M) * R0[6] + (F(M) + F(M - 1)) * R0[7]
    # F2(0) = 0
    # F2(1) = 1
    # F2(2) = 4 (2 * 2) +1
    # F2(3) = 10 (3 * 3,3) +1.3
    # F2(4) = 20 (4 * 5) +1.6
    # F2(5) = 35 (5 * 7) +2
    # F2(X) = ...
    # 2 => 2, 3 => 3,3, 4 => 5, 5 => 7, 6 => 9.3
    #
    # R1[3] = R0[3] + R0[4] + R0[5] + R0[6]
    # R2[3] = R1[3] + R1[4] + R1[5] + R1[6] =
    # = (R0[3] + R0[4] + R0[5] + R0[6]) + (R0[4] + R0[5] + R0[6] + R0[7]) + (R0[5] + R0[6] + R0[7]) + (R0[6] + R0[7]) =
    # = R0[3] + 2 * R0[4] + 3 * R0[5] + 4 * R0[6] + 3 * R0[7]
    # 
    # Let's say
    # 3)  0  0  0  1  1  1  1  1
    #
    # R1[3] = R0[3] + R0[4] + R0[5] + R0[6] + R0[7]
    # R2[3] = R0[3] + 2 * R0[4] + 3 * R0[5] + 4 * R0[6] + 5 * R0[7]
    # R3[3] = R0[3] + 3 * R0[4] + 6 * R0[5] + 10 * R0[6] + 15 * R0[7]
    # R4[3] = R0[3] + 4 * R0[4] + 10 * R0[5] + 20 * R0[6] + 35 * R0[7]
    # RM[3] = R0[3] + M * R0[4] + F(M) * R0[5] + (F(M) + F(M - 1)) * R0[6] + 35 * R0[7]
    #
    # https://en.wikipedia.org/wiki/Binomial_theorem

    $offset = [int]$signal.substring(0, 7)

    $finalSignalLength = $Signal.Length * $Repeats
    if (($finalSignalLength / 2) -ge $offset) {
        throw "Not supported offset"
    }

    $signalArray = @()
    for ($index = 0; $index -lt $Signal.Length; $index++) {
        $signalArray += [int]($Signal[$index]) - 48
    }


    $prepareBinExps = $finalSignalLength - $offset
    Write-Verbose "$(Get-Date) Preparing '$prepareBinExps' binomical exponents"
    $binExps = New-Object "int[]" $prepareBinExps
    for ($i = 0; $i -lt $prepareBinExps; $i++) {
        $binExps[$i] = Get-BinonExpMod10 ($Phases + $i - 1) $i
        if ((($i + 1) % 10000) -eq 0) {
            Write-Verbose "$(Get-Date) '$($i + 1)'/'$prepareBinExps' binomical exponents prepared"
        }
    }
    Write-Verbose "$(Get-Date) Binomical exponents ready"

    $res = ""
    for ($l = 0; $l -lt $Length; $l++) {
        Write-Verbose "$(Get-Date) Calculating [$l] signal output"
        $resI = 0
        $offsetToGo = $finalSignalLength - $offset - $l
        for ($i = 0; $i -lt $offsetToGo; $i++) {
            $signalIndex = ($offset + $i + $l) % $Signal.Length
            # $exp = (Get-BinonExpMod10 ($Phases + $i - 1) $i)
            $exp = $binExps[$i]
            $resI = ($resI + ($signalArray[$signalIndex] * $exp)) % 10
            # write-host "'$exp' ($((100 + $i)) choose $i) * [$signalIndex]"
        }
        $res += $resI
    }
    Write-Verbose "$(Get-Date) Done: '$res'"
    
    return $res
}

# https://en.wikipedia.org/wiki/Binomial_coefficient#Binomial_coefficient_in_programming_languages
# https://fishi.devtail.io/weblog/2015/06/25/computing-large-binomial-coefficients-modulo-prime-non-prime/
function Get-PrimeDegree {
    [CmdletBinding()]
    param(
        [int]
        $number,
        [int]
        $p
    )

    [int]$degree = 0
    [int]$pPower = $p
    while ($pPower -le $number) {
        $degree += [Math]::Floor($number / $pPower)
        $pPower *= $p
    }

    return $degree
}

function Get-BinonExpModPrime {
    [CmdletBinding()]
    param(
        [int]
        $n,

        [int]
        $k,

        [int]
        $p
    )

    $k = [Math]::Min($k, $n - $k)

    if (((Get-PrimeDegree $n $p) - (Get-PrimeDegree ($n - $k) $p)) -gt (Get-PrimeDegree $k $p)) {
        return 0
    }
    
    $dividendFact = 1
    for ($i = $n; $i -gt $n - $k; $i--) {
        $d = $i
        while (-not ($d % $p)) {
            $d = $d / $p
        }
        $dividendFact = ($dividendFact * $d) % $p
        # Write-Host "* $d = $dividendFact"
    }

    # Write-Host "Divident: $dividendFact"
    $divisorFact = 1
    for ($i = 1; $i -le $k; $i++) {
        $d = $i
        while (-not ($d % $p)) {
            $d = $d / $p
        }
        $divisorFact = ($divisorFact * $d) % $p
        # Write-Host "* $d = $divisorFact"
    }
    # Write-Host "Divisor: $divisorFact"
    $power = $divisorFact
    if ($p -lt 3) {
        $divisorFact = 1
    } else {
        for ($i = 0; $i -lt $p - 3; $i++) {
            $divisorFact = ($divisorFact * $power) % $p
        }
    }
    # Write-Host "Divisor^$($p-2): $divisorFact"

    $res = ($dividendFact * $divisorFact) % $p
    return $res
}

function Get-BinonExpMod10 {
    [CmdletBinding()]
    param(
        [int]
        $n,

        [int]
        $k
    )

    # Chinese remainder theorem
    $a = Get-BinonExpModPrime $n $k 2
    $b = Get-BinonExpModPrime $n $k 5

    if (($b % 2) -eq $a) {
        return $b
    }

    return $b + 5
}

function Get-Part2Result {
    $signal = "59728776137831964407973962002190906766322659303479564518502254685706025795824872901465838782474078135479504351754597318603898249365886373257507600323820091333924823533976723324070520961217627430323336204524247721593859226704485849491418129908885940064664115882392043975997862502832791753443475733972832341211432322108298512512553114533929906718683734211778737511609226184538973092804715035096933160826733751936056316586618837326144846607181591957802127283758478256860673616576061374687104534470102346796536051507583471850382678959394486801952841777641763547422116981527264877636892414006855332078225310912793451227305425976335026620670455240087933409"
    
    $res = Clear-RepeatedSignalPhases $signal 10000 100 8 -Verbose
    $res

    # 84079620 - too high
    # 84024125 - correct
}
