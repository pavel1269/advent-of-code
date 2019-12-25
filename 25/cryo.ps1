
. ..\09\intComp.ps1

function Manual-RobotControl {
    [CmdletBinding()]
    param()

    $OpCode = Load-IntCompProgram "./program.txt"

    $compInputs = @{
        OpCodeIndex = 0
        OpCodes = $OpCode
        InputParams = @()
    }
    
    do {
        Write-Host "Index: '$($compInputs.OpCodeIndex)', input: '$($hostInput)' ($([string]::Join(" ", $compInputs.InputParams)))"

        $res = IntComp @compInputs

        $printableOut = [string][char[]][int[]]$res.Outputs
        write-host $printableOut

        if ($res.OpCodeIndex) {
            $compInputs = $res
            $compInputs.Remove("Outputs")

            $compInputs.InputParams = @()
            $hostInput = Read-Host
            $compInputs.InputParams += [int[]][char[]]$hostInput
            $compInputs.InputParams += [int][char]"`n"
        }
    } while ($res.OpCodeIndex)
}

function Get-Part1Result {
    [CmdletBinding()]
    param()

    Manual-RobotControl

    # start - east - .weather machine
    # start - south - !photons - south - ---
    # start - west - !giant electromagnet - #1 Navigation
    # 1 - west - .bowl of rice
    # 1 - north - #2 Kitchen, .polygon
    # 2 - east - .hypercube - south - .dark matter - west - ---
    # 2 - north - #3 Hallway, .candy cane
    # 3 - north - !escape pod
    # 3 - west - #4, Crew Quarters, ?molten lava
    # 4 - north - manifold - west - !infinite loop
    # 4 - west - #5, Stables
    # 5 - south - ---
    # 5 - north - ?dehydrated water - west - south - final door

    # manifold 1
    # dehydrated water 10
    # polygon 100
    # weather machine 1000

    # bowl of rice 1 0000
    # hypercube 10 0000
    # candy cane 100 0000
    # dark matter 1000 0000

    # 1111 1101 - too heavy
    # 1100 1000 - too heavy
    # 1100 0100 - too heavy
    # 1111 0010 - too heavy
    # 1111 0001 - too heavy
    # 1111 0000 - too light

    # 1110 0010 - too light
    # 1011 1111 - too light
    # 1011 1101 - too light

    # 1101 1111 - too heavy
    # 1101 1101 - too heavy
    # 1101 0010 - correct

    # 1100 1101 - too heavy
    # 1100 1000 - too heavy
    # 1100 0101 - too heavy
    # 1100 0100 - too heavy
    # 1100 0010 - too light
    # 1100 0001 - too heavy
    # 1100 0000 - too light

    # 0111 1111 - too light
    # 0111 1101 - too light
    # 0011 1101 - too light
    # 0001 1101 - too light
    # 0000 1101 - too light
    # 0000 1001 - too light
    # 0000 1000 - too light
    # 0000 0100 - too light
    # 0000 0001 - too light
    
    # A   l o u d ,   r o b o t i c   v o i c e   s a y s   " A n a l y s i s   c o m p l e t e !   Y o u   m a y   p r o c e e d . "   a n d   y o u   e n t e r   t h e   c o c k p i t .
    # S a n t a   n o t i c e s   y o u r   s m a l l   d r o i d ,   l o o k s   p u z z l e d   f o r   a   m o m e n t ,   r e a l i z e s   w h a t   h a s   h a p p e n e d ,   a n d   r a d i o s
    # y o u r   s h i p   d i r e c t l y .
    # " O h ,   h e l l o !   Y o u   s h o u l d   b e   a b l e   t o   g e t   i n   b y   t y p i n g   1 0 5 0 4 1 9 2   o n   t h e   k e y p a d   a t   t h e   m a i n   a i r l o c k . "

    # 10504192 - correct
}
