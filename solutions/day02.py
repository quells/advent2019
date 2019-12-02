from helpers import check_result
import unittest

class BreakLoop(Exception):
    pass

def main():
    with open('input/02.txt') as f:
        data = f.readline()
    
    code = list(map(int, data.split(',')))
    # 1202 override
    part_a = init_phrase(code[:], 12, 2)
    result = handle_intcode(part_a)
    check_result('02A', 4570637, result[0])

    search_for = 19690720
    found = None
    try:
        for noun in range(100):
            for verb in range(100):
                case = init_phrase(code[:], noun, verb)
                result = handle_intcode(case)
                if result[0] == search_for:
                    found = 100*noun + verb
                    raise BreakLoop
    except BreakLoop:
        pass
    if not found:
        print('02B failed: pair not found')
        return
    check_result('02B', 5485, found)

def handle_intcode(codes):
    idx = 0
    while True:
        opcode = codes[idx]
        if opcode == 1 or opcode == 2:
            a_ptr = codes[idx+1]
            b_ptr = codes[idx+2]
            r_ptr = codes[idx+3]
            a = codes[a_ptr]
            b = codes[b_ptr]
            
            if opcode == 1:
                f = lambda x, y: x + y
            elif opcode == 2:
                f = lambda x, y: x * y
            
            codes[r_ptr] = f(a, b)
            
            idx += 4
        else:
            break
    return codes

def init_phrase(code, noun, verb):
    code[1] = noun
    code[2] = verb
    return code

class TestIntcodeInterpreter(unittest.TestCase):
    def test_handle_intcode(self):
        cases = [
            ([1,9,10,3,2,3,11,0,99,30,40,50], 3500, 0),
            ([1,0,0,0,99], 2, 0),
            ([2,3,0,3,99], 6, 3),
            ([2,4,4,5,99,0], 9801, 5),
            ([1,1,1,4,99,5,6,0,99], 30, 0),
        ]
        for (code, expected, r_ptr) in cases:
            result = handle_intcode(code)
            self.assertEqual(result[r_ptr], expected)

if __name__ == '__main__':
    main()
    unittest.main()