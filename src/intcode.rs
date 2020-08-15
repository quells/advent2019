
#[derive(Copy, Clone, Debug)]
pub enum Op {
    // 1 : a, b, dst
    // *dst := *a + *b
    Add,

    // 2 : a, b, dst
    // *dst := *a * *b
    Mul,

    // 99
    Halt,
}

impl Op {
    pub fn from(x: usize) -> Option<Self> {
        let op = match x {
            1 => Self::Add,
            2 => Self::Mul,
            99 => Self::Halt,
            _ => return None,
        };
        Some(op)
    }
}

#[derive(Clone, Debug)]
pub struct VM {
    pub mem: Vec<usize>,
    pub pc: usize,
    pub halt: bool,
}

impl VM {
    pub fn new(program: &[usize]) -> Self {
        Self {
            mem: program.to_vec(),
            pc: 0,
            halt: false,
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.halt {
                return;
            }

            self.step();
        }
    }

    pub fn step(&mut self) {
        if self.halt {
            return;
        }

        let op_code = self.mem[self.pc];
        let op = Op::from(op_code);
        self.pc += 1;

        match op {
            Some(Op::Add) => self.binop(op.unwrap()),
            Some(Op::Mul) => self.binop(op.unwrap()),
            Some(Op::Halt) => {
                self.halt = true;
                return;
            },
            None => {
                eprintln!("invalid op code {}", op_code);
            },
        }
    }

    fn binop(&mut self, op: Op) {
        let a_ptr = self.mem[self.pc];
        let b_ptr = self.mem[self.pc + 1];
        let dst_ptr = self.mem[self.pc + 2];
        self.pc += 3;

        let a = self.mem[a_ptr];
        let b = self.mem[b_ptr];
        
        self.mem[dst_ptr] = match op {
            Op::Add => a + b,
            Op::Mul => a * b,
            _ => return,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_mul_halt() {
        let prog = vec![
            1, 9, 10, 3,
            2, 3, 11, 0,
            99,
            30, 40, 50,
        ];
        let mut vm = VM::new(&prog);
        
        vm.step();
        assert_eq!(vm.mem[3], 70);

        vm.step();
        assert_eq!(vm.mem[0], 3500);

        vm.step();
        assert_eq!(vm.halt, true);
    }
}
