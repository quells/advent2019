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
            Some(Op::Halt) => {
                self.halt = true;
                return;
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
            _ => return,
        };
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

    #[test]
    fn io() {
        let prog = vec![
            3, 0,
            4, 0,
            99,
        ];
        let (mut vm, input, output) = VM::with_io(&prog);
        let i = thread::spawn(move || input.send(123).unwrap());
        let o = thread::spawn(move || output.recv().unwrap());
        let vm = thread::spawn(move || vm.run());

        i.join().expect("input thread panicked");
        let result = o.join().expect("output thread panicked");
        vm.join().expect("vm thread panicked");
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
}
