import unittest

def main():
    with open('input/01.txt') as f:
        data = f.readlines()
    
    masses = map(lambda x: int(x), data)
    fuel_amounts = map(mass_to_fuel, masses)
    total_fuel = sum(fuel_amounts)
    print('Checking Part A')
    expected = 3432671
    if total_fuel != expected:
        print('Expected {}, Found {}'.format(expected, total_fuel))
    else:
        print('Part A Passed')

def mass_to_fuel(x):
    return int(x / 3) - 2

class TestMassToFuel(unittest.TestCase):
    def test_mass_to_fuel(self):
        self.assertEqual(mass_to_fuel(12), 2)
        self.assertEqual(mass_to_fuel(14), 2)
        self.assertEqual(mass_to_fuel(1969), 654)
        self.assertEqual(mass_to_fuel(100756), 33583)

if __name__ == '__main__':
    main()
    unittest.main()
