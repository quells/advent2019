
pub fn mass_to_fuel(m: usize) -> usize {
    if m <= 6 {
        0
    } else {
        m/3 - 2
    }
}

pub fn incremental_mass_to_fuel(m: usize) -> usize {
    let mut f = 0;
    let mut fi = mass_to_fuel(m);
    while fi > 0 {
        f += fi;
        fi = mass_to_fuel(fi);
    }
    f
}


