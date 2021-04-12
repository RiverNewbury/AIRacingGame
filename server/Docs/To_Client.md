# What a JSON response to client looks like :

Json(SimulationHistory {
    history: [
        Car { pos: Point { x: 1.0, y: 1.0 },
        angle: 0.0, speed: 0.0,
        max_speed: 1.0, max_acc: 1.0, max_dec: 1.0 },
        Car { pos: Point { x: 1.5, y: 1.5 },
        angle: 45.0, speed: 3.0,
        max_speed: 1.0, max_acc: 1.0, max_dec: 1.0 },
        Car { pos: Point { x: 3.5, y: 3.5 },
        angle: 90.0, speed: 12.0,
        max_speed: 1.0, max_acc: 1.0, max_dec: 1.0 }],
    tps: 100 },
    Score { successful: true, time: 129 }))


Car objects should be in the order they occured - ie start is at the beginning of the history array
Each Car reprents the car each tick ie the first one resprents car at tick 1 (so has potentially moved from the start) and the second is at tick 2

tps tells how many ticks per second for the client to use to help simulate

Sucessful tells you if the car crashed

```assert(time = history.length)
```
 ie is number of ticks that it has taken to go round the course
