# This is our test python code!

class CarCommand:
    # Constructs the `CarCommand` object, optionally setting the
    # acceleration and turning speed
    def __init__(self, acc: float = 0.0, turning_speed: float = 0.0):
        self.acc = acc
        self.turning_speed = turning_speed

def outputs(env):
    return CarCommand(acc = 0.1, turning_speed = 0.1)
