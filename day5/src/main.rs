extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "inputstack.pest"]
pub struct StacksParser;

/// A representation of a crate
#[derive(Debug)]
struct ElfCrate<'a>(&'a str);
/// A representation of a stack of crates
#[derive(Debug)]
struct CrateStack<'a> {
    stack: Vec<ElfCrate<'a>>,
}
/// A representation of the loading dock
#[derive(Debug)]
struct Dock<'a> {
    stacks: Vec<CrateStack<'a>>,
}

impl<'a> Dock<'a> {
    fn sort(&mut self) {
        for stack in &mut self.stacks {
            stack.stack.reverse();
        }
    }

    #[allow(dead_code)]
    fn apply_move_one_by_one(&mut self, some_move: &Move) {
        for _ in 0..some_move.amount {
            let somecrate = self.stacks[some_move.from - 1].stack.pop().unwrap();
            self.stacks[some_move.to - 1].stack.push(somecrate);
        }
    }

    fn apply_move_all_in_one(&mut self, some_move: &Move) {
        let mut all_crates = Vec::new();
        for _ in 0..some_move.amount {
            all_crates.push(self.stacks[some_move.from - 1].stack.pop().unwrap());
        }
        all_crates.reverse();
        self.stacks[some_move.to - 1].stack.append(&mut all_crates);
    }

    fn print_top_crates(&self) {
        for stack in &self.stacks {
            if let Some(crat) = stack.stack.last() {
                print!("{:?}", crat);
            } else {
                print!(" ");
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum ParserState {
    NotStarted,
    CrateRowsStarted,
    CrateRows,
}

/// A move of x crates from and to another stack
#[derive(Debug)]
struct Move {
    amount: u32,
    from: usize,
    to: usize,
}

type Moves = Vec<Move>;

fn main() {
    let unparsed_file = std::fs::read_to_string("input.txt").expect("cannot read file");
    let file = StacksParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    let mut dock = Dock { stacks: Vec::new() };
    let mut parserstate = ParserState::NotStarted;
    let mut moves = Moves::new();

    for record in file.into_inner() {
        match record.as_rule() {
            Rule::craterow => {
                if parserstate == ParserState::NotStarted {
                    parserstate = ParserState::CrateRowsStarted;
                }

                for (stacknum, field) in record.into_inner().enumerate() {
                    if parserstate == ParserState::CrateRowsStarted {
                        dock.stacks.push(CrateStack { stack: Vec::new() });
                    }

                    match field.as_rule() {
                        Rule::emptycrate => (),
                        Rule::elfcrate => {
                            dock.stacks[stacknum]
                                .stack
                                .push(ElfCrate(field.into_inner().as_str()));
                        }
                        _ => (),
                    }
                }

                parserstate = ParserState::CrateRows;
            }
            Rule::moverow => {
                let mut somemove = Move {
                    amount: 0,
                    from: 0,
                    to: 0,
                };

                for field in record.into_inner() {
                    match field.as_rule() {
                        Rule::moveamount => {
                            somemove.amount = field.into_inner().as_str().parse().unwrap()
                        }
                        Rule::movefrom => {
                            somemove.from = field.into_inner().as_str().parse().unwrap()
                        }
                        Rule::moveto => somemove.to = field.into_inner().as_str().parse().unwrap(),
                        _ => (),
                    }
                }

                moves.push(somemove);
            }
            Rule::EOI => (),
            _ => (),
        }
    }

    dock.sort();
    for some_move in moves {
        dock.apply_move_all_in_one(&some_move);
    }
    dock.print_top_crates();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_crates_all() {
        let mut dock = Dock {
            stacks: vec![
                CrateStack {
                    stack: vec![ElfCrate("N"), ElfCrate("Z")],
                },
                CrateStack {
                    stack: vec![ElfCrate("D"), ElfCrate("C"), ElfCrate("M")],
                },
                CrateStack {
                    stack: vec![ElfCrate("P")],
                },
            ],
        };
        dock.sort();

        let move1 = Move {
            amount: 1,
            from: 2,
            to: 1,
        };
        dock.apply_move_all_in_one(&move1);

        let move2 = Move {
            amount: 3,
            from: 1,
            to: 3,
        };
        dock.apply_move_all_in_one(&move2);

        let move3 = Move {
            amount: 2,
            from: 2,
            to: 1,
        };
        dock.apply_move_all_in_one(&move3);

        let move4 = Move {
            amount: 1,
            from: 1,
            to: 2,
        };
        dock.apply_move_all_in_one(&move4);
        println!("All in one:");
        dock.print_top_crates();
    }

    #[test]
    fn move_crates_one() {
        let mut dock = Dock {
            stacks: vec![
                CrateStack {
                    stack: vec![ElfCrate("N"), ElfCrate("Z")],
                },
                CrateStack {
                    stack: vec![ElfCrate("D"), ElfCrate("C"), ElfCrate("M")],
                },
                CrateStack {
                    stack: vec![ElfCrate("P")],
                },
            ],
        };
        dock.sort();
        let move1 = Move {
            amount: 1,
            from: 2,
            to: 1,
        };
        dock.apply_move_one_by_one(&move1);

        let move2 = Move {
            amount: 3,
            from: 1,
            to: 3,
        };
        dock.apply_move_one_by_one(&move2);

        let move3 = Move {
            amount: 2,
            from: 2,
            to: 1,
        };
        dock.apply_move_one_by_one(&move3);

        let move4 = Move {
            amount: 1,
            from: 1,
            to: 2,
        };
        dock.apply_move_one_by_one(&move4);
        //println!("One by one:");
        //dock.print_top_crates();
    }
}
