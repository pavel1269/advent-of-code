
. ./intComp.ps1

function Amplify-Signal {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [int[]]
        $OpCodes
    )

    $max = 0
    for ($amp1 = 0; $amp1 -le 4; $amp1++) {
        $res1 = IntComp $code @($amp1, 0) -Verbose:$false
        for ($amp2 = 0; $amp2 -le 4; $amp2++) {
            if ($amp1 -eq $amp2) {
                continue
            }
            $res2 = IntComp $code @($amp2, $res1.Outputs[0]) -Verbose:$false
            for ($amp3 = 0; $amp3 -le 4; $amp3++) {
                if (($amp1 -eq $amp3) -or ($amp2 -eq $amp3)) {
                    continue
                }
                $res3 = IntComp $code @($amp3, $res2.Outputs[0]) -Verbose:$false
                for ($amp4 = 0; $amp4 -le 4; $amp4++) {
                    if (($amp1 -eq $amp4) -or ($amp2 -eq $amp4) -or ($amp3 -eq $amp4)) {
                        continue
                    }
                    $res4 = IntComp $code @($amp4, $res3.Outputs[0]) -Verbose:$false
                    for ($amp5 = 0; $amp5 -le 4; $amp5++) {
                        if (($amp1 -eq $amp5) -or ($amp2 -eq $amp5) -or ($amp3 -eq $amp5) -or ($amp4 -eq $amp5)) {
                            continue
                        }
                        $res5 = (IntComp $code @($amp5, $res4.Outputs[0]) -Verbose:$false).Outputs[0]
                        Write-Verbose "['$amp1','$amp2','$amp3','$amp4','$amp5'] = '$res5'"
                        if ($res5 -gt $max) {
                            $max = $res5
                        }
                    }
                }
            }
        }
    }

    return $max
}

function Amplify-SignalWithFeedback {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [int[]]
        $OpCodes
    )

    $max = 0
    for ($amp1 = 5; $amp1 -le 9; $amp1++) {
        for ($amp2 = 5; $amp2 -le 9; $amp2++) {
            if ($amp1 -eq $amp2) {
                continue
            }
            for ($amp3 = 5; $amp3 -le 9; $amp3++) {
                if (($amp1 -eq $amp3) -or ($amp2 -eq $amp3)) {
                    continue
                }
                for ($amp4 = 5; $amp4 -le 9; $amp4++) {
                    if (($amp1 -eq $amp4) -or ($amp2 -eq $amp4) -or ($amp3 -eq $amp4)) {
                        continue
                    }
                    for ($amp5 = 5; $amp5 -le 9; $amp5++) {
                        if (($amp1 -eq $amp5) -or ($amp2 -eq $amp5) -or ($amp3 -eq $amp5) -or ($amp4 -eq $amp5)) {
                            continue
                        }

                        $computers = @(
                            @{
                                OpCodeIndex = 0
                                OpCode = @($code)
                                InputParams = $amp1
                                Output = $null
                            }
                            @{
                                OpCodeIndex = 0
                                OpCode = @($code)
                                InputParams = $amp2
                                Output = $null
                            }
                            @{
                                OpCodeIndex = 0
                                OpCode = @($code)
                                InputParams = $amp3
                                Output = $null
                            }
                            @{
                                OpCodeIndex = 0
                                OpCode = @($code)
                                InputParams = $amp4
                                Output = $null
                            }
                            @{
                                OpCodeIndex = 0
                                OpCode = @($code)
                                InputParams = $amp5
                                Output = $null
                            }
                        )

                        for ($index = 0; $index -lt $computers.Count; $index++) {
                            $compData = $computers[$index]
                            $res = IntComp $compData.OpCode $compData.InputParams -Verbose:$false
                            $compData.OpCode = $res.OpCodes
                            $compData.OpCodeIndex = $res.OpCodeIndex
                        }

                        $lastResult = 0
                        $index = 0
                        $compData = $computers[$index]
                        while ($null -ne $compData.OpCodeIndex) {
                            $res = IntComp $compData.OpCode $lastResult $compData.OpCodeIndex -Verbose:$false
                            $compData.OpCodeIndex = $res.OpCodeIndex
                            $lastResult = $res.Outputs[0]

                            $index++
                            $index = $index % $computers.Count
                            $compData = $computers[$index]
                            # Write-Verbose "Iteration '$index', res: '$lastResult'"
                        }

                        Write-Verbose "['$amp1','$amp2','$amp3','$amp4','$amp5'] = '$lastResult'"
                        if ($lastResult -gt $max) {
                            $max = $lastResult
                        }
                    }
                }
            }
        }
    }

    return $max
}

function Get-ResultPart1 {
    $code = [int[]](Get-Content "./program-amplifiers.txt").Split(',')
    $res = Amplify-Signal $code 
    $res
    
    # 526307796 - too high
    # 206580 - correct
}

function Get-ResultPart2 {
    $code = [int[]](Get-Content "./program-amplifiers.txt").Split(',')
    $res = Amplify-SignalWithFeedback $code  -Verbose
    $res
    
    # 2299406 - correct
}
