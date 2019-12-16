
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
        $Phases = 100
    )

    $finalSignal = ""
    Write-Verbose "$(Get-Date) Preparing signal"
    for ($index = 0; $index -lt $Repeats; $index++) {
        $finalSignal += $signal
    }
    Write-Verbose "$(Get-Date) Multiplied signal, now it has '$($finalSignal.Length)' characters"

    $res = Clear-SignalPhases $finalSignal
    return $res
}

function Get-Part2Result {
    $signal = "59728776137831964407973962002190906766322659303479564518502254685706025795824872901465838782474078135479504351754597318603898249365886373257507600323820091333924823533976723324070520961217627430323336204524247721593859226704485849491418129908885940064664115882392043975997862502832791753443475733972832341211432322108298512512553114533929906718683734211778737511609226184538973092804715035096933160826733751936056316586618837326144846607181591957802127283758478256860673616576061374687104534470102346796536051507583471850382678959394486801952841777641763547422116981527264877636892414006855332078225310912793451227305425976335026620670455240087933409"
    $res = Clear-RepeatedSignalPhases $signal
}
