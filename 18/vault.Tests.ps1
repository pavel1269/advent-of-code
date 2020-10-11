
. ./vault.ps1

$VerbosePreference = "SilentlyContinue"

function Assert {
    param($a, $b)

    if ($a -ne $b) {
        throw "'$a' differs from '$b'"
    }
}

$stringMap =
"#########
#b.A.@.a#
#########"
Assert (Collect-AllKeysSteps $stringMap) 8

$stringMap =
"########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"
Assert (Collect-AllKeysSteps $stringMap) 86

$stringMap =
"########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################"
Assert (Collect-AllKeysSteps $stringMap) 132

$stringMap =
"#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################"
# Assert (Collect-AllKeysSteps $stringMap) 136

$stringMap =
"########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################"
Assert (Collect-AllKeysSteps $stringMap) 81

$stringMap = 
"#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######"
Assert (Collect-AllKeysSteps4Way $stringMap) 8

$stringMap =
"###############
#d.ABC.#.....a#
######.#.######
#######@#######
######.#.######
#b.....#.....c#
###############"
Assert (Collect-AllKeysSteps4Way $stringMap) 24

$stringMap =
"#############
#DcBa.#.GhKl#
#.###.#.#I###
#e#d##@##j#k#
###C#.#.###J#
#fEbA.#.FgHi#
#############"
Assert (Collect-AllKeysSteps4Way $stringMap) 32

$stringMap =
"#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba.#.BcIJ#
######@######
#nK.L.#.G...#
#M###N#H###.#
#o#m..#i#jk.#
#############"
Assert (Collect-AllKeysSteps4Way $stringMap) 72
