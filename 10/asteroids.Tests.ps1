
. ./asteroids.ps1

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

$mapString = ".#..#
.....
#####
....#
...##"
$res = Get-VisibilityMap ($mapString.Split())
$expected = ".7..7
.....
67775
....7
...87
"

Assert $expected $res

$mapString = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"
$res = Get-BestVisibility $mapString
Assert "5,8" "$($res.x),$($res.y)"

$mapString = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."
$res = Get-BestVisibility $mapString
Assert "1,2" "$($res.x),$($res.y)"

$mapString = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."
$res = Get-BestVisibility $mapString
Assert "6,3" "$($res.x),$($res.y)"

$mapString = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"
$res = Get-BestVisibility $mapString
Assert "11,13" "$($res.x),$($res.y)"
