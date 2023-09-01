//! Hanoi Tower solution builder.


fn main() {
    let n: u8 = read_number_from_stdin();
    dbg!(HanoiTower::new(n).find_solution());
}


#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
enum HanoiTowerSolutionStep {
    A_to_B,
    B_to_A,
    A_to_C,
    C_to_A,
    B_to_C,
    C_to_B,
}

#[derive(Debug, PartialEq, Eq)]
struct HanoiTowerSolution {
    steps: Vec<HanoiTowerSolutionStep>
}
impl HanoiTowerSolution {
    pub const fn new() -> Self {
        Self { steps: vec![] }
    }
}

struct Rod {
    disks: Vec<u8>
}

struct HanoiTower {
    rod_a: Rod,
    rod_b: Rod,
    rod_c: Rod,
}
impl HanoiTower {
    pub fn new(n: u8) -> Self {
        Self {
            rod_a: Rod { disks: (1..=n).collect() },
            rod_b: Rod { disks: Vec::with_capacity(n as usize) },
            rod_c: Rod { disks: Vec::with_capacity(n as usize) },
        }
    }

    fn find_solution(&mut self) -> HanoiTowerSolution {
        let n: u8 = *[
            self.rod_a.disks.iter().max(),
            self.rod_b.disks.iter().max(),
            self.rod_c.disks.iter().max()
        ].iter().max().unwrap().unwrap();
        let mut hanoi_tower_solution = HanoiTowerSolution::new();
        match n % 2 {
            0 => {
                for _ in 0 .. 2_u64.pow(n as u32)-1 {
                    if self.rod_a.disks.last().unwrap_or(&0) > self.rod_b.disks.last().unwrap_or(&0) {
                        self.rod_b.disks.push(self.rod_a.disks.pop().unwrap());
                        hanoi_tower_solution.steps.push(HanoiTowerSolutionStep::A_to_B);
                    } else {
                        self.rod_a.disks.push(self.rod_b.disks.pop().unwrap());
                        hanoi_tower_solution.steps.push(HanoiTowerSolutionStep::B_to_A);
                    }
                    if self.rod_a.disks.last().unwrap_or(&0) > self.rod_c.disks.last().unwrap_or(&0) {
                        self.rod_c.disks.push(self.rod_a.disks.pop().unwrap());
                        hanoi_tower_solution.steps.push(HanoiTowerSolutionStep::A_to_C);
                    } else {
                        self.rod_a.disks.push(self.rod_c.disks.pop().unwrap());
                        hanoi_tower_solution.steps.push(HanoiTowerSolutionStep::C_to_A);
                    }
                    if self.rod_c.disks.len() == n as usize {
                        return hanoi_tower_solution;
                    }
                    if self.rod_b.disks.last().unwrap_or(&0) > self.rod_c.disks.last().unwrap_or(&0) {
                        self.rod_c.disks.push(self.rod_b.disks.pop().unwrap());
                        hanoi_tower_solution.steps.push(HanoiTowerSolutionStep::B_to_C);
                    } else {
                        self.rod_b.disks.push(self.rod_c.disks.pop().unwrap());
                        hanoi_tower_solution.steps.push(HanoiTowerSolutionStep::C_to_B);
                    }
                    if self.rod_c.disks.len() == n as usize {
                        return hanoi_tower_solution;
                    }
                }
            }
            1 => {
                for _ in 0 .. 2_u64.pow(n as u32)-1 {
                    if self.rod_a.disks.last().unwrap_or(&0) > self.rod_c.disks.last().unwrap_or(&0) {
                        self.rod_c.disks.push(self.rod_a.disks.pop().unwrap());
                        hanoi_tower_solution.steps.push(HanoiTowerSolutionStep::A_to_C);
                    } else {
                        self.rod_a.disks.push(self.rod_c.disks.pop().unwrap());
                        hanoi_tower_solution.steps.push(HanoiTowerSolutionStep::C_to_A);
                    }
                    if self.rod_c.disks.len() == n as usize {
                        return hanoi_tower_solution;
                    }
                    if self.rod_a.disks.last().unwrap_or(&0) > self.rod_b.disks.last().unwrap_or(&0) {
                        self.rod_b.disks.push(self.rod_a.disks.pop().unwrap());
                        hanoi_tower_solution.steps.push(HanoiTowerSolutionStep::A_to_B);
                    } else {
                        self.rod_a.disks.push(self.rod_b.disks.pop().unwrap());
                        hanoi_tower_solution.steps.push(HanoiTowerSolutionStep::B_to_A);
                    }
                    if self.rod_b.disks.last().unwrap_or(&0) > self.rod_c.disks.last().unwrap_or(&0) {
                        self.rod_c.disks.push(self.rod_b.disks.pop().unwrap());
                        hanoi_tower_solution.steps.push(HanoiTowerSolutionStep::B_to_C);
                    } else {
                        self.rod_b.disks.push(self.rod_c.disks.pop().unwrap());
                        hanoi_tower_solution.steps.push(HanoiTowerSolutionStep::C_to_B);
                    }
                    if self.rod_c.disks.len() == n as usize {
                        return hanoi_tower_solution;
                    }
                }
            }
            _ => unreachable!()
        }
        todo!()
    }
}


fn read_number_from_stdin<T>() -> T
where T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug
{
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer = buffer.strip_suffix('\n').unwrap().to_string();
    buffer.parse().unwrap()
}


#[test]
fn find_solution_n_1() {
    assert_eq!(
        HanoiTowerSolution {
            steps: vec![
                HanoiTowerSolutionStep::A_to_C,
            ]
        },
        HanoiTower::new(1).find_solution()
    );
}

#[test]
fn find_solution_n_2() {
    assert_eq!(
        HanoiTowerSolution {
            steps: vec![
                HanoiTowerSolutionStep::A_to_B,
                HanoiTowerSolutionStep::A_to_C,
                HanoiTowerSolutionStep::B_to_C,
            ]
        },
        HanoiTower::new(2).find_solution()
    );
}

#[test]
fn find_solution_n_3() {
    assert_eq!(
        HanoiTowerSolution {
            steps: vec![
                HanoiTowerSolutionStep::A_to_C,
                HanoiTowerSolutionStep::A_to_B,
                HanoiTowerSolutionStep::C_to_B,
                HanoiTowerSolutionStep::A_to_C,
                HanoiTowerSolutionStep::B_to_A,
                HanoiTowerSolutionStep::B_to_C,
                HanoiTowerSolutionStep::A_to_C,
            ]
        },
        HanoiTower::new(3).find_solution()
    );
}

