

. ./moons.ps1

$VerbosePreference = "SilentlyContinue"

function Assert {
    [CmdletBinding()]
    param($a, $b)

    if ($a.Length -ne $b.Length) {
        Write-Error "Strings have different length, '$($a.Length)' vs '$($b.Length)'" -ErrorAction "stop"
    }

    for ($index = 0; $index -lt $a.Length; $index++) {
        if ($a[$index] -ne $b[$index]) {
            Write-Error "Differece at position '$index', '$($a[$index])' vs '$($b[$index])'" -ErrorAction "stop"
        }
    }
}

# <x=-1, y=0, z=2>
# <x=2, y=-10, z=-7>
# <x=4, y=-8, z=8>
# <x=3, y=5, z=-1>
$moons = @(
    @{ x = -1; y = 0; z = 2 }
    @{ x = 2; y = -10; z = -7 }
    @{ x = 4; y = -8; z = 8 }
    @{ x = 3; y = 5; z = -1 }
)
$velocity = @(
    @{ x = 0; y = 0; z = 0 }
    @{ x = 0; y = 0; z = 0 }
    @{ x = 0; y = 0; z = 0 }
    @{ x = 0; y = 0; z = 0 }
)
Simulate-Steps $moons $velocity 10
Assert (Sum-Energy $moons $velocity) 179

# <x=-8, y=-10, z=0>
# <x=5, y=5, z=10>
# <x=2, y=-7, z=3>
# <x=9, y=-8, z=-3>

$moons = @(
    @{ x = -8; y = -10; z = 0 }
    @{ x = 5; y = 5; z = 10 }
    @{ x = 2; y = -7; z = 3 }
    @{ x = 9; y = -8; z = -3 }
)
$velocity = @(
    @{ x = 0; y = 0; z = 0 }
    @{ x = 0; y = 0; z = 0 }
    @{ x = 0; y = 0; z = 0 }
    @{ x = 0; y = 0; z = 0 }
)
Simulate-Steps $moons $velocity 100
Assert (Sum-Energy $moons $velocity) 1940
