/// Possible hands you can throw in a match
#[derive(Copy, Clone, Debug)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

/// Input parsing
impl From<&str> for Throw {
    fn from(from: &str) -> Self {
        match from {
            "A" => Throw::Rock,
            "B" => Throw::Paper,
            "C" => Throw::Scissors,
            "X" => Throw::Rock,
            "Y" => Throw::Paper,
            "Z" => Throw::Scissors,
            _ => panic!("Impossible choice: {}", &from),
        }
    }
}

impl Throw {
    /// Generate an `Outcome` comparing against another `Throw`
    fn wins_over(&self, other: &Throw) -> Outcome {
        match (&self, other) {
            (Throw::Rock, Throw::Paper) => Outcome::Lose,
            (Throw::Rock, Throw::Scissors) => Outcome::Win,
            (Throw::Paper, Throw::Rock) => Outcome::Win,
            (Throw::Paper, Throw::Scissors) => Outcome::Lose,
            (Throw::Scissors, Throw::Rock) => Outcome::Lose,
            (Throw::Scissors, Throw::Paper) => Outcome::Win,
            _ => Outcome::Draw,
        }
    }

    /// Generate a `Throw` based on what we want to achieve via a `StrategyChoice`
    fn get_desired(&self, desired: StrategyChoice) -> Throw {
        match self {
            Throw::Rock => match desired {
                StrategyChoice::Win => Throw::Paper,
                StrategyChoice::Lose => Throw::Scissors,
                _ => Throw::Rock,
            },
            Throw::Paper => match desired {
                StrategyChoice::Win => Throw::Scissors,
                StrategyChoice::Lose => Throw::Rock,
                _ => Throw::Paper,
            },
            Throw::Scissors => match desired {
                StrategyChoice::Win => Throw::Rock,
                StrategyChoice::Lose => Throw::Paper,
                _ => Throw::Scissors,
            },
        }
    }
}

/// Possible outcomes of a match
#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

/// A strategy for a match
#[derive(Debug)]
struct MatchStrategy {
    opponent: Throw,
    me: StrategyChoice,
}

impl MatchStrategy {
    /// Look at what the opponent is playing and generate a `Throw` based on desired outcome
    fn get_choice_based_on_strategy(&self) -> Throw {
        self.opponent.get_desired(self.me)
    }
}

/// Input parsing
impl From<&str> for MatchStrategy {
    fn from(from: &str) -> Self {
        let mut parts = from.split_whitespace();
        let opponent = Throw::from(parts.next().unwrap());
        let me = StrategyChoice::from(parts.next().unwrap());
        Self { opponent, me }
    }
}

/// How we would like the game to end
#[derive(Copy, Clone, Debug)]
enum StrategyChoice {
    /// X; we would like to lose
    Lose,
    /// Y; we would like to end in a draw
    Draw,
    /// Z; we would like ot win
    Win,
}

/// input parsing
impl From<&str> for StrategyChoice {
    fn from(from: &str) -> Self {
        match from {
            "X" => StrategyChoice::Lose,
            "Y" => StrategyChoice::Draw,
            "Z" => StrategyChoice::Win,
            _ => panic!("Impossible choice: {}", &from),
        }
    }
}

/// A pair of hand-`Throw`s makes up a round/match
#[derive(Debug)]
struct MatchPair {
    /// What my adversary is playing
    opponent: Throw,
    /// What I am playing
    me: Throw,
}

impl MatchPair {
    /// The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)i
    /// Score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
    fn get_score(&self) -> usize {
        let choice_score = match self.me {
            Throw::Rock => 1,
            Throw::Paper => 2,
            Throw::Scissors => 3,
        };

        let match_score = match self.me.wins_over(&self.opponent) {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        };

        choice_score + match_score
    }
}

/// If we have an opponents hand and a strategy we would want to execute, we can create a `MatchPair`
impl From<MatchStrategy> for MatchPair {
    fn from(from: MatchStrategy) -> Self {
        Self {
            opponent: from.opponent,
            me: from.get_choice_based_on_strategy(),
        }
    }
}

/// Input parsing
impl From<&str> for MatchPair {
    fn from(from: &str) -> Self {
        let parts: Vec<Throw> = from
            .split_whitespace()
            .into_iter()
            .map(Throw::from)
            .collect();
        Self {
            opponent: parts[0],
            me: parts[1],
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut all_games: Vec<MatchPair> = Vec::new();
    let mut new_strategy: Vec<MatchPair> = Vec::new();
    for line in input.split('\n') {
        if !line.is_empty() {
            all_games.push(MatchPair::from(line));
            new_strategy.push(MatchPair::from(MatchStrategy::from(line)));
        }
    }
    let total_score: usize = all_games.iter().map(|x| x.get_score()).sum();
    let new_score: usize = new_strategy.iter().map(|x| x.get_score()).sum();
    println!("Total score for first part: {}", total_score);
    println!("Total score using second part: {}", new_score);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn am_i_choosing_correct_strategy_for_a_y() {
        let matchstr = "A Y";
        let matchstrat = MatchStrategy::from(matchstr);
        let game = MatchPair::from(matchstrat);

        assert_eq!(game.get_score(), 4);
    }

    #[test]
    fn am_i_choosing_correct_strategy_for_b_x() {
        let matchstr = "B X";
        let matchstrat = MatchStrategy::from(matchstr);
        let game = MatchPair::from(matchstrat);

        assert_eq!(game.get_score(), 1);
    }

    #[test]
    fn am_i_choosing_correct_strategy_for_c_z() {
        let matchstr = "C Z";
        let matchstrat = MatchStrategy::from(matchstr);
        let game = MatchPair::from(matchstrat);

        assert_eq!(game.get_score(), 7);
    }

    #[test]
    fn am_i_winning_with_rock() {
        let me = Throw::Rock;
        let opponent = Throw::Scissors;
        let game = MatchPair { opponent, me };
        // I should win
        assert_eq!(Outcome::Win, me.wins_over(&opponent));

        assert_eq!(game.get_score(), 7);
    }

    #[test]
    fn am_i_losing_with_rock() {
        let me = Throw::Rock;
        let opponent = Throw::Paper;
        let game = MatchPair { opponent, me };
        // I should lose
        assert_eq!(Outcome::Lose, me.wins_over(&opponent));

        // I get 1 since i chose rock
        assert_eq!(game.get_score(), 1);
    }

    #[test]
    fn am_i_drawing_with_paper() {
        let me = Throw::Paper;
        let opponent = Throw::Paper;
        let game = MatchPair { opponent, me };
        // I should lose
        assert_eq!(Outcome::Draw, me.wins_over(&opponent));

        // I get 5 since i chose paper = 2, and draw = 3
        assert_eq!(game.get_score(), 5);
    }

    #[test]
    fn rock_wins_over_scissors() {
        let opponent = Throw::Rock;
        assert_eq!(Outcome::Win, opponent.wins_over(&Throw::Scissors));
    }
    #[test]
    fn rock_draws_against_rock() {
        let opponent = Throw::Rock;
        assert_eq!(Outcome::Draw, opponent.wins_over(&Throw::Rock));
    }

    #[test]
    fn rock_loses_against_paper() {
        let opponent = Throw::Rock;
        assert_eq!(Outcome::Lose, opponent.wins_over(&Throw::Paper));
    }

    #[test]
    fn scissors_wins_over_paper() {
        let opponent = Throw::Scissors;
        assert_eq!(Outcome::Win, opponent.wins_over(&Throw::Paper));
    }
    #[test]
    fn scissors_draws_against_scissors() {
        let opponent = Throw::Scissors;
        assert_eq!(Outcome::Draw, opponent.wins_over(&Throw::Scissors));
    }

    #[test]
    fn scissors_loses_against_rock() {
        let opponent = Throw::Scissors;
        assert_eq!(Outcome::Lose, opponent.wins_over(&Throw::Rock));
    }

    #[test]
    fn paper_wins_over_rock() {
        let opponent = Throw::Paper;
        assert_eq!(Outcome::Win, opponent.wins_over(&Throw::Rock));
    }
    #[test]
    fn paper_draws_against_paper() {
        let opponent = Throw::Paper;
        assert_eq!(Outcome::Draw, opponent.wins_over(&Throw::Paper));
    }

    #[test]
    fn paper_loses_against_sciccors() {
        let opponent = Throw::Paper;
        assert_eq!(Outcome::Lose, opponent.wins_over(&Throw::Scissors));
    }
}
