# This is our test python code!

class CarCommand:
    # Constructs the `CarCommand` object, optionally setting the
    # acceleration and turning speed
    def __init__(self, acc: float = 0.0, turning_speed: float = 0.0):
        self.acc = acc
        self.turning_speed = turning_speed

def outputs(env):
    best_i = -1
    max_d = -1
    for i, d in enumerate(env.dist_to_wall):
        if d > max_d:
            best_i = i
            max_d = d

    rad_per = 2*3.1415 / len(env.dist_to_wall)

    desired_angle = best_i*rad_per

    current_angle = env.car_currently.angle

    if ((desired_angle-current_angle) > 3.1415/4):
        turn = 0
    elif (desired_angle < current_angle):
         turn = -0.2
    else:
         turn = 0.2

    dist_len = len(env.dist_to_wall)
    return CarCommand(acc = 1, turning_speed = turn)
