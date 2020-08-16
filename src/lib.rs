mod fuel;
mod intcode;
mod password;
mod wires;

use std::io::prelude::Read;

fn load(input_file: &str) -> String {
    let filename = std::path::Path::new("./input").join(input_file);
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::str::FromStr;

    #[test]
    fn day01a() {
        let masses: Vec<usize> = load("01.txt")
            .split("\n")
            .map(|s| usize::from_str(s).unwrap())
            .collect();
        let fuel_amounts = masses.into_iter().map(fuel::mass_to_fuel);
        let total_fuel = fuel_amounts.fold(0, |acc, f| acc + f);
        assert_eq!(total_fuel, 3432671);
    }

    #[test]
    fn day01b() {
        let masses: Vec<usize> = load("01.txt")
            .split("\n")
            .map(|s| usize::from_str(s).unwrap())
            .collect();
        let fuel_amounts = masses.into_iter().map(fuel::incremental_mass_to_fuel);
        let total_fuel = fuel_amounts.fold(0, |acc, f| acc + f);
        assert_eq!(total_fuel, 5146132);
    }

    #[test]
    fn day02a() {
        let mut program: Vec<isize> = load("02.txt")
            .split(",")
            .map(|s| isize::from_str(s).unwrap())
            .collect();
        program[1] = 12;
        program[2] = 2;
        let mut vm = intcode::VM::new(&program);
        vm.run();
        assert_eq!(vm.mem[0], 4570637);
    }

    #[test]
    fn day02b() {
        let mut program: Vec<isize> = load("02.txt")
            .split(",")
            .map(|s| isize::from_str(s).unwrap())
            .collect();
    
        'outer: for noun in 0..100 {
            program[1] = noun;
            for verb in 0..100 {
                program[2] = verb;

                let mut vm = intcode::VM::new(&program);
                vm.run();
                if vm.mem[0] == 19690720 {
                    let result = 100*noun + verb;
                    assert_eq!(result, 5485);
                    break 'outer;
                }
            }
        }
    }

    #[test]
    fn day03a() {
        let input = load("03.txt");
        let mut paths = input
            .split("\n")
            .map(|line| line.split(",")
                .map(|s| s.to_owned())
                .collect::<Vec<String>>())
            .map(|segments| wires::path(&segments));
        let a = paths.next().unwrap();
        let b = paths.next().unwrap();
        let crossings = wires::crossings(&a, &b);
        let min_distance = crossings.into_iter()
            .map(|c| wires::manhattan_distance(c.x, c.y))
            .fold(isize::max_value(), |m, x| isize::min(m, x));
        assert_eq!(min_distance, 1983);
    }

    #[test]
    fn day03b() {
        let input = load("03.txt");
        let mut paths = input
            .split("\n")
            .map(|line| line.split(",")
                .map(|s| s.to_owned())
                .collect::<Vec<String>>())
            .map(|segments| wires::path(&segments));
        let a = paths.next().unwrap();
        let b = paths.next().unwrap();
        let crossings = wires::crossings(&a, &b);
        let min_time = crossings.into_iter()
            .map(|c| c.total_time)
            .fold(isize::max_value(), |m, x| isize::min(m, x));
        assert_eq!(min_time, 107754);
    }

    #[test]
    fn day04() {
        let mut a = 0;
        let mut b = 0;
        for i in 356261..846303 {
            let runs = password::runs(&i.to_string());
            if password::is_monotonic(&runs) {
                if password::contains_consecutive(&runs) {
                    a += 1;
                    if password::contains_double(&runs) {
                        b += 1;
                    }
                }
            }
        }
        assert_eq!(a, 544);
        assert_eq!(b, 334);
    }
}
