# A simple example program, to showcase how we're able to store data
# between individual calls to `outputs`

class CarCommand:
    # Constructs the `CarCommand` object, optionally setting the
    # acceleration and turning speed
    def __init__(self, acc: float = 0.0, turning_speed: float = 0.0):
        self.acc = acc
        self.turning_speed = turning_speed

i = 0

def outputs(env):
    global i
    i += 1
    print(i)
    
    return CarCommand(acc = 0.5, turning_speed = 0.0)

