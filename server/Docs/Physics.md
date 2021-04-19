# Documentation of Physics Engine

## Values that the user can control
- Output = {acceleration value (fractional - how much the pedal is down), final speed, intended angle, turning speed}
 - values important to the car
 - possible target volicity
 - Drift? bad thing
 - Gears?

## Things to Note
 - The users affect on the car happen at the start of the tick (before calculating new position)


## Current assumptions
 - The car's max acc and dec are linearly dependant on speed
    - So if 0% speed it has 100% of max acceleration and 0% max deceleration
    - So if 50% speed it has 50% of max acceleration and 50% max deceleration
    - So if 80% speed it has 20% of max acceleration and 80% max deceleration
