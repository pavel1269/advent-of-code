
. ./donut.ps1

function Assert {
    param($a, $b)

    if ($a -ne $b) {
        throw "'$a' differs from '$b'"
    }
}

$stringMap = 
"         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       "

$state = Parse-Map $stringMap
Assert $state.start.x 2
Assert $state.start.y 2
Assert $state.end.x 18
Assert $state.end.y 16
Assert $state.innerStart.x 7
Assert $state.innerStart.y 7
Assert $state.innerEnd.x 13
Assert $state.innerEnd.y 11
Assert $state.Portals.Count 6
Assert $state.startPos.x 9
Assert $state.startPos.y 2
Assert $state.endPos.x 13
Assert $state.endPos.y 16
Assert (Traverse-Map $state) 23

$stringMap = 
"                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               "

$state = Parse-Map $stringMap
Assert $state.start.x 2
Assert $state.start.y 2
Assert $state.end.x 32
Assert $state.end.y 34
Assert $state.innerStart.x 9
Assert $state.innerStart.y 9
Assert $state.innerEnd.x 25
Assert $state.innerEnd.y 27
Assert $state.Portals.Count 20
Assert $state.startPos.x 19
Assert $state.startPos.y 2
Assert $state.endPos.x 2
Assert $state.endPos.y 17
Assert (Traverse-Map $state) 58
