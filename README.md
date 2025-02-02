# LinkedIn Puzzle Game Helpers

## Queens

- A quantum solver that uses D-WAVE's CQM (Constrained Quadratic Model) quantum annealer
- Flow:
    - Read the input file and build a dictionary of regions
    - Create a `ConstrainedQuadraticModel` and add contraints
        - Only one queen per row and column
        - No 2 queens adjacent to each other
        - No 2 queens in the same region
    - Use the `LeapHybridCQMSampler` and find solutions (there should be only one)
    - Print the solution
- Input: `problems.txt` (grid where 2 spots having the same number mean the same region)

## Tango

### Quantum

- A quantum solver that uses D-WAVE's CQM (Constrained Quadratic Model) quantum annealer
- Flow:
    - Read the input file and find fixed suns/moons and links (x and =)
    - Create a `ConstrainedQuadraticModel` and add contraints
        - Fixed suns/moons
        - Same number of suns and moons in each row and column
        - No 2 suns or moons adjacent to each other in a row or column
        - Links
    - Use the `LeapHybridCQMSampler` and find solutions (there should be only one)
    - Print the solution
- Input: `problems.txt`
    - Grid where S means sun, M means moon, comma (only) separated
    - Afterwards, links are in the format: x1,y1,x2,y2,[link] where [link] is either `x` or `=`

### Brute Force

- GUI using Macroquad
- Usage:
    - Left click on a square to toggle between sun, moon, and empty
    - Right click on a square then an adjacent square to toggle between `x` and `=`
    - Press space to trigger the solver
- It's really really slow though