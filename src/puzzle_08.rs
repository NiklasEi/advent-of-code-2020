use std::option::Option::Some;

fn run_program(input: Vec<String>) -> isize {
    let mut program = parse_program(input);
    program.run().unwrap()
}

fn run_program_with_command_replace(input: Vec<String>) -> Option<isize> {
    let base_program = parse_program(input);

    // brute force, what else ;P
    let mut replace_index: usize = 0;
    while replace_index < base_program.instructions.len() {
        let mut replaced_instructions: Vec<(Instruction, Option<isize>)> =
            base_program.instructions.clone();
        let current_instruction = base_program.instructions.iter().nth(replace_index).unwrap();
        if let Instruction::ACC(_) = current_instruction.0 {
            replace_index += 1;
            continue;
        } else if let Instruction::NOP(value) = current_instruction.0 {
            replaced_instructions = replaced_instructions
                .iter()
                .enumerate()
                .map(|(index, instruction)| -> (Instruction, Option<isize>) {
                    if index == replace_index {
                        (Instruction::JMP(value), None)
                    } else {
                        instruction.clone()
                    }
                })
                .collect();
        } else if let Instruction::JMP(value) = current_instruction.0 {
            replaced_instructions = replaced_instructions
                .iter()
                .enumerate()
                .map(|(index, instruction)| -> (Instruction, Option<isize>) {
                    if index == replace_index {
                        (Instruction::NOP(value), None)
                    } else {
                        instruction.clone()
                    }
                })
                .collect();
        }
        let mut program = Program {
            instructions: replaced_instructions,
            accumulator: 0,
        };
        if let Some(accumulator) = program.run_to_end() {
            return Some(accumulator);
        }
        replace_index += 1;
    }
    None
}

fn parse_program(input: Vec<String>) -> Program {
    let mut program = Program::default();
    for line in input {
        let mut parts = line.split(" ");
        let instruction_raw = parts.next().unwrap();
        match instruction_raw {
            "nop" => program.instructions.push((
                Instruction::NOP(parts.next().unwrap().parse::<isize>().unwrap()),
                None,
            )),
            "jmp" => program.instructions.push((
                Instruction::JMP(parts.next().unwrap().parse::<isize>().unwrap()),
                None,
            )),
            "acc" => program.instructions.push((
                Instruction::ACC(parts.next().unwrap().parse::<isize>().unwrap()),
                None,
            )),
            _ => (),
        }
    }
    program
}

#[derive(Default)]
struct Program {
    accumulator: isize,
    pub instructions: Vec<(Instruction, Option<isize>)>,
}

impl Program {
    fn run(&mut self) -> Option<isize> {
        let mut index: usize = 0;
        while index < self.instructions.len() {
            let instruction = self.instructions.iter_mut().nth(index).unwrap();
            if instruction.1.is_some() {
                // been here, done that
                return Some(self.accumulator);
            }
            match instruction.0 {
                Instruction::NOP(_) => index += 1,
                Instruction::JMP(jump) => index = (jump + index as isize) as usize,
                Instruction::ACC(acc) => {
                    self.accumulator += acc;
                    index += 1;
                }
            }
            instruction.1 = Some(self.accumulator);
        }
        return Some(self.accumulator);
    }

    fn run_to_end(&mut self) -> Option<isize> {
        let last_index = self.instructions.len();
        let mut index: usize = 0;
        while index < self.instructions.len() {
            let instruction = self.instructions.iter_mut().nth(index).unwrap();
            if instruction.1.is_some() {
                return None;
            }
            match instruction.0 {
                Instruction::NOP(_) => index += 1,
                Instruction::JMP(jump) => index = (jump + index as isize) as usize,
                Instruction::ACC(acc) => {
                    self.accumulator += acc;
                    index += 1;
                }
            }
            if index == last_index {
                return Some(self.accumulator);
            }
            instruction.1 = Some(self.accumulator);
        }
        return None;
    }
}

#[derive(Clone)]
enum Instruction {
    NOP(isize),
    JMP(isize),
    ACC(isize),
}

#[cfg(test)]
mod tests {
    use crate::puzzle_08::{run_program, run_program_with_command_replace};
    use crate::read_file::read_file_to_vec;

    #[test]
    fn solve_day_08_1() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_08.txt");
        println!("Program returns with: {:?}", run_program(input));
    }

    #[test]
    fn solve_day_08_2() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_08.txt");
        println!(
            "Program returns with: {:?}",
            run_program_with_command_replace(input).unwrap()
        );
    }
}
