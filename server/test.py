# This is our test python code!
import math

class CarCommand:
    # Constructs the `CarCommand` object, optionally setting the
    # acceleration and steering
    def __init__(self, acc: float = 0.0, steering: float = 0.0):
        self.acc = acc
        self.steering = steering

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
|xxx      xxx      c      xxxxxxx         xx|
|xxxx   a                                 xx|
|xxxxxx      b    xxx    d           e   xxx|
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

data = [(17,7), (25,5), (37,9), (49,6), (72,6), (73,15), (51,21), (17,21), (7,9)]

def outputs(car):
    global data
    x = float(car.pos.x)
    y = float(car.pos.y)
    cur_angle = car.angle % (2 * math.pi)

    (remove, angle) = go_to(x, y, data[0])

    if (remove == True):
        data = data[1:]
        #print(x,y)
        #print(data[0])

    turn = 0

    if  (angle - cur_angle)%(2*math.pi) < math.pi :
        turn = -0.05
    else:
        turn = 0.05

    accc = 0
    if (car.speed < 0.5):
        accc = 0.3

    #print(angle)
    return CarCommand(acc = accc, steering = turn)

def go_to(x, y, p):
    (px,py) = p

    deltax = px - x
    deltay = py - y


    return ((deltax**2 + deltay**2 < 1), math.atan2(py - y, px - x) % (2* math.pi))
