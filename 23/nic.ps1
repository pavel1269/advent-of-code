
. ..\09\intComp.ps1

function Get-Part1Result {
    [CmdletBinding()]
    param()

    $OpCode = Load-IntCompProgram "./nic.txt"

    $computers = New-Object "Object[]" 50
    for ($index = 0; $index -lt $computers.Count; $index++) {
        $computers[$index] = @{
            OpCodes = $OpCode.Clone()
            OpCodeIndex = 0
            InputParams = $index
        }
    }

    for ($iteration = 0; $iteration -lt 10000; $iteration++) {
        $outputs = New-Object "System.Collections.ArrayList"
        for ($index = 0; $index -lt $computers.Count; $index++) {
            $comp = $computers[$index]
            if ($comp.InputParams -is [Array] -and $comp.InputParams.Count -eq 0) {
                $comp.InputParams = @(-1)
            }

            Write-Host "$(Get-Date) Comp $index ('$iteration') start (input: '$([string]::Join(" ", $comp.InputParam))')"
            $res = IntComp @comp

            $computers[$index] = @{
                OpCodes = $res.OpCodes
                OpCodeIndex = $res.OpCodeIndex
                InputParams = @()
            }

            if ($res.Outputs) {
                Write-host "$(Get-Date) '$index' outputs: '$([string]::Join(" ", $res.Outputs))'"
                for ($outIndex = 0; $outIndex -lt $res.Outputs.Count; $outIndex += 3) {
                    $outputs.Add(@{
                        address = $res.Outputs[$outIndex]
                        x = $res.Outputs[$outIndex + 1]
                        y = $res.Outputs[$outIndex + 2]
                    }) | Out-Null
                }
            }
            Write-Host "$(Get-Date) Comp $index ('$iteration') end"
        }

        for ($index = 0; $index -lt $outputs.Count; $index++) {
            $output = $outputs[$index]
            if ($output.address -lt 0 -or $output.address -ge $computers.Count) {
                throw "Invalid address '$($output.address)', val: '$($output.x)' '$($output.y)'"
            }
            else {
                $computers[$output.address].InputParams += $output.x
                $computers[$output.address].InputParams += $output.y
            }
        }
    }

    # Invalid address '255', val: '33461' '23213'
    # 23213 - correct
}

function Get-Part2Result {
    [CmdletBinding()]
    param()

    $OpCode = Load-IntCompProgram "./nic.txt"

    $computers = New-Object "Object[]" 50
    for ($index = 0; $index -lt $computers.Count; $index++) {
        $computers[$index] = @{
            OpCodes = $OpCode.Clone()
            OpCodeIndex = 0
            InputParams = $index
        }
    }

    $nat = @{
        x = 0
        y = 0
    }
    $lastY = $null
    for ($iteration = 0; $iteration -lt 10000; $iteration++) {
        Write-Host "$(Get-Date) '$iteration' start"
        $outputs = New-Object "System.Collections.ArrayList"
        for ($index = 0; $index -lt $computers.Count; $index++) {
            $comp = $computers[$index]
            if ($comp.InputParams -is [Array] -and $comp.InputParams.Count -eq 0) {
                $comp.InputParams = @(-1)
            }

            # Write-Host "$(Get-Date) Comp $index ('$iteration') start (input: '$([string]::Join(" ", $comp.InputParam))')"
            $res = IntComp @comp

            $computers[$index] = @{
                OpCodes = $res.OpCodes
                OpCodeIndex = $res.OpCodeIndex
                InputParams = @()
            }

            if ($res.Outputs) {
                # Write-host "$(Get-Date) '$index' outputs: '$([string]::Join(" ", $res.Outputs))'"
                for ($outIndex = 0; $outIndex -lt $res.Outputs.Count; $outIndex += 3) {
                    $outputs.Add(@{
                        address = $res.Outputs[$outIndex]
                        x = $res.Outputs[$outIndex + 1]
                        y = $res.Outputs[$outIndex + 2]
                    }) | Out-Null
                }
            }
            # Write-Host "$(Get-Date) Comp $index ('$iteration') end"
        }

        if ($iteration -gt 0 -and $outputs.Count -eq 0) {
            $outputs.Add(@{
                address = 0
                x = $nat.x
                y = $nat.y
            }) | Out-Null

            Write-Host "$(Get-Date) '$iteration' NAT sends '$($nat.x)' '$($nat.y)'"

            if ($null -eq $lastY) {
                $lastY = $nat.y
            }
            elseif ($lastY -eq $nat.y) {
                throw "Duplicate, '$lastY'"
            }
            else {
                $lastY = $nat.y
            }
        }

        for ($index = 0; $index -lt $outputs.Count; $index++) {
            $output = $outputs[$index]
            if ($output.address -lt 0 -or $output.address -ge $computers.Count) {
                if ($outputs.address -eq 255) {
                    $nat.x = $output.x
                    $nat.y = $output.y
                }
                else {
                    throw "Invalid address '$($output.address)', val: '$($output.x)' '$($output.y)'"
                }
            }
            else {
                $computers[$output.address].InputParams += $output.x
                $computers[$output.address].InputParams += $output.y
            }
        }
    }

    # 12/23/2019 18:51:38 '130' NAT sends '33461' '17874'
    # 17874 - correct
}
