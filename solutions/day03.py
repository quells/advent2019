from helpers import check_result
import unittest

def main():
    # should just convert each cmd to a line segment and then do pairwise hit-tests between wires
    # that's probably better than a bitmap approach
    pass

# This isn't that useful - better to just keep direction keys and unsigned distance
def parse_segment(s):
    if len(s) < 2:
        raise Exception("invalid segment `{}`".format(s))
    direction = s[0]
    distance = int(s[1:])
    if direction == "U":
        return (0, distance)
    elif direction == "R":
        return (distance, 0)
    elif direction == "D":
        return (0, -distance)
    elif direction == "L":
        return (-distance, 0)
    raise Exception("invalid direction `{}` in segment {}".format(direction, s))

def parse_segments(paths):
    [a, b] = paths.split("\n")
    a_segments = list(map(parse_segment, a.split(",")))
    b_segments = list(map(parse_segment, b.split(",")))
    return (a_segments, b_segments)

class TestCrossedWires(unittest.TestCase):
    def test_closest_crossing_a(self):
        paths = """R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"""
        expected = 159

    def test_closest_crossing_b(self):
        path = """R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"""
        expected = 135

if __name__ == '__main__':
    main()
    unittest.main()
