use std::collections::VecDeque;

#[derive(Debug)]
pub enum StepResult {
    Continue,
    InputRequired,
    Halted
}

#[derive(Debug)]
pub struct IntCodeEmulator {
    ram: Vec<i64>,
    pointer: usize,
    stdin: VecDeque<i64>,
    stdout: VecDeque<i64>
}

impl IntCodeEmulator {
    pub fn from_input(input: &str) -> IntCodeEmulator {
        let program = input.trim().split(',').map(|l| l.parse().expect("Unable to parse input")).collect();
        IntCodeEmulator::new(program)
    }

    pub fn new(program: Vec<i64>) -> IntCodeEmulator {
        IntCodeEmulator {
            ram: program,
            pointer: 0,
            stdin: VecDeque::new(),
            stdout: VecDeque::new(),
        }
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

    fn step(&mut self) -> StepResult {
        let program = &mut self.ram;
        let instruction = Instruction::parse(program, self.pointer);
        let steps = instruction.steps();

        match instruction {
            Instruction::Add(left, right, dest) => {
                dest.write(program, left.read(program) + right.read(program))
            },

            Instruction::Multiply(left, right, dest) => {
                dest.write(program, left.read(program) * right.read(program))
            },

            Instruction::Input(dest) => {
                let input = match self.stdin.pop_front() {
                    None => return StepResult::InputRequired,
                    Some(v) => v
                };

                dest.write(program, input);
            },

            Instruction::Output(src) => {
                self.stdout.push_back(src.read(program));
            },

            Instruction::JumpTrue(condition, dest) => {
                if condition.read(program) != 0 {
                    self.pointer = dest.read(program) as usize;
                    return StepResult::Continue;
                }
            },

            Instruction::JumpFalse(condition, dest) => {
                if condition.read(program) == 0 {
                    self.pointer = dest.read(program) as usize;
                    return StepResult::Continue;
                }
            },

            Instruction::LessThan(left, right, dest) => {
                dest.write(program, if left.read(program) < right.read(program) { 1 } else { 0 });
            },

            Instruction::Equals(left, right, dest) => {
                dest.write(program, if left.read(program) == right.read(program) { 1 } else { 0 });
            },

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
    Halt
}

impl Instruction {
    fn parse(program: &Vec<i64>, pointer: usize) -> Instruction {
        let opcode = program[pointer];

        let mode1: ParameterMode = ((opcode / 100) % 10).into();
        let mode2: ParameterMode = ((opcode / 1000) % 10).into();
        let _mode3: ParameterMode = ((opcode / 10000) % 10).into();

        let instruction = match opcode % 100 {
            1 => Instruction::Add(
                ReadValue::new(program[pointer + 1], mode1),
                ReadValue::new(program[pointer + 2], mode2),
                WriteValue::new(program[pointer + 3]),
            ),
            2 => Instruction::Multiply(
                ReadValue::new(program[pointer + 1], mode1),
                ReadValue::new(program[pointer + 2], mode2),
                WriteValue::new(program[pointer + 3]),
            ),
            3 => Instruction::Input(
                WriteValue::new(program[pointer + 1])
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
                WriteValue::new(program[pointer + 3]),
            ),
            8 => Instruction::Equals(
                ReadValue::new(program[pointer + 1], mode1),
                ReadValue::new(program[pointer + 2], mode2),
                WriteValue::new(program[pointer + 3]),
            ),
            99 => Instruction::Halt,
            _ => panic!("Unknown opcode {} at pointer {}", opcode, pointer)
        };

        instruction
    }

    fn steps(&self) -> usize {
        match self {
            Instruction::Add(..)       => 4,
            Instruction::Multiply(..)  => 4,
            Instruction::Input(..)     => 2,
            Instruction::Output(..)    => 2,
            Instruction::JumpTrue(..)  => 3,
            Instruction::JumpFalse(..) => 3,
            Instruction::LessThan(..)  => 4,
            Instruction::Equals(..)    => 4,
            Instruction::Halt          => 0
        }
    }
}

#[derive(Debug)]
struct ReadValue {
    value: i64,
    mode: ParameterMode
}

impl ReadValue {
    fn new(value: i64, mode: ParameterMode) -> ReadValue {
        ReadValue {
            value,
            mode
        }
    }

    fn read(&self, program: &Vec<i64>) -> i64 {
        match self.mode {
            ParameterMode::Position => program[self.value as usize],
            ParameterMode::Immediate => self.value,
        }
    }
}

#[derive(Debug)]
struct WriteValue {
    position: usize
}

impl WriteValue {
    fn new(position: i64) -> WriteValue {
        WriteValue {
            position: position as usize
        }
    }

    fn write(&self, program: &mut Vec<i64>, value: i64) {
        program[self.position] = value;
    }
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate
}

impl From<i64> for ParameterMode {
    fn from(v: i64) -> ParameterMode {
        match v {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!(format!("Unsupported parameter mode {}", v))
        }
    }
}
