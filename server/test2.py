class CarCommand:
    def __init__(self, acc: float = 0.0, steering: float = 0.0):
        self.acc = acc
        self.steering = steering

def outputs(car):
    # print(car.dist_to_wall)

    best_i = -1
    max_d = -1
    for i, d in enumerate(car.dist_to_wall):
        if d > max_d:
            best_i = i
            max_d = d

    dist_len = len(car.dist_to_wall)
    target_angle = 2 * (best_i / (dist_len - 1)) - 1

    return CarCommand(acc = 0.3, steering = target_angle)
