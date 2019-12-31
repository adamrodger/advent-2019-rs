pub enum StepResult {
    Continue,
    Halted
}

pub struct IntCodeEmulator {
    ram: Vec<i64>,
    pointer: usize,
}

impl IntCodeEmulator {
    pub fn new(program: Vec<i64>) -> IntCodeEmulator {
        IntCodeEmulator {
            ram: program,
            pointer: 0,
        }
    }

    pub fn ram(&self) -> &Vec<i64> {
        &self.ram
    }

    pub fn execute(&mut self) {
        loop {
            match self.step() {
                StepResult::Continue => continue,
                StepResult::Halted => break
            }
        }
    }

    fn step(&mut self) -> StepResult {
        let program = &mut self.ram;
        let instruction = Instruction::parse(&program, self.pointer);

        match instruction {
            Instruction::Add(a, b, c) => c.write(program, a.read(&program) + b.read(&program)),
            Instruction::Multiply(a, b, c) => c.write(program, a.read(&program) * b.read(&program)),
            Instruction::Halt => return StepResult::Halted
        }

        self.pointer += 4;
        StepResult::Continue
    }
}

enum Instruction {
    Add(InputValue, InputValue, OutputValue),
    Multiply(InputValue, InputValue, OutputValue),
    Halt
}

impl Instruction {
    fn parse(program: &Vec<i64>, pointer: usize) -> Instruction {
        let opcode = program[pointer];

        let instruction = match opcode {
            1 => Instruction::Add(
                InputValue::new(pointer + 1),
                InputValue::new(pointer + 2),
                OutputValue::new(pointer + 3),
            ),
            2 => Instruction::Multiply(
                InputValue::new(pointer + 1),
                InputValue::new(pointer + 2),
                OutputValue::new(pointer + 3),
            ),
            99 => Instruction::Halt,
            _ => panic!("Unknown opcode {} at pointer {}", opcode, pointer)
        };

        instruction
    }
}

struct InputValue(usize);

impl InputValue {
    fn new(position: usize) -> InputValue {
        InputValue(position)
    }

    fn read(&self, program: &Vec<i64>) -> i64 {
        program[self.0]
    }
}

struct OutputValue(usize);

impl OutputValue {
    fn new(position: usize) -> OutputValue {
        OutputValue(position)
    }

    fn write(&self, program: &mut Vec<i64>, value: i64) {
        program[self.0] = value;
    }
}
