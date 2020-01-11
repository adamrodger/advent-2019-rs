use std::collections::VecDeque;

#[derive(Debug)]
enum StepResult {
    Continue,
    InputRequired,
    Halted
}

#[derive(Debug)]
pub enum YieldReason {
    InputRequired,
    Halted
}

#[derive(Debug)]
pub struct IntCodeEmulator {
    ram: Vec<i64>,
    pointer: usize,
    base: i64,
    stdin: VecDeque<i64>,
    stdout: VecDeque<i64>
}

impl IntCodeEmulator {
    pub fn new(program: Vec<i64>) -> IntCodeEmulator {
        IntCodeEmulator {
            ram: program,
            pointer: 0,
            base: 0,
            stdin: VecDeque::new(),
            stdout: VecDeque::new(),
        }
    }

    pub fn from_input(input: &str) -> IntCodeEmulator {
        let program = IntCodeEmulator::parse_input(input);
        IntCodeEmulator::new(program)
    }

    pub fn parse_input(input: &str) -> Vec<i64> {
        input.trim().split(',').map(|l| l.parse().expect("Unable to parse input")).collect()
    }

    pub fn ram(&self) -> &Vec<i64> {
        &self.ram
    }

    pub fn stdin(&mut self) -> &mut VecDeque<i64> {
        &mut self.stdin
    }

    pub fn stdout(&mut self) -> &mut VecDeque<i64> {
        &mut self.stdout
    }

    pub fn execute(&mut self) {
        loop {
            match self.step() {
                StepResult::Continue => continue,
                StepResult::InputRequired => panic!("Input required but none received"),
                StepResult::Halted => break
            }
        }
    }

    pub fn execute_until_yield(&mut self) -> YieldReason {
        loop {
            match self.step() {
                StepResult::Continue => continue,
                StepResult::InputRequired => return YieldReason::InputRequired,
                StepResult::Halted => return YieldReason::Halted
            }
        }
    }

    fn step(&mut self) -> StepResult {
        let program = &mut self.ram;
        let base = &mut self.base;
        let instruction = Instruction::parse(program, self.pointer);
        let steps = instruction.steps();

        match instruction {
            Instruction::Add(left, right, dest) => {
                dest.write(program, *base, left.read(program, *base) + right.read(program, *base))
            },

            Instruction::Multiply(left, right, dest) => {
                dest.write(program, *base, left.read(program, *base) * right.read(program, *base))
            },

            Instruction::Input(dest) => {
                let input = match self.stdin.pop_front() {
                    None => return StepResult::InputRequired,
                    Some(v) => v
                };

                dest.write(program, *base, input);
            },

            Instruction::Output(src) => {
                self.stdout.push_back(src.read(program, *base));
            },

            Instruction::JumpTrue(condition, dest) => {
                if condition.read(program, *base) != 0 {
                    self.pointer = dest.read(program, *base) as usize;
                    return StepResult::Continue;
                }
            },

            Instruction::JumpFalse(condition, dest) => {
                if condition.read(program, *base) == 0 {
                    self.pointer = dest.read(program, *base) as usize;
                    return StepResult::Continue;
                }
            },

            Instruction::LessThan(left, right, dest) => {
                dest.write(program, *base, if left.read(program, *base) < right.read(program, *base) { 1 } else { 0 });
            },

            Instruction::Equals(left, right, dest) => {
                dest.write(program, *base, if left.read(program, *base) == right.read(program, *base) { 1 } else { 0 });
            },

            Instruction::AdjustBase(offset) => {
                *base += offset.read(program, *base);
            }

            Instruction::Halt => return StepResult::Halted
        }

        self.pointer += steps;
        StepResult::Continue
    }
}

#[derive(Debug)]
enum Instruction {
    Add(ReadValue, ReadValue, WriteValue),
    Multiply(ReadValue, ReadValue, WriteValue),
    Input(WriteValue),
    Output(ReadValue),
    JumpTrue(ReadValue, ReadValue),
    JumpFalse(ReadValue, ReadValue),
    LessThan(ReadValue, ReadValue, WriteValue),
    Equals(ReadValue, ReadValue, WriteValue),
    AdjustBase(ReadValue),
    Halt
}

impl Instruction {
    fn parse(program: &Vec<i64>, pointer: usize) -> Instruction {
        let opcode = program[pointer];

        let mode1 = (opcode / 100) % 10;
        let mode2 = (opcode / 1000) % 10;
        let mode3 = (opcode / 10000) % 10;

        let instruction = match opcode % 100 {
            1 => Instruction::Add(
                ReadValue::new(program[pointer + 1], mode1),
                ReadValue::new(program[pointer + 2], mode2),
                WriteValue::new(program[pointer + 3], mode3),
            ),
            2 => Instruction::Multiply(
                ReadValue::new(program[pointer + 1], mode1),
                ReadValue::new(program[pointer + 2], mode2),
                WriteValue::new(program[pointer + 3], mode3),
            ),
            3 => Instruction::Input(
                WriteValue::new(program[pointer + 1], mode1)
            ),
            4 => Instruction::Output(
                ReadValue::new(program[pointer + 1], mode1)
            ),
            5 => Instruction::JumpTrue(
                ReadValue::new(program[pointer + 1], mode1),
                ReadValue::new(program[pointer + 2], mode2)
            ),
            6 => Instruction::JumpFalse(
                ReadValue::new(program[pointer + 1], mode1),
                ReadValue::new(program[pointer + 2], mode2)
            ),
            7 => Instruction::LessThan(
                ReadValue::new(program[pointer + 1], mode1),
                ReadValue::new(program[pointer + 2], mode2),
                WriteValue::new(program[pointer + 3], mode3),
            ),
            8 => Instruction::Equals(
                ReadValue::new(program[pointer + 1], mode1),
                ReadValue::new(program[pointer + 2], mode2),
                WriteValue::new(program[pointer + 3], mode3),
            ),
            9 => Instruction::AdjustBase(
                ReadValue::new(program[pointer + 1], mode1)
            ),
            99 => Instruction::Halt,
            _ => panic!("Unknown opcode {} at pointer {}", opcode, pointer)
        };

        instruction
    }

    fn steps(&self) -> usize {
        match self {
            Instruction::Add(..)        => 4,
            Instruction::Multiply(..)   => 4,
            Instruction::Input(..)      => 2,
            Instruction::Output(..)     => 2,
            Instruction::JumpTrue(..)   => 3,
            Instruction::JumpFalse(..)  => 3,
            Instruction::LessThan(..)   => 4,
            Instruction::Equals(..)     => 4,
            Instruction::AdjustBase(..) => 2,
            Instruction::Halt           => 0
        }
    }
}

#[derive(Debug)]
enum ReadValue {
    Position(i64),
    Immediate(i64),
    Relative(i64)
}

impl ReadValue {
    fn new(value: i64, mode: i64) -> ReadValue {
        match mode {
            0 => ReadValue::Position(value),
            1 => ReadValue::Immediate(value),
            2 => ReadValue::Relative(value),
            _ => panic!("Unsupported read mode: {:?}", mode)
        }
    }

    fn read(&self, program: &Vec<i64>, base: i64) -> i64 {
        let position = match *self {
            ReadValue::Position(position) => position as usize,
            ReadValue::Relative(position) => (position + base) as usize,
            ReadValue::Immediate(value) => return value,
        };

        *program.get(position).unwrap_or(&0)
    }
}

#[derive(Debug)]
enum WriteValue {
    Position(i64),
    Relative(i64)
}

impl WriteValue {
    fn new(value: i64, mode: i64) -> WriteValue {
        match mode {
            0 => WriteValue::Position(value),
            2 => WriteValue::Relative(value),
            _ => panic!("Unsupported write mode: {:?}", mode)
        }
    }

    fn write(&self, program: &mut Vec<i64>, base: i64, value: i64) {
        let position: usize = match *self {
            WriteValue::Position(position) => position as usize,
            WriteValue::Relative(position) => (position + base) as usize
        };

        if position >= program.len() {
            // memory can grow dynamically if we try to access a non-existant index
            program.resize_with(position + 1, Default::default);
        }

        program[position] = value;
    }
}
