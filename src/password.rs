use std::str::FromStr;

pub fn runs(s: &str) -> Vec<(usize, usize)> {
    let mut digits: Vec<usize> = s.chars()
        .map(|c| usize::from_str(&c.to_string()).unwrap())
        .collect();
    let mut runs = Vec::new();
    let (prev, digits) = digits.split_first_mut().unwrap();
    let mut run = 1;
    for d in digits {
        if *d == *prev {
            run += 1;
        } else {
            runs.push((*prev, run));
            *prev = *d;
            run = 1;
        }
    }
    runs.push((*prev, run));

    runs
}

pub fn is_monotonic(runs: &[(usize, usize)]) -> bool {
    let mut m = 0;
    for (v, _) in runs {
        if *v < m {
            return false;
        }
        m = *v;
    }
    true
}

pub fn contains_consecutive(runs: &[(usize, usize)]) -> bool {
    for (_, count) in runs {
        if *count > 1 {
            return true;
        }
    }
    false
}

pub fn contains_double(runs: &[(usize, usize)]) -> bool {
    for (_, count) in runs {
        if *count == 2 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_runs() {
        assert_eq!(runs(&"111111"), vec![(1, 6)]);
        assert_eq!(runs(&"2230"), vec![(2, 2), (3, 1), (0, 1)]);
    }

    #[test]
    fn monotonic_run() {
        assert!(is_monotonic(&runs(&"111111")));
        assert!(is_monotonic(&runs(&"123789")));
        assert!(!is_monotonic(&runs(&"223450")));
    }

    #[test]
    fn run_contains_adjacent() {
        assert!(contains_consecutive(&runs(&"111111")));
        assert!(contains_consecutive(&runs(&"223450")));
        assert!(!contains_consecutive(&runs(&"123789")));
    }

    #[test]
    fn run_contains_double() {
        assert!(!contains_double(&runs(&"111111")));
        assert!(contains_double(&runs(&"223450")));
        assert!(!contains_double(&runs(&"123789")));
    }
}
