from helpers import check_result
import unittest

def main():
    with open('input/03.txt') as f:
        paths = f.read()
    
    (a, b) = parse_segments(paths)
    crossings = find_crossings(a, b)
    distances = map(manhattan_distance, crossings)
    min_distance = min(distances)
    check_result('03A', 1983, min_distance)

    times = map(total_time, crossings)
    min_time = min(times)
    check_result('03B', 107754, min_time)

class LineSegment(object):
    def __init__(self, x1, y1, d, l):
        self.x1, self.y1 = x1, y1
        self.l = l
        if d == 'U':
            self.x2, self.y2 = x1, y1+l
            self.v = True
        elif d == 'D':
            self.x2, self.y2 = x1, y1-l
            self.v = True
        elif d == 'R':
            self.x2, self.y2 = x1+l, y1
            self.v = False
        elif d == 'L':
            self.x2, self.y2 = x1-l, y1
            self.v = False
        else:
            raise Exception('invalid direction')
    
    def __repr__(self):
        return '{}[({}, {}) ({}, {})]'.format('v' if self.v else 'h', self.x1, self.y1, self.x2, self.y2)
    
    def intersect(self, other):
        if self.v == other.v:
            return None
        if self.v:
            miny, maxy = min(self.y1, self.y2), max(self.y1, self.y2)
            minx, maxx = min(other.x1, other.x2), max(other.x1, other.x2)
            if miny <= other.y1 and other.y1 <= maxy and minx <= self.x1 and self.x1 <= maxx:
                return (self.x1, other.y1, abs(self.y1-other.y1))
        else:
            minx, maxx = min(self.x1, self.x2), max(self.x1, self.x2)
            miny, maxy = min(other.y1, other.y2), max(other.y1, other.y2)
            if minx <= other.x1 and other.x1 <= maxx and miny <= self.y1 and self.y1 <= maxy:
                return (other.x1, self.y1, abs(self.x1-other.x1))
        return None

def build_path(segments):
    x, y = 0, 0
    path = []
    for segment in segments:
        d = segment[0]
        l = int(segment[1:])
        line = LineSegment(x, y, d, l)
        path.append(line)
        x, y = line.x2, line.y2
    return path

def find_crossings(a, b):
    crossings = []
    at = 0
    for ai in a:
        bt = 0
        for bi in b:
            intersection = ai.intersect(bi)
            if intersection:
                (x, y, aint) = intersection
                bint = manhattan_distance((abs(x-bi.x1), abs(y-bi.y1)))
                crossings.append((x, y, at+aint, bt+bint))
            bt += bi.l
        at += ai.l
    return set(filter(lambda p: p[0] != 0 and p[1] != 0, crossings))

def parse_segments(paths):
    [a, b] = paths.split('\n')
    a_path = build_path(a.split(','))
    b_path = build_path(b.split(','))
    return (a_path, b_path)

def manhattan_distance(d):
    return abs(d[0]) + abs(d[1])

def total_time(d):
    return d[2] + d[3]

class TestCrossedWires(unittest.TestCase):
    def a_paths(self):
        return '''R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83'''
    
    def b_paths(self):
        return '''R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7'''

    def test_closest_distance_a(self):
        expected = 159
        (a, b) = parse_segments(self.a_paths())
        crossings = find_crossings(a, b)
        distances = map(manhattan_distance, crossings)
        min_distance = min(distances)
        self.assertEqual(min_distance, expected)

    def test_closest_distance_b(self):
        expected = 135
        (a, b) = parse_segments(self.b_paths())
        crossings = find_crossings(a, b)
        distances = map(manhattan_distance, crossings)
        min_distance = min(distances)
        self.assertEqual(min_distance, expected)

    def test_least_time_a(self):
        expected = 610
        (a, b) = parse_segments(self.a_paths())
        crossings = find_crossings(a, b)
        times = map(total_time, crossings)
        min_time = min(times)
        self.assertEqual(min_time, expected)
    
    def test_least_time_b(self):
        expected = 410
        (a, b) = parse_segments(self.b_paths())
        crossings = find_crossings(a, b)
        times = map(total_time, crossings)
        min_time = min(times)
        self.assertEqual(min_time, expected)

if __name__ == '__main__':
    main()
    unittest.main()
