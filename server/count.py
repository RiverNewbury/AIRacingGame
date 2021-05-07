# A simple example program, to showcase how we're able to store data
# between individual calls to `outputs`

class CarCommand:
    # Constructs the `CarCommand` object, optionally setting the
    # acceleration and steering
    def __init__(self, acc: float = 0.0, steering: float = 0.0):
        self.acc = acc
        self.steering = steering

i = 0

def outputs(car):
    global i
    i += 1
    print(i)
    
    return CarCommand(acc = 0.5, steering = 0.0)

