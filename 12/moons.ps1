
function Simulate-Step {
    [CmdletBinding()]
    param([Hashtable[]]$moons, [Hashtable[]]$velocity)

    $count = $moons.Count
    for ($index1 = 0; $index1 -lt $count; $index1++) {
        for ($index2 = $index1 + 1; $index2 -lt $count; $index2++) {
            $moon1 = $moons[$index1]
            $moon2 = $moons[$index2]

            if ($moon1.x -lt $moon2.x) {
                $velocity[$index1].x++
                $velocity[$index2].x--
            } elseif ($moon2.x -lt $moon1.x) {
                $velocity[$index1].x--
                $velocity[$index2].x++
            }
            if ($moon1.y -lt $moon2.y) {
                $velocity[$index1].y++
                $velocity[$index2].y--
            } elseif ($moon2.y -lt $moon1.y) {
                $velocity[$index1].y--
                $velocity[$index2].y++
            }
            if ($moon1.z -lt $moon2.z) {
                $velocity[$index1].z++
                $velocity[$index2].z--
            } elseif ($moon2.z -lt $moon1.z) {
                $velocity[$index1].z--
                $velocity[$index2].z++
            }
        }
    }

    for ($index = 0; $index -lt $count; $index++) {
        $moons[$index].x += $velocity[$index].x
        $moons[$index].y += $velocity[$index].y
        $moons[$index].z += $velocity[$index].z
    }
}

function Simulate-Steps {
    [CmdletBinding()]
    param([Hashtable[]]$moons, [Hashtable[]]$velocity, [int]$steps)

    for ($index = 0; $index -lt $steps; $index++) {
        Simulate-Step $moons $velocity
    }
}

function Sum-Energy {
    [CmdletBinding()]
    param([Hashtable[]]$moons, [Hashtable[]]$velocity)

    $count = $moons.Count
    $energy = 0
    for ($index = 0; $index -lt $count; $index++) {
        $moon = $moons[$index]
        $vel = $velocity[$index]

        $actPotential = [Math]::Abs($moon.x) + [Math]::Abs($moon.y) + [Math]::Abs($moon.z)
        $actKinetic = [Math]::Abs($vel.x) + [Math]::Abs($vel.y) + [Math]::Abs($vel.z)
        $sum = $actPotential * $actKinetic

        Write-Verbose "Potential: '$actPotential', Kinetic: '$actKinetic', Sum: '$sum'"
        $energy += $sum
    }

    Write-Verbose "Total energy: '$energy'"
    return $energy
}

function Print-Status {
    [CmdletBinding()]
    param([Hashtable[]]$moons, [Hashtable[]]$velocity)

    $count = $moons.Count
    for ($index = 0; $index -lt $count; $index++) {
        $moon = $moons[$index]
        $vel = $velocity[$index]

        Write-Host "pos=<x=$($moon.x.ToString().PadLeft(3, ' ')), y=$($moon.y.ToString().PadLeft(3, ' ')), z=$($moon.z.ToString().PadLeft(3, ' '))>, vel=<x=$($vel.x.ToString().PadLeft(3, ' ')), y=$($vel.y.ToString().PadLeft(3, ' ')), z=$($vel.z.ToString().PadLeft(3, ' '))>"
    }

    Sum-Energy $moons $velocity -Verbose | Out-Null
}

function Get-Part1Result {
    [CmdletBinding()]
    param()

    # <x=14, y=9, z=14>
    # <x=9, y=11, z=6>
    # <x=-6, y=14, z=-4>
    # <x=4, y=-4, z=-3>
    $moons = @(
        @{ x = 14; y = 9; z = 14 }
        @{ x = 9; y = 11; z = 6 }
        @{ x = -6; y = 14; z = -4 }
        @{ x = 4; y = -4; z = -3 }
    )
    $velocity = @(
        @{ x = 0; y = 0; z = 0 }
        @{ x = 0; y = 0; z = 0 }
        @{ x = 0; y = 0; z = 0 }
        @{ x = 0; y = 0; z = 0 }
    )
    
    Simulate-Steps $moons $velocity 1000
    Write-Host "Result: '$(Sum-Energy $moons $velocity)'"

    # 6065 - too low
    # 9999 - correct
}
