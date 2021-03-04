# AIRacingGame

## Targets

 - -points based on damage to car
 - wall physics


## To Server

 - Json message with code + username
 - Request leaderboard

## To Client 

 - Message with current pos, orientation and speed + timestamp

## Scripting Language

 - Python as basis
 - User makes a function which given the environment says what the car should be doing
 - Environment = {the whole map, velocity of car} 
 - Output = {acceleration value (fractional - how much the pedal is down), intended angle, turning speed}
  - values important to the car 
  - possible target volicity
  - Drift? bad thing
  - Gears?

