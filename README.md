# AIRacingGame

## Targets

 - -points based on damage to car
 - wall physics


## To Server
 - In Rust
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



## Time Plan
 + River
   - <del>Finish the work on Exec Environment</del>
   - <del>Treat car as a rectangle as opposed to a point</del>
   - Work out how the acceleration curves for the car should work (turb vs not)
   - Come up with a more complex damage system for the car
   - Research into gears and Drifting


 + Beth
   - <del>Get initial implementation of leader board done</del>
   - Add in scene transitions
   - Add in way of visualising lap finish/crash
   - Work out how the FE wants to receive the racetrack

 + Max
   - <del>Finish coding scripting language branch</del>
   - Add functionality on server side to send out the racetrack
   -<del> Do a terrible initial example of a racecar AI to demonstrate it working</del>

 + Luca
   - <del>Finish off the server facing bit of the front end</del>
   - Help beth with the server facing part of the client

 + General Front end
   - Improve visuals : make the car not a sphere + add skins for car?
   - Market research

 + General Back End
   - Do docs for scripting language
