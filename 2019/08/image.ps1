
function Get-Part1Result {
    [CmdletBinding()]
    param()

    $content = Get-Content -Path "./input.txt" -Raw

    $width = 25
    $length = 6
    $layerSize = $width * $length

    $layersCount = [int]($content.Length / $layerSize)
    $layers = @()
    for ($layer = 0; $layer -lt $layersCount; $layer++) {
        $layers += $content.Substring($layer * $layerSize, $layerSize)
    }

    $minZeroes = $layerSize + 1
    $minZeroesIndex = -1
    for ($layer = 0; $layer -lt $layersCount; $layer++) {
        $zeroes = $layerSize - $layers[$layer].Replace('0', '').Length
        if ($zeroes -lt $minZeroes) {
            $minZeroes = $zeroes
            $minZeroesIndex = $layer
        }
    }

    $ones = $layerSize - $layers[$minZeroesIndex].Replace('1', '').Length
    $twos = $layerSize - $layers[$minZeroesIndex].Replace('2', '').Length

    Write-Host "Result: $($ones * $twos)"

    # 1340 - correct
    # 2240 - too high
}

function Get-Part2Result {
    [CmdletBinding()]
    param()

    $content = Get-Content -Path "./input.txt" -Raw

    $width = 25
    $length = 6
    $layerSize = $width * $length

    $layersCount = [int]($content.Length / $layerSize)
    $layers = @()
    for ($layer = 0; $layer -lt $layersCount; $layer++) {
        $layers += $content.Substring($layer * $layerSize, $layerSize)
    }

    $final = ""
    for ($imageIndex = 0; $imageIndex -lt $layerSize; $imageIndex++) {
        $res = "2"
        for ($layer = 0; $layer -lt $layersCount; $layer++) {
            if ($layers[$layer][$imageIndex] -eq '2') {
                continue
            }

            if ($layers[$layer][$imageIndex] -eq '0') {
                $res = "."
            } else {
                $res = "#"
            }
            #$res = $layers[$layer][$imageIndex]
            break
        }

        $final += $res
    }

    for ($row = 0; $row -lt $length; $row++) {
        write-host ($final.Substring($row * $width, $width))
    }

    # #....####...##.#..#..##..
    # #....#.......#.#.#..#..#.
    # #....###.....#.##...#....
    # #....#.......#.#.#..#....
    # #....#....#..#.#.#..#..#.
    # ####.####..##..#..#..##..

    # 1000011110001101001001100
    # 1000010000000101010010010
    # 1000011100000101100010000
    # 1000010000000101010010000
    # 1000010000100101010010010
    # 1111011110011001001001100

    # LEJKC - correct
}
