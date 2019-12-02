from helpers import check_result
import unittest

def main():
    with open('input/01.txt') as f:
        data = f.readlines()
    
    masses = map(lambda x: int(x), data)
    fuel_amounts = map(mass_to_fuel, masses)
    total_fuel = sum(fuel_amounts)
    check_result('01A', 3432671, total_fuel)
    
    masses = map(lambda x: int(x), data)
    fuel_amounts = map(incremental_mass_to_fuel, masses)
    total_fuel = sum(fuel_amounts)
    check_result('01B', 5146132, total_fuel)

def mass_to_fuel(x):
    return max(0, int(x / 3) - 2)

def incremental_mass_to_fuel(x):
    f, fi = 0, mass_to_fuel(x)
    while fi:
        f += fi
        fi = mass_to_fuel(fi)
    return f

class TestMassToFuel(unittest.TestCase):
    def test_mass_to_fuel(self):
        self.assertEqual(mass_to_fuel(12), 2)
        self.assertEqual(mass_to_fuel(14), 2)
        self.assertEqual(mass_to_fuel(1969), 654)
        self.assertEqual(mass_to_fuel(100756), 33583)
    
    def test_incremental_mass_to_fuel(self):
        self.assertEqual(incremental_mass_to_fuel(14), 2)
        self.assertEqual(incremental_mass_to_fuel(100756), 50346)

if __name__ == '__main__':
    main()
    unittest.main()
