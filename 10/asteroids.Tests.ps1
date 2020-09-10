
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

$shots = ShootDown-Asteroids $mapString 11 13

# The 1st asteroid to be vaporized is at 11,12.
# The 2nd asteroid to be vaporized is at 12,1.
# The 3rd asteroid to be vaporized is at 12,2.
# The 10th asteroid to be vaporized is at 12,8.
# The 20th asteroid to be vaporized is at 16,0.
# The 50th asteroid to be vaporized is at 16,9.
# The 100th asteroid to be vaporized is at 10,16.
# The 199th asteroid to be vaporized is at 9,6.
# The 200th asteroid to be vaporized is at 8,2.
# The 201st asteroid to be vaporized is at 10,9.
# The 299th and final asteroid to be vaporized is at 11,1.
Assert "11,12" "$($shots[0].x),$($shots[0].y)"
Assert "12,1" "$($shots[1].x),$($shots[1].y)"
Assert "12,2" "$($shots[1].x),$($shots[2].y)"
Assert "12,8" "$($shots[9].x),$($shots[9].y)"
Assert "16,0" "$($shots[19].x),$($shots[19].y)"
Assert "16,9" "$($shots[49].x),$($shots[49].y)"
Assert "10,16" "$($shots[99].x),$($shots[99].y)"
Assert "9,6" "$($shots[198].x),$($shots[198].y)"
Assert "8,2" "$($shots[199].x),$($shots[199].y)"
Assert "10,9" "$($shots[200].x),$($shots[200].y)"
Assert "11,1" "$($shots[298].x),$($shots[298].y)"
Assert "299" "$($shots.Count)"
