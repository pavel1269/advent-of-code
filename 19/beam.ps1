
. ..\09\intComp.ps1

function Print-Beam {
    [CmdletBinding()]
    param($map, $count = $map.Count)
    
    for ($y = 0; $y -lt $count; $y++) {
        $line = ""
        for ($x = 0; $x -lt $count; $x++) {
            switch ($map[$y][$x]) {
                1 {
                    $line += "#"
                }
                -1 {
                    $line += "."
                }
                default {
                    $line += "-"
                }
            }
        }
        Write-Host $line
    }
}

function Get-Part1Res {
    [CmdletBinding()]
    param()

    $code = Get-Content ".\beamProgram.txt"
    $OpCode = $code.Split(',') | ForEach-Object { [int64]$_}
    
    $toScan = 50
    $map = New-Object "int[][]" $toScan
    for ($index = 0; $index -lt $toScan; $index++) {
        $map[$index] = New-Object "int[]" $toScan
    }

    for ($y = 0; $y -lt $toScan; $y++) {
        for ($x = 0; $x -lt $toScan; $x++) {
            $res = IntComp $OpCode @($x, $y)
            $lastResult = $res.Outputs[0]
            Write-Host "$(Get-Date) Scanning [$y][$x], res: '$lastResult'"
            
            if ($lastResult -eq 1) {
                $map[$y][$x] = 1
            }
            else {
                $map[$y][$x] = -1
            }
        }
    }

    Print-Beam $map

    (($map | ForEach-Object { $_ } ) | Where-Object { $_ -eq 1 }).Coun
    # 229 - correct

}
