# This is our test python code!
import math

class CarCommand:
    # Constructs the `CarCommand` object, optionally setting the
    # acceleration and turning speed
    def __init__(self, acc: float = 0.0, turning_speed: float = 0.0):
        self.acc = acc
        self.turning_speed = turning_speed

s = """
+-------------------------------------------+
|xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|
|xxxxxxx                            xxxxxxxx|
|xxxxxx  h                g          xxxxxxx|
|xxxx         xxxxxxxxxxx              xxxxx|
|xxx      xxxxxxxxxxxxxxxxxxx           xxxx|
|xxx     xxxxxxxxxxxxxxxxxxxxxx      f   xxx|
|xxx**s**xxxxxxxxxxxxxxxxxxxxxxxx         xx|
|xxx  i   xxxxx         xxxxxxxxx         xx|
|xxx      xxx      b      xxxxxxx         xx|
|xxxx   a              d                  xx|
|xxxxxx           xxx                e   xxx|
|xxxxxxxxx      xxxxxxxxxxxx             xxx|
|xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|
+-------------------------------------------+
"""

"""
XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
XXXXXXX╱╨╨╨╨╨╨╨╨╨╨╨╨╨╨╨╨╨╨╨╨╨╨╨╨╨╨╲XXXXXXXX
XXXXXX╱      ╥╥╥╥╥╥╥╥╥╥╥           ╲XXXXXXX
XXXX╱╨   ╥╥╥╱XXXXXXXXXXX╲╥╥╥        ╨╲XXXXX
XXX╱    ╱XXXXXXXXXXXXXXXXXXX╲╥        ╲XXXX
XXX╡   ╞XXXXXXXXXXXXXXXXXXXXXX╲╥       ╲XXX
XXX╡   ╞XXXXXXXXXXXXXXXXXXXXXXXX╡       ╲XX
XXX╡    ╲XXXXX╱╨╨╨╨╨╨╨╲XXXXXXXXX╡       ╞XX
XXX╲    ╞XXX╱╨         ╨╲XXXXXXX╡       ╞XX
XXXX╲╥   ╨╨╨     ╥╥╥     ╨╨╨╨╨╨╨        ╱XX
XXXXXX╲╥╥      ╥╱XXX╲╥╥╥╥╥╥            ╞XXX
XXXXXXXXX╲╥╥╥╥╱XXXXXXXXXXXX╲╥╥╥╥╥╥╥╥╥╥╥╱XXX
XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

"""

def outputs(env):
    x = float(env.car_currently.pos.x)
    y = float(env.car_currently.pos.y)
    cur_angle = env.car_currently.angle

    with open("mem.txt", "r") as f:
        data = f.readlines()

    for i in range(len(data)):
        data[i] = data[i][0:-1]
        data[i] = data[i].split(',')
        data[i] = (int(data[i][0]), int(data[i][1]))

    (remove, angle) = go_to(x, y, data[0])

    if (remove == True):
        data = data[1:]
        with open("mem.txt", "w") as f:
            for element in data:
                f.write(str(element[0]) + "," + str(element[1]) + "\n")

    turn = 0

    if cur_angle > angle :
        turn = 0.05
    else:
        turn = -0.05

    accc = 0
    if (env.car_currently.speed < 0.01):
        accc = 0.1

    #print(angle)
    return CarCommand(acc = accc, turning_speed = turn)

def go_to(x, y, p):
    (px,py) = p

    deltax = px - x
    deltay = py - y


    return ((deltax**2 + deltay**2 < 1), math.atan2(py - y, px - x) % (2* math.pi))
