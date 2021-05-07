# Information / Specification of the scripting language

User-submitted code is expected to be valid Python. More than that, though - we're expecting a
couple particular items to be there.

Typically, code will look something like:

```python
class CarCommand:
    # Constructs the `CarCommand` object, optionally setting the
    # acceleration and steering
    def __init__(self, acc: float = 0.0, steering: float = 0.0):
        self.acc = acc
        self.steering = steering

def outputs(car):
	# Logic for calculating what to do :)
	#
	# For this example, we'll just go straight:
	return CarCommand(acc = 0.5, steering = 0.0)
```

It should have a function `outputs`, taking a single argument as input. `outputs` is called
every so often in order to determine what the car should do. The input, `car` has a couple of fields
describing it's state:

* `pos`: a point with fields `x` and `y` to give the current coordinates,
* `angle`: the direction the car is facing, in radians
* `speed`: the current speed of the car, from zero to 1
* `dist_to_wall`: for a range of angles from left to right, the distance to the wall at that angle

The distances to the nearest wall are provided as a list of floats, with the the first value
corresponding to the angle pointing directly left, and the last value pointing directly to the
right.

The returned value from the function doesn't *need* to be a `CarCommand` -- any class with `acc` and
`steering` fields will do. The `acc` field is a float from `-1` to `1` where a value of `1`
corresponds to speeding up as much as possible and `-1` to slowing down as much as possible. The
`steering` field gives a float -- also from `-1` to `1` -- that controls how far to turn the wheel,
where `-1` is all the way to the left, and `1` is all the way to the right.

As the car speeds up, the amount it's able to turn decreases -- so be careful while driving quickly!
