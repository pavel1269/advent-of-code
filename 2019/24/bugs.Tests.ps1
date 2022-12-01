
. ./bugs.ps1

$VerbosePreference = "SilentlyContinue"

function Assert {
    param($a, $b)

    if ($a -ne $b) {
        Write-Error "'$a' differs from '$b'" -ErrorAction "Stop"
    }
}

Assert (Get-BioDiversity @(
    ,@(0, 0, 0, 0, 0)
    ,@(0, 0, 0, 0, 0)
    ,@(0, 0, 0, 0, 0)
    ,@(1, 0, 0, 0, 0)
    ,@(0, 1, 0, 0, 0)
)) 2129920

$map = @(
    ,@(0, 0, 0, 0, 1)
    ,@(1, 0, 0, 1, 0)
    ,@(1, 0, 0, 1, 1)
    ,@(0, 0, 1, 0, 0)
    ,@(1, 0, 0, 0, 0)
)
Assert (Get-BiodiversityRating $map) 2129920

$map = @(
    ,@(0, 0, 0, 0, 1)
    ,@(1, 0, 0, 1, 0)
    ,@(1, 0, 0, 1, 1)
    ,@(0, 0, 1, 0, 0)
    ,@(1, 0, 0, 0, 0)
)
$space = @{}
$space.0 = $map
$newSpace = Monitor-Space $space 10
Assert (Get-NumberOfBugsInSpace $newSpace) 99
