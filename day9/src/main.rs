use std::collections::HashSet;

#[derive(Debug)]
struct Head(Position);
impl Head {
    fn step(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.0.y += 1,
            Direction::Down => self.0.y -= 1,
            Direction::Left => self.0.x -= 1,
            Direction::Right => self.0.x += 1,
        }
    }
}
#[derive(Debug)]
struct RopeVector {
    x: isize,
    y: isize,
}
#[derive(Debug)]
struct Tail(Position, HashSet<Position>);
impl Tail {
    fn step(&mut self, vector: RopeVector) {
        match (vector.x, vector.y) {
            (0, 0)
            | (0, 1)
            | (1, 0)
            | (1, 1)
            | (0, -1)
            | (-1, -1)
            | (-1, 0)
            | (-1, 1)
            | (1, -1) => {} // No movement
            (x, 0) => {
                if x < 0 {
                    self.0.x -= 1;
                } else {
                    self.0.x += 1;
                }
            }
            (0, y) => {
                if y < 0 {
                    self.0.y -= 1;
                } else {
                    self.0.y += 1;
                }
            }
            (x, -2) => {
                self.0.y -= 1;
                if x < 0 {
                    self.0.x -= 1;
                } else {
                    self.0.x += 1;
                }
            }
            (x, 2) => {
                self.0.y += 1;
                if x < 0 {
                    self.0.x -= 1;
                } else {
                    self.0.x += 1;
                }
            }
            (2, y) => {
                self.0.x += 1;
                if y < 0 {
                    self.0.y -= 1;
                } else {
                    self.0.y += 1;
                }
            }
            (-2, y) => {
                self.0.x -= 1;
                if y < 0 {
                    self.0.y -= 1;
                } else {
                    self.0.y += 1;
                }
            }
            (x, y) => {
                panic!("Unkown movement... x:{}, y:{}", x, y)
            }
        };

        self.1.insert(self.0);
    }
}

fn get_vector(prev: &Position, next: &Position) -> RopeVector {
    RopeVector {
        x: prev.x - next.x,
        y: prev.y - next.y,
    }
}
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Position {
    x: isize,
    y: isize,
}
#[derive(Debug)]
struct Rope {
    head: Head,
    tail: Vec<Tail>,
}
impl Rope {
    fn apply_move(&mut self, mv: Move) {
        for _step in 0..mv.dist {
            self.head.step(mv.dir);
            let ropevector = get_vector(&self.head.0, &self.tail[0].0);
            self.tail[0].step(ropevector);
            for idx in 1..self.tail.len() {
                let vector = get_vector(&self.tail[idx - 1].0, &self.tail[idx].0);
                self.tail[idx].step(vector);
            }
        }
    }
}
impl Default for Rope {
    fn default() -> Self {
        Rope {
            head: Head(Position { x: 0, y: 0 }),
            tail: vec![Tail(Position { x: 0, y: 0 }, HashSet::new())],
        }
    }
}
#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone)]
struct Move {
    dir: Direction,
    dist: usize,
}
impl From<&str> for Move {
    fn from(from: &str) -> Self {
        let mut split = from.split_whitespace();
        Move {
            dir: match split.next().unwrap() {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid move"),
            },
            dist: split.next().unwrap().parse().unwrap(),
        }
    }
}
fn main() {
    let moves: Vec<Move> = include_str!("../input.txt")
        .lines()
        .map(Move::from)
        .collect();

    let mut rope = Rope::default();
    for mv in &moves {
        rope.apply_move(*mv);
    }

    println!(
        "Number of positions visited at least once by the tail: {}",
        rope.tail[0].1.len()
    );

    let mut longrope = Rope {
        head: Head(Position { x: 0, y: 0 }),
        tail: vec![
            Tail(Position { x: 0, y: 0 }, HashSet::new()),
            Tail(Position { x: 0, y: 0 }, HashSet::new()),
            Tail(Position { x: 0, y: 0 }, HashSet::new()),
            Tail(Position { x: 0, y: 0 }, HashSet::new()),
            Tail(Position { x: 0, y: 0 }, HashSet::new()),
            Tail(Position { x: 0, y: 0 }, HashSet::new()),
            Tail(Position { x: 0, y: 0 }, HashSet::new()),
            Tail(Position { x: 0, y: 0 }, HashSet::new()),
            Tail(Position { x: 0, y: 0 }, HashSet::new()),
        ],
    };
    for mv in moves {
        longrope.apply_move(mv);
    }

    println!(
        "Number of positions visited by tail: {}",
        longrope.tail[8].1.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_tail() {
        let mut rope = Rope {
            head: Head(Position { x: 0, y: 0 }),
            tail: vec![
                Tail(Position { x: 0, y: 0 }, HashSet::new()),
                Tail(Position { x: 0, y: 0 }, HashSet::new()),
                Tail(Position { x: 0, y: 0 }, HashSet::new()),
                Tail(Position { x: 0, y: 0 }, HashSet::new()),
                Tail(Position { x: 0, y: 0 }, HashSet::new()),
                Tail(Position { x: 0, y: 0 }, HashSet::new()),
                Tail(Position { x: 0, y: 0 }, HashSet::new()),
                Tail(Position { x: 0, y: 0 }, HashSet::new()),
                Tail(Position { x: 0, y: 0 }, HashSet::new()),
            ],
        };

        let mv = Move {
            dir: Direction::Right,
            dist: 4,
        };
        rope.apply_move(mv);
        assert_eq!(0, rope.tail[8].0.x);
    }

    #[test]
    fn apply_moves() {
        let mut rope = Rope {
            head: Head(Position { x: 0, y: 0 }),
            tail: vec![Tail(Position { x: 0, y: 0 }, HashSet::new())],
        };

        let mv = Move {
            dir: Direction::Right,
            dist: 4,
        };

        rope.apply_move(mv);
        assert_eq!(4, rope.head.0.x);
        assert_eq!(3, rope.tail[0].0.x);
        assert_eq!(0, rope.head.0.y);
        assert_eq!(0, rope.tail[0].0.y);

        // up 4
        let mv = Move {
            dir: Direction::Up,
            dist: 1,
        };

        rope.apply_move(mv);

        assert_eq!(4, rope.head.0.x);
        assert_eq!(3, rope.tail[0].0.x);
        assert_eq!(1, rope.head.0.y);
        assert_eq!(0, rope.tail[0].0.y);

        rope.apply_move(mv);

        assert_eq!(4, rope.head.0.x);
        assert_eq!(4, rope.tail[0].0.x);
        assert_eq!(2, rope.head.0.y);
        assert_eq!(1, rope.tail[0].0.y);

        rope.apply_move(mv);

        assert_eq!(4, rope.head.0.x);
        assert_eq!(4, rope.tail[0].0.x);
        assert_eq!(3, rope.head.0.y);
        assert_eq!(2, rope.tail[0].0.y);

        rope.apply_move(mv);

        assert_eq!(4, rope.head.0.x);
        assert_eq!(4, rope.tail[0].0.x);
        assert_eq!(4, rope.head.0.y);
        assert_eq!(3, rope.tail[0].0.y);

        // Left 3
        let mv = Move {
            dir: Direction::Left,
            dist: 1,
        };

        rope.apply_move(mv);

        assert_eq!(3, rope.head.0.x);
        assert_eq!(4, rope.tail[0].0.x);
        assert_eq!(4, rope.head.0.y);
        assert_eq!(3, rope.tail[0].0.y);

        rope.apply_move(mv);

        assert_eq!(2, rope.head.0.x);
        assert_eq!(3, rope.tail[0].0.x);
        assert_eq!(4, rope.head.0.y);
        assert_eq!(4, rope.tail[0].0.y);

        rope.apply_move(mv);

        assert_eq!(1, rope.head.0.x);
        assert_eq!(2, rope.tail[0].0.x);
        assert_eq!(4, rope.head.0.y);
        assert_eq!(4, rope.tail[0].0.y);

        // Down 1
        let mv = Move {
            dir: Direction::Down,
            dist: 1,
        };
        rope.apply_move(mv);

        assert_eq!(1, rope.head.0.x);
        assert_eq!(2, rope.tail[0].0.x);
        assert_eq!(3, rope.head.0.y);
        assert_eq!(4, rope.tail[0].0.y);

        // Right 4
        let mv = Move {
            dir: Direction::Right,
            dist: 1,
        };

        rope.apply_move(mv);

        assert_eq!(2, rope.head.0.x);
        assert_eq!(2, rope.tail[0].0.x);
        assert_eq!(3, rope.head.0.y);
        assert_eq!(4, rope.tail[0].0.y);

        rope.apply_move(mv);

        assert_eq!(3, rope.head.0.x);
        assert_eq!(2, rope.tail[0].0.x);
        assert_eq!(3, rope.head.0.y);
        assert_eq!(4, rope.tail[0].0.y);

        rope.apply_move(mv);

        assert_eq!(4, rope.head.0.x);
        assert_eq!(3, rope.tail[0].0.x);
        assert_eq!(3, rope.head.0.y);
        assert_eq!(3, rope.tail[0].0.y);

        rope.apply_move(mv);

        assert_eq!(5, rope.head.0.x);
        assert_eq!(4, rope.tail[0].0.x);
        assert_eq!(3, rope.head.0.y);
        assert_eq!(3, rope.tail[0].0.y);

        let mv = Move {
            dir: Direction::Down,
            dist: 1,
        };
        rope.apply_move(mv);

        assert_eq!(5, rope.head.0.x);
        assert_eq!(4, rope.tail[0].0.x);
        assert_eq!(2, rope.head.0.y);
        assert_eq!(3, rope.tail[0].0.y);

        let mv = Move {
            dir: Direction::Left,
            dist: 5,
        };
        rope.apply_move(mv);

        assert_eq!(0, rope.head.0.x);
        assert_eq!(1, rope.tail[0].0.x);
        assert_eq!(2, rope.head.0.y);
        assert_eq!(2, rope.tail[0].0.y);

        let mv = Move {
            dir: Direction::Right,
            dist: 2,
        };
        rope.apply_move(mv);

        assert_eq!(2, rope.head.0.x);
        assert_eq!(1, rope.tail[0].0.x);
        assert_eq!(2, rope.head.0.y);
        assert_eq!(2, rope.tail[0].0.y);

        let positions_visited = rope.tail[0].1.len();
        assert_eq!(13, positions_visited);
    }
}
