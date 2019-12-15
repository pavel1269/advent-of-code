
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

function Get-ResultPart1 {
    $code = [int[]](Get-Content "./program-amplifiers.txt").Split(',')
    $res = Amplify-Signal $code 
    $res
    
    # 526307796 - too high
    # 206580 - correct
}
