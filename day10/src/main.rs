use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{BTreeMap, VecDeque};

static NOOP_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"noop").unwrap());
static ADDX_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"addx\s([-]?[0-9]*)").unwrap());

struct Cpu<'a> {
    queue: VecDeque<(usize, Instruction)>,
    cycle: usize,
    registers: BTreeMap<&'a str, i64>,
}

impl<'a> Cpu<'a> {
    fn exec_cycle(&mut self) {
        let mut done = false;
        if let Some(instr) = self.queue.get_mut(0) {
            match instr {
                (cycles_left, Instruction::Addx(value)) => {
                    // Takes two cycles
                    if cycles_left > &mut 1 {
                        instr.0 -= 1;
                    } else {
                        done = true;
                        if let Some(x) = self.registers.get_mut("X") {
                            *x += *value;
                        }
                    }
                }
                (_, Instruction::Noop) => {
                    // Takes one cycle
                    done = true;
                }
            }
        }
        if done {
            self.queue.pop_front();
        }
        self.cycle += 1;
    }

    fn load_program(&mut self, instructions: Vec<Instruction>) {
        let mut queue = VecDeque::with_capacity(instructions.len());

        for instr in instructions {
            match instr {
                Instruction::Addx(_value) => {
                    queue.push_back((2, instr));
                }
                Instruction::Noop => {
                    queue.push_back((1, instr));
                }
            }
        }

        self.queue = queue;
    }

    fn instrument(&self, register: &str) -> Option<i64> {
        self.registers.get(register).copied()
    }
}
impl<'a> Default for Cpu<'a> {
    fn default() -> Self {
        let mut reg = BTreeMap::new();
        reg.insert("X", 1);

        Cpu {
            queue: VecDeque::new(),
            cycle: 1,
            registers: reg,
        }
    }
}

enum Instruction {
    Addx(i64),
    Noop,
}

#[derive(Debug)]
struct Crt<'a> {
    /// Two dimensional array of single string chars
    rows: Vec<Vec<&'a str>>,
}
impl<'a> Crt<'a> {
    fn draw(&mut self, cycle: usize, symbol: &'a str) {
        match cycle {
            1..=40 => {
                self.rows[0][cycle - 1] = symbol;
            }
            41..=80 => {
                self.rows[1][cycle - 40 - 1] = symbol;
            }
            81..=120 => {
                self.rows[2][cycle - 80 - 1] = symbol;
            }
            121..=160 => {
                self.rows[3][cycle - 120 - 1] = symbol;
            }
            161..=200 => {
                self.rows[4][cycle - 160 - 1] = symbol;
            }
            201..=240 => {
                self.rows[5][cycle - 200 - 1] = symbol;
            }
            _ => {}
        }
    }

    fn take_screenshot(&self) -> Vec<String> {
        let mut print = Vec::new();
        for row in &self.rows {
            let out = row.concat();
            print.push(out);
        }
        print
    }
}

impl<'a> Default for Crt<'a> {
    fn default() -> Self {
        Crt {
            rows: vec![
                vec!["!"; 40],
                vec!["!"; 40],
                vec!["!"; 40],
                vec!["!"; 40],
                vec!["!"; 40],
                vec!["!"; 40],
            ],
        }
    }
}

#[derive(Default)]
struct Device<'a> {
    cpu: Cpu<'a>,
    monitor: Crt<'a>,
}
impl<'a> Device<'a> {
    fn run(&mut self, cycles: usize) {
        for _cycle in 1..cycles {
            let sprite_center = self.cpu.instrument("X").unwrap();
            let cycle_pos = (self.cpu.cycle % 40) as i64;
            let sprite = Device::get_sprite_for_register(sprite_center, cycle_pos);
            self.monitor.draw(self.cpu.cycle, sprite);
            self.cpu.exec_cycle();
        }
    }

    fn get_sprite_for_register(sprite_center: i64, cpu_cycle: i64) -> &'a str {
        let mut symbol = ".";

        let sprite_start = sprite_center - 1;
        let sprite_end = sprite_center + 1;
        if cpu_cycle > sprite_start && cpu_cycle - 1 <= sprite_end {
            symbol = "#";
        }
        symbol
    }
}
fn asm_from_str(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            if NOOP_REGEX.is_match(line) {
                return Instruction::Noop;
            }
            if let Some(captures) = ADDX_REGEX.captures(line) {
                return Instruction::Addx(
                    captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                );
            }
            panic!("Invalid line");
        })
        .collect::<Vec<Instruction>>()
}

fn main() {
    let program = asm_from_str(include_str!("../input.txt"));
    let mut cpu = Cpu::default();
    let mut signal_strengths = Vec::new();
    cpu.load_program(program);

    for _cycle in 1..220 {
        cpu.exec_cycle();
        match cpu.cycle {
            // During the 20th cycle
            20 => {
                signal_strengths.push(cpu.cycle as i64 * cpu.instrument("X").unwrap());
            }
            60 => {
                signal_strengths.push(cpu.cycle as i64 * cpu.instrument("X").unwrap());
            }
            100 => {
                signal_strengths.push(cpu.cycle as i64 * cpu.instrument("X").unwrap());
            }
            140 => {
                signal_strengths.push(cpu.cycle as i64 * cpu.instrument("X").unwrap());
            }
            180 => {
                signal_strengths.push(cpu.cycle as i64 * cpu.instrument("X").unwrap());
            }
            220 => {
                signal_strengths.push(cpu.cycle as i64 * cpu.instrument("X").unwrap());
            }
            _ => {}
        }
    }

    println!(
        "Sum of signal strengths: {}",
        signal_strengths.iter().sum::<i64>()
    );

    // Part 2
    let mut device = Device::default();
    let program = asm_from_str(include_str!("../input.txt"));
    device.cpu.load_program(program);
    device.run(240);
    let out = device.monitor.take_screenshot();
    for line in out {
        println!("{line}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crt_drawing() {
        let mut device = Device::default();
        let program = asm_from_str(get_lines());
        device.cpu.load_program(program);
        device.run(250);

        let out = device.monitor.take_screenshot();
        assert_eq!(
            String::from("##..##..##..##..##..##..##..##..##..##.."),
            out[0]
        );
        assert_eq!(
            String::from("###...###...###...###...###...###...###."),
            out[1]
        );
        assert_eq!(
            String::from("####....####....####....####....####...."),
            out[2]
        );
        assert_eq!(
            String::from("#####.....#####.....#####.....#####....."),
            out[3]
        );
        assert_eq!(
            String::from("#######.......#######.......#######....."),
            out[5]
        );
    }

    #[test]
    fn symbol() {
        assert_eq!("#", Device::get_sprite_for_register(1, 1));
        assert_eq!("#", Device::get_sprite_for_register(1, 2));

        assert_eq!(".", Device::get_sprite_for_register(16, 3));
        assert_eq!(".", Device::get_sprite_for_register(16, 4));

        assert_eq!("#", Device::get_sprite_for_register(5, 5));
        assert_eq!("#", Device::get_sprite_for_register(5, 6));

        assert_eq!(".", Device::get_sprite_for_register(11, 7));
        assert_eq!(".", Device::get_sprite_for_register(11, 8));

        assert_eq!("#", Device::get_sprite_for_register(8, 9));
        assert_eq!("#", Device::get_sprite_for_register(8, 10));
    }

    #[test]
    fn test_cpu() {
        let program = vec![
            Instruction::Noop,
            Instruction::Addx(3),
            Instruction::Addx(-5),
        ];

        let mut cpu = Cpu::default();
        cpu.load_program(program);
        assert_eq!(1, cpu.instrument("X").unwrap());

        // At the start of the first cycle, the noop instruction begins execution.
        // During the first cycle, X is 1. After the first cycle, the noop instruction finishes execution, doing nothing.
        cpu.exec_cycle();
        assert_eq!(1, cpu.instrument("X").unwrap());

        // At the start of the second cycle, the addx 3 instruction begins execution. During the second cycle, X is still 1.
        cpu.exec_cycle();
        assert_eq!(1, cpu.instrument("X").unwrap());

        // During the third cycle, X is still 1. After the third cycle, the addx 3 instruction finishes execution, setting X to 4.
        cpu.exec_cycle();
        assert_eq!(4, cpu.instrument("X").unwrap());

        // At the start of the fourth cycle, the addx -5 instruction begins execution. During the fourth cycle, X is still 4
        cpu.exec_cycle();
        assert_eq!(4, cpu.instrument("X").unwrap());

        // During the fifth cycle, X is still 4. After the fifth cycle, the addx -5 instruction finishes execution, setting X to -1.
        cpu.exec_cycle();
        assert_eq!(-1, cpu.instrument("X").unwrap());
    }

    #[test]
    fn test_instrumentation() {
        let program = asm_from_str(get_lines());

        let mut signal_strengths = Vec::new();
        let mut cpu = Cpu::default();
        cpu.load_program(program);

        for _cycle in 0..220 {
            cpu.exec_cycle();
            match cpu.cycle {
                // During the 20th cycle
                20 => {
                    assert_eq!(21, cpu.instrument("X").unwrap());
                    signal_strengths.push(cpu.cycle as i64 * cpu.instrument("X").unwrap());
                }

                60 => {
                    assert_eq!(19, cpu.instrument("X").unwrap());
                    signal_strengths.push(cpu.cycle as i64 * cpu.instrument("X").unwrap());
                }

                100 => {
                    assert_eq!(18, cpu.instrument("X").unwrap());
                    signal_strengths.push(cpu.cycle as i64 * cpu.instrument("X").unwrap());
                }
                140 => {
                    assert_eq!(21, cpu.instrument("X").unwrap());
                    signal_strengths.push(cpu.cycle as i64 * cpu.instrument("X").unwrap());
                }
                180 => {
                    assert_eq!(16, cpu.instrument("X").unwrap());
                    signal_strengths.push(cpu.cycle as i64 * cpu.instrument("X").unwrap());
                }
                220 => {
                    assert_eq!(18, cpu.instrument("X").unwrap());
                    signal_strengths.push(cpu.cycle as i64 * cpu.instrument("X").unwrap());
                }
                _ => {}
            }
        }
        assert_eq!(13140, signal_strengths.iter().sum::<i64>());
    }

    fn get_lines() -> &'static str {
        "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
    }
}
