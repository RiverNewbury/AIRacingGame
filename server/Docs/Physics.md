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
 - The car has a max speed and acc and dec and it is always able to go to those acc and dec no matter the Speed
 - The car is incapable of reversing
 - The car can instantly snap to whatever angle is required
