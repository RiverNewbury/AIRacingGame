# This is our test python code!
class CarCommand:
    # Constructs the `CarCommand` object, optionally setting the
    # acceleration and steering
    def __init__(self, acc: float = 0.0, steering: float = 0.0):
        self.acc = acc
        self.steering = steering

def outputs(car):
    left = car.dist_to_wall[0]
    right = car.dist_to_wall[-1]

    turn = 0

    if  left > 2* right :
        turn = -0.1
    elif right > 2* left:
        turn = 0.1

    accc = 0
    if (car.speed < 0.3):
        accc = 0.1

    #print(angle)
    return CarCommand(acc = accc, steering = turn)
