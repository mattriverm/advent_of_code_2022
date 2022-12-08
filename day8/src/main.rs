#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct Tree(u8);
impl From<&str> for Tree {
    fn from(from: &str) -> Self {
        Self(from.parse().unwrap())
    }
}
#[derive(Debug)]
struct Forest {
    rows: Vec<Vec<Tree>>,
}
impl Forest {
    fn new(forest: Vec<Vec<Tree>>) -> Self {
        Forest { rows: forest }
    }

    fn visible_from_left_of_pos(&self, x: usize, y: usize) -> bool {
        if x == 0 {
            return true;
        }

        let row = &self.rows[y];
        let tree = row[x];

        for pos in 0..x {
            if row[pos] >= tree {
                return false;
            }
        }

        true
    }

    fn visible_from_right_of_pos(&self, x: usize, y: usize) -> bool {
        if x == &self.rows[0].len() - 1 {
            return true;
        }

        let row = &self.rows[y];
        let tree = row[x];

        for pos in x + 1..row.len() {
            if row[pos] >= tree {
                return false;
            }
        }

        true
    }

    fn visible_from_top(&self, x: usize, y: usize) -> bool {
        if y == 0 {
            return true;
        }

        let tree = &self.rows[y][x];
        for row in 0..y {
            if &self.rows[row][x] >= tree {
                return false;
            }
        }
        true
    }
    fn visible_from_bot(&self, x: usize, y: usize) -> bool {
        if y == &self.rows.len() - 1 {
            return true;
        }

        let tree = &self.rows[y][x];
        for row in y + 1..self.rows.len() {
            if &self.rows[row][x] >= tree {
                return false;
            }
        }
        true
    }

    fn check_if_in_outer_ring(&self, x: usize, y: usize) -> bool {
        if x == 0 || x == self.rows[0].len() - 1 {
            return true;
        }
        if y == 0 || y == self.rows.len() - 1 {
            return true;
        }
        false
    }
    fn check_all_directions(&self, x: usize, y: usize) -> bool {
        // Shortcut
        if self.check_if_in_outer_ring(x, y) {
            return true;
        }

        self.visible_from_left_of_pos(x, y)
            || self.visible_from_right_of_pos(x, y)
            || self.visible_from_top(x, y)
            || self.visible_from_bot(x, y)
    }

    fn count_visible_trees(&self) -> usize {
        let mut count = 0;
        for (y, row) in self.rows.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                if self.check_all_directions(x, y) {
                    count += 1;
                }
            }
        }

        count
    }

    fn get_scenic_up(&self, x: usize, y: usize) -> usize {
        // If a tree is right on the edge, at least one of its viewing distances will be zero.
        if y == 0 {
            return 0;
        }
        let mut scenic_score = 1;
        let tree = &self.rows[y][x];
        for row in (1..y).rev() {
            if &self.rows[row][x] >= tree {
                return scenic_score;
            }
            scenic_score += 1;
        }
        scenic_score
    }
    fn get_scenic_left(&self, x: usize, y: usize) -> usize {
        if x == 0 {
            return 0;
        }
        let mut scenic_score = 1;
        let tree = self.rows[y][x];
        let row = &self.rows[y];
        for pos in (1..x).rev() {
            if row[pos] >= tree {
                return scenic_score;
            }
            scenic_score += 1;
        }
        scenic_score
    }
    fn get_scenic_right(&self, x: usize, y: usize) -> usize {
        if x == self.rows[0].len() - 1 {
            return 0;
        }
        let mut scenic_score = 1;
        let tree = self.rows[y][x];
        let row = &self.rows[y];
        for pos in x + 1..self.rows[0].len() - 1 {
            if row[pos] >= tree {
                return scenic_score;
            }
            scenic_score += 1;
        }
        scenic_score
    }
    fn get_scenic_down(&self, x: usize, y: usize) -> usize {
        if y == self.rows.len() - 1 {
            return 0;
        }
        let mut scenic_score = 1;
        let tree = &self.rows[y][x];
        for row in y + 1..self.rows.len() - 1 {
            if &self.rows[row][x] >= tree {
                return scenic_score;
            }
            scenic_score += 1;
        }
        scenic_score
    }

    fn get_scenic_score(&self, x: usize, y: usize) -> usize {
        let u = self.get_scenic_up(x, y);
        let d = self.get_scenic_down(x, y);
        let l = self.get_scenic_left(x, y);
        let r = self.get_scenic_right(x, y);
        if u == 0 || d == 0 || l == 0 || r == 0 {
            return 0;
        }

        u * d * l * r
    }

    fn get_highest_scenic_score(&self) -> usize {
        let mut highest = 0;
        for (y, row) in self.rows.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                let score = self.get_scenic_score(x, y);
                if score > highest {
                    highest = score;
                }
            }
        }
        highest
    }
}

fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.split("")
                .filter(|c| !c.is_empty())
                .map(Tree::from)
                .collect::<Vec<Tree>>()
        })
        .collect::<Vec<Vec<Tree>>>();

    let forest = Forest::new(input);
    println!(
        "Number of visible trees in the forest is: {}",
        forest.count_visible_trees()
    );

    // Part 2
    println!(
        "Highest scenic score is {}",
        forest.get_highest_scenic_score()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenic() {
        let forest = Forest::new(vec![
            vec![Tree(3), Tree(0), Tree(3), Tree(7), Tree(3)],
            vec![Tree(2), Tree(5), Tree(5), Tree(1), Tree(2)],
            vec![Tree(6), Tree(5), Tree(3), Tree(3), Tree(2)],
            vec![Tree(3), Tree(3), Tree(5), Tree(4), Tree(9)],
            vec![Tree(3), Tree(5), Tree(3), Tree(9), Tree(0)],
        ]);

        assert_eq!(1, forest.get_scenic_up(2, 1));
        assert_eq!(1, forest.get_scenic_left(2, 1));
        assert_eq!(2, forest.get_scenic_right(2, 1));
        assert_eq!(2, forest.get_scenic_down(2, 1));

        assert_eq!(2, forest.get_scenic_up(2, 3));
        assert_eq!(2, forest.get_scenic_left(2, 3));
        assert_eq!(1, forest.get_scenic_down(2, 3));
        assert_eq!(2, forest.get_scenic_right(2, 3));
    }
    #[test]
    fn look_up_on_forest() {
        let forest = Forest::new(vec![
            vec![Tree(3), Tree(0), Tree(3), Tree(7), Tree(3)],
            vec![Tree(2), Tree(5), Tree(5), Tree(1), Tree(2)],
            vec![Tree(6), Tree(5), Tree(3), Tree(3), Tree(2)],
            vec![Tree(3), Tree(3), Tree(5), Tree(4), Tree(9)],
            vec![Tree(3), Tree(5), Tree(3), Tree(9), Tree(0)],
        ]);

        assert!(forest.visible_from_bot(0, 4));
        assert!(forest.visible_from_bot(0, 2));
        assert!(!forest.visible_from_bot(1, 1));
        assert!(!forest.visible_from_bot(1, 3));
        assert!(!forest.visible_from_bot(4, 2));
        assert!(forest.visible_from_bot(4, 3));

        assert_eq!(21, forest.count_visible_trees());
    }

    #[test]
    fn look_down_on_forest() {
        let forest = Forest::new(vec![
            vec![Tree(3), Tree(0), Tree(3), Tree(7), Tree(3)],
            vec![Tree(2), Tree(5), Tree(5), Tree(1), Tree(2)],
            vec![Tree(6), Tree(5), Tree(3), Tree(3), Tree(2)],
            vec![Tree(3), Tree(3), Tree(5), Tree(4), Tree(9)],
            vec![Tree(3), Tree(5), Tree(3), Tree(9), Tree(0)],
        ]);

        // first row, should be visible
        assert!(forest.visible_from_top(0, 0));
        assert!(!forest.visible_from_top(0, 1)); // 3 > 2
        assert!(!forest.visible_from_top(1, 2)); // 5 is ahead of this one
        assert!(forest.visible_from_top(3, 4)); // 9 is highest
        assert!(!forest.visible_from_top(4, 4)); // 0
    }

    #[test]
    fn look_left_on_forest() {
        let forest = Forest::new(vec![
            vec![Tree(3), Tree(0), Tree(3), Tree(7), Tree(3)],
            vec![Tree(2), Tree(5), Tree(5), Tree(1), Tree(2)],
            vec![Tree(6), Tree(5), Tree(3), Tree(3), Tree(2)],
            vec![Tree(3), Tree(3), Tree(5), Tree(4), Tree(9)],
            vec![Tree(3), Tree(5), Tree(3), Tree(9), Tree(0)],
        ]);

        // for row in &forest.rows {i
        // First tree is always visible
        assert!(forest.visible_from_left_of_pos(0, 0));
        // 0 is less than 3
        assert!(!forest.visible_from_left_of_pos(1, 0));

        // 3,3 is less than 5
        assert!(forest.visible_from_left_of_pos(2, 3));
        // outer right, not visible from left but is farthest right so outer ring fuse should trigger
        //assert!(forest.visible_from_left_of_pos(4, 4));
        // dbg!(forest);
        assert!(!forest.visible_from_left_of_pos(2, 4));
    }

    #[test]
    fn look_right_on_forest() {
        let forest = Forest::new(vec![
            vec![Tree(3), Tree(0), Tree(3), Tree(7), Tree(3)],
            vec![Tree(2), Tree(5), Tree(5), Tree(1), Tree(2)],
            vec![Tree(6), Tree(5), Tree(3), Tree(3), Tree(2)],
            vec![Tree(3), Tree(3), Tree(5), Tree(4), Tree(9)],
            vec![Tree(3), Tree(5), Tree(3), Tree(9), Tree(0)],
        ]);

        // 7 > 3
        assert!(forest.visible_from_right_of_pos(3, 0));
        // 2 < 5
        assert!(!forest.visible_from_right_of_pos(0, 1));
        // outer, should be visible
        assert!(forest.visible_from_right_of_pos(4, 2));
        // 5 < 9
        assert!(!forest.visible_from_right_of_pos(2, 3));
        // 9 > 0
        assert!(forest.visible_from_right_of_pos(3, 4));
    }
}
