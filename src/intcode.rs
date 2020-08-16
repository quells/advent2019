use std::convert::TryFrom;
use std::sync::mpsc::{SyncSender, Receiver};
use std::sync::mpsc;

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Ptr,
    Imm,
}

#[derive(Copy, Clone, Debug)]
pub enum Op {
    // 01 : a, b, dst
    // *dst := *a + *b
    Add(Mode, Mode),

    // 02 : a, b, dst
    // *dst := *a * *b
    Mul(Mode, Mode),

    // 03 : dst
    // *dst := input
    Read,

    // 04 : src
    // output := *src
    Write(Mode),

    // 05 : a dst
    // jmp to *dst if *a
    // 06 : a dst
    // jmp to *dst if !*a
    Jump(bool, Mode, Mode),

    // 07 : a b dst
    // if (*a < *b) then *dst = 1 else *dst = 0
    Less(Mode, Mode),

    // 08 : a b dst
    // if (*a == *b) then *dst = 1 else *dst = 0
    Equal(Mode, Mode),

    // 99
    Halt,
}

impl Op {
    pub fn from(x: isize) -> Option<Self> {
        let op = match x {
               1 => Self::Add(Mode::Ptr, Mode::Ptr),
             101 => Self::Add(Mode::Imm, Mode::Ptr),
            1001 => Self::Add(Mode::Ptr, Mode::Imm),
            1101 => Self::Add(Mode::Imm, Mode::Imm),

               2 => Self::Mul(Mode::Ptr, Mode::Ptr),
             102 => Self::Mul(Mode::Imm, Mode::Ptr),
            1002 => Self::Mul(Mode::Ptr, Mode::Imm),
            1102 => Self::Mul(Mode::Imm, Mode::Imm),

            3 => Self::Read,

              4 => Self::Write(Mode::Ptr),
            104 => Self::Write(Mode::Imm),

               5 => Self::Jump(true, Mode::Ptr, Mode::Ptr),
             105 => Self::Jump(true, Mode::Imm, Mode::Ptr),
            1005 => Self::Jump(true, Mode::Ptr, Mode::Imm),
            1105 => Self::Jump(true, Mode::Imm, Mode::Imm),

               6 => Self::Jump(false, Mode::Ptr, Mode::Ptr),
             106 => Self::Jump(false, Mode::Imm, Mode::Ptr),
            1006 => Self::Jump(false, Mode::Ptr, Mode::Imm),
            1106 => Self::Jump(false, Mode::Imm, Mode::Imm),

               7 => Self::Less(Mode::Ptr, Mode::Ptr),
             107 => Self::Less(Mode::Imm, Mode::Ptr),
            1007 => Self::Less(Mode::Ptr, Mode::Imm),
            1107 => Self::Less(Mode::Imm, Mode::Imm),

               8 => Self::Equal(Mode::Ptr, Mode::Ptr),
             108 => Self::Equal(Mode::Imm, Mode::Ptr),
            1008 => Self::Equal(Mode::Ptr, Mode::Imm),
            1108 => Self::Equal(Mode::Imm, Mode::Imm),

            99 => Self::Halt,
            _ => return None,
        };
        Some(op)
    }
}

#[derive(Debug)]
pub struct VM {
    pub mem: Vec<isize>,
    pub pc: usize,
    pub halt: bool,
    reader: Receiver<isize>,
    writer: SyncSender<isize>,
}

impl VM {
    pub fn new(program: &[isize]) -> Self {
        Self::with_io(program).0
    }

    pub fn with_io(program: &[isize]) -> (Self, SyncSender<isize>, Receiver<isize>) {
        let (input_tx, input_rx): (SyncSender<isize>, Receiver<isize>) = mpsc::sync_channel(0);
        let (output_tx, output_rx): (SyncSender<isize>, Receiver<isize>) = mpsc::sync_channel(0);
        let s = Self {
            mem: program.to_vec(),
            pc: 0,
            halt: false,
            reader: input_rx,
            writer: output_tx,
        };
        (s, input_tx, output_rx)
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
            Some(Op::Add(_, _)) => self.binop(op.unwrap()),
            Some(Op::Mul(_, _)) => self.binop(op.unwrap()),
            Some(Op::Less(_, _)) => self.cmpop(op.unwrap()),
            Some(Op::Equal(_, _)) => self.cmpop(op.unwrap()),
            Some(Op::Read) => {
                let ptr = self.mem[self.pc];
                self.pc += 1;
                self.put(ptr, self.reader.recv().unwrap());
            },
            Some(Op::Write(mode)) => {
                let ptr = self.mem[self.pc];
                self.pc += 1;
                self.writer.send(self.deref(ptr, mode)).unwrap();
            },
            Some(Op::Jump(m, a_mode, dst_mode)) => {
                let ptr = self.mem[self.pc];
                let val = self.deref(ptr, a_mode);
                let dst_ptr = self.mem[self.pc + 1];
                let dst = self.deref(dst_ptr, dst_mode);
                if (val != 0) == m {
                    self.pc = usize::try_from(dst).unwrap();
                } else {
                    self.pc += 2;
                }
            },
            Some(Op::Halt) => {
                self.halt = true;
            },
            None => {
                eprintln!("invalid op code {}", op_code);
            },
        }
    }

    #[inline(always)]
    fn put(&mut self, ptr: isize, value: isize) {
        self.mem[usize::try_from(ptr).unwrap()] = value;
    }

    #[inline(always)]
    fn deref(&self, ptr: isize, mode: Mode) -> isize {
        match mode {
            Mode::Ptr => self.mem[usize::try_from(ptr).unwrap()],
            Mode::Imm => ptr,
        }
    }

    fn binop(&mut self, op: Op) {
        let a_ptr = self.mem[self.pc];
        let b_ptr = self.mem[self.pc + 1];
        let dst_ptr = self.mem[self.pc + 2];
        self.pc += 3;
        
        let result = match op {
            Op::Add(a_mode, b_mode) => {
                let a = self.deref(a_ptr, a_mode);
                let b = self.deref(b_ptr, b_mode);
                a + b
            },
            Op::Mul(a_mode, b_mode) => {
                let a = self.deref(a_ptr, a_mode);
                let b = self.deref(b_ptr, b_mode);
                a * b
            },
            _ => panic!("unhandled binop"),
        };
        self.put(dst_ptr, result);
    }

    fn cmpop(&mut self, op: Op) {
        let a_ptr = self.mem[self.pc];
        let b_ptr = self.mem[self.pc + 1];
        let dst_ptr = self.mem[self.pc + 2];
        self.pc += 3;
        
        let cmp = match op {
            Op::Less(a_mode, b_mode) => {
                let a = self.deref(a_ptr, a_mode);
                let b = self.deref(b_ptr, b_mode);
                a < b
            },
            Op::Equal(a_mode, b_mode) => {
                let a = self.deref(a_ptr, a_mode);
                let b = self.deref(b_ptr, b_mode);
                a == b
            },
            _ => panic!("unhandled cmp op"),
        };
        let result = if cmp { 1 } else { 0 };
        self.put(dst_ptr, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

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
        assert!(vm.halt);
    }

    fn test_io<'a> (prog: &[isize], input_vals: Vec<isize>) -> Vec<isize> {
        let (mut vm, input, output) = VM::with_io(&prog);
        let i = thread::spawn(move || {
            for v in input_vals {
                input.send(v).unwrap();
            }
        });
        let o = thread::spawn(move || {
            let mut responses = Vec::new();
            loop {
                match output.recv() {
                    Ok(r) => responses.push(r),
                    _ => break,
                }
            }
            responses
        });
        let vm = thread::spawn(move || vm.run());
        
        i.join().expect("input thread panicked");
        vm.join().expect("vm thread panicked");
        o.join().expect("output thread panicked")
    }

    #[test]
    fn io() {
        let prog = vec![
            3, 0,
            4, 0,
            99,
        ];
        let result = test_io(&prog, vec![123])[0];
        assert_eq!(result, 123);
    }

    #[test]
    fn modes() {
        let prog = vec![
            1002, 4, 3, 4,
            33,
        ];
        let mut vm = VM::new(&prog);

        vm.step();
        assert_eq!(vm.mem[4], 99);

        vm.step();
        assert!(vm.halt);
    }

    #[test]
    fn jump_ptr() {
        let prog = vec![
            3, 12,
            6, 12, 15,
            1, 13, 14, 13,
            4, 13,
            99,
            -1, 0, 1, 9,
        ];

        let result = test_io(&prog, vec![123])[0];
        assert_eq!(result, 1);

        let result = test_io(&prog, vec![0])[0];
        assert_eq!(result, 0);
    }

    #[test]
    fn jump_imm() {
        let prog = vec![
            3, 3,
            1105, -1, 9,
            1101, 0, 0, 12,
            4, 12,
            99,
            1,
        ];

        let result = test_io(&prog, vec![123])[0];
        assert_eq!(result, 1);

        let result = test_io(&prog, vec![0])[0];
        assert_eq!(result, 0);
    }

    #[test]
    fn cmp_ptr() {
        let mut prog = vec![
            3, 9,
            8, 9, 10, 9,
            4, 9,
            99,
            -1, 8,
        ];

        // input == 8
        let result = test_io(&prog, vec![8])[0];
        assert_eq!(result, 1);
        let result = test_io(&prog, vec![7])[0];
        assert_eq!(result, 0);

        // input < 8
        prog[2] = 7;
        let result = test_io(&prog, vec![8])[0];
        assert_eq!(result, 0);
        let result = test_io(&prog, vec![7])[0];
        assert_eq!(result, 1);
    }

    #[test]
    fn cmp_imm() {
        let mut prog = vec![
            3, 3,
            1108, -1, 8, 3,
            4, 3,
            99,
        ];

        // input == 8
        let result = test_io(&prog, vec![8])[0];
        assert_eq!(result, 1);
        let result = test_io(&prog, vec![7])[0];
        assert_eq!(result, 0);

        // input < 8
        prog[2] = 1107;
        let result = test_io(&prog, vec![8])[0];
        assert_eq!(result, 0);
        let result = test_io(&prog, vec![7])[0];
        assert_eq!(result, 1);
    }

    #[test]
    fn cmp_jmp() {
        let prog = vec![
            3, 21,             // 00 READ  input     -> [21]
            1008, 21, 8, 20,   // 02 EQUAL [21] == 8 -> [20]
            1005, 20, 22,      // 06 JUMP  [20] != 0 ->  22
            107, 8, 21, 20,    // 09 LESS  8 < [21]  -> [20]
            1006, 20, 31,      // 13 JUMP  [20] == 0 ->  31
            1106, 0, 36,       // 16 JUMP     0 == 0 ->  36
            98, 0, 0,          // 19, 20, 21
            1002, 21, 125, 20, // 22 MUL   [21]*125  -> [20]
            4, 20,             // 26 WRITE [20]      -> output
            1105, 1, 46,       // 28 JUMP     1 == 0 ->  46
            104, 999,          // 31 WRITE 999       -> output
            1105, 1, 46,       // 33 JUMP     1 == 0 ->  46
            1101, 1000, 1, 20, // 36 ADD   1000+1    -> [20]
            4, 20,             // 40 WRITE [20]      -> output
            1105, 1, 46,       // 42 JUMP     1 == 0 ->  46
            98,                // 45
            99,                // 46 HALT
        ];

        let result = test_io(&prog, vec![7])[0];
        assert_eq!(result, 999);
        let result = test_io(&prog, vec![8])[0];
        assert_eq!(result, 1000);
        let result = test_io(&prog, vec![9])[0];
        assert_eq!(result, 1001);
    }
}
