#[cfg(not(test))]
use rand::Rng;
use std::fmt;

#[derive(Copy, Clone)]
pub enum GameDifficulty {
    Basic,
    Medium,
    Hard,
    Mastery,
}

#[derive(Copy, Clone)]
pub enum GameType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    FractionAddition,
    FractionSubtraction,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Answer {
    Integer(i32),
    Fraction { numerator: i32, denominator: i32 },
}
// check this
impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Answer::Integer(n) => write!(f, "{}", n),
            Answer::Fraction {
                numerator,
                denominator,
            } => write!(f, "{}/{}", numerator, denominator),
        }
    }
}

impl Answer {
    pub fn check(&self, user_answer: &Answer) -> bool {
        match (self, user_answer) {
            (Answer::Integer(a), Answer::Integer(b)) => a == b,
            (
                Answer::Fraction {
                    numerator: n1,
                    denominator: d1,
                },
                Answer::Fraction {
                    numerator: n2,
                    denominator: d2,
                },
            ) => n1 * d2 == n2 * d1,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Fraction {
    pub numerator: i32,
    pub denominator: i32,
}

impl Fraction {
    fn new(numerator: i32, denominator: i32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    fn to_string(&self) -> String {
        format!("{}/{}", self.numerator, self.denominator)
    }
}

#[derive(PartialEq)]
pub enum GameState {
    NotStarted,
    Playing,
    RoundComplete,
}

pub struct Game {
    pub score: i32,
    pub current_type: GameType,
    pub current_difficulty: GameDifficulty,
    pub current_problem: Option<Problem>,
    pub problems_per_round: i32,
    pub current_round_completed_problems: i32,
    pub game_state: GameState,
    #[cfg(test)]
    rng: fn(i32, i32) -> (i32, i32),
}

impl Default for Game {
    fn default() -> Self {
        Self {
            score: 0,
            current_type: GameType::Addition,
            current_difficulty: GameDifficulty::Basic,
            current_problem: None,
            problems_per_round: 10,
            current_round_completed_problems: 0,
            game_state: GameState::NotStarted,
            #[cfg(test)]
            rng: |min, max| (min, max),
        }
    }
}

// Game is a session of the game. the user will select a game type and then play the game 10 times.
impl Game {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(test)]
    pub fn with_mock_rng(rng: fn(i32, i32) -> (i32, i32)) -> Self {
        Self { 
            score: 0,
            current_type: GameType::Addition,
            current_difficulty: GameDifficulty::Basic,
            current_problem: None,
            problems_per_round: 10,
            current_round_completed_problems: 0,
            game_state: GameState::NotStarted,
            rng 
        }
    }

    pub fn generate_problem(
        &mut self
    ) -> Problem {
        let (min, max) = self.generate_range(&self.current_difficulty);

        #[cfg(test)]
        let (a, b) = (self.rng)(min, max);

        #[cfg(not(test))]
        let (a, b) = {
            let mut rng = rand::thread_rng();
            (rng.gen_range(min..=max), rng.gen_range(min..=max))
        };

        // math problems types to generate custom problems
        let problem = match &self.current_type {
            GameType::Addition => {
                Problem::new(format!("{} + {}", a, b), Answer::Integer(a + b), a, b)
            }
            GameType::Subtraction => {
                Problem::new(format!("{} - {}", a, b), Answer::Integer(a - b), a, b)
            }
            GameType::Multiplication => {
                Problem::new(format!("{} * {}", a, b), Answer::Integer(a * b), a, b)
            }
            GameType::Division => {
                let dividend = a * b;
                Problem::new(
                    format!("{} / {}", dividend, b),
                    Answer::Integer(a),
                    dividend,
                    b,
                )
            }
            GameType::FractionAddition => {
                // Get second pair of numbers for second fraction
                #[cfg(test)]
                let (c, d) = (self.rng)(min, max);

                #[cfg(not(test))]
                let (c, d) = {
                    let mut rng = rand::thread_rng();
                    (rng.gen_range(min..=max), rng.gen_range(min..=max))
                };

                Problem::new(
                    format!("{}/{} + {}/{}", a, b, c, d),
                    Answer::Fraction {
                        numerator: a * d + b * c,
                        denominator: b * d,
                    },
                    a,
                    b,
                )
            }
            GameType::FractionSubtraction => {
                #[cfg(test)]
                let (c, d) = (self.rng)(min, max);

                #[cfg(not(test))]
                let (c, d) = {
                    let mut rng = rand::thread_rng();
                    (rng.gen_range(min..=max), rng.gen_range(min..=max))
                };

                Problem::new(
                    format!("{}/{} - {}/{}", a, b, c, d),
                    Answer::Fraction {
                        numerator: a * d - b * c,
                        denominator: b * d,
                    },
                    a,
                    b,
                )
            }
        };

        // se the current problem to the generated problem
        self.current_problem = Some(problem.clone());

        problem
    }

    fn generate_range(&self, difficulty: &GameDifficulty) -> (i32, i32) {
        match difficulty {
            GameDifficulty::Basic => (1, 9),
            GameDifficulty::Medium => (10, 99),
            GameDifficulty::Hard => (100, 999),
            GameDifficulty::Mastery => (1000, 10000),
        }
    }

    pub fn check_answer(&mut self, user_answer: &Answer) -> bool {
        if let Some(problem) = &mut self.current_problem {
            let correct = problem.check_answer(user_answer);
            if correct {
                self.score += 1;
                self.current_round_completed_problems += 1;
            }
            correct
        } else {
            false
        }
    }

    pub fn is_round_completed(&self) -> bool {
        self.current_round_completed_problems == self.problems_per_round
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Problem {
    pub problem: String,
    pub answer: Answer,
    pub operand1: i32,
    pub operand2: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub solved_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Problem {
    pub fn new(problem: String, answer: Answer, operand1: i32, operand2: i32) -> Self {
        Self {
            problem,
            answer,
            operand1,
            operand2,
            created_at: chrono::Utc::now(),
            solved_at: None,
        }
    }

    pub fn check_answer(&mut self, user_answer: &Answer) -> bool {
        let correct = self.answer.check(user_answer);
        if correct {
            self.solved_at = Some(chrono::Utc::now());
        }
        correct
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_range() {
        let game = Game::new();
        assert_eq!(game.generate_range(&GameDifficulty::Basic), (1, 9));
        assert_eq!(game.generate_range(&GameDifficulty::Medium), (10, 99));
        assert_eq!(game.generate_range(&GameDifficulty::Hard), (100, 999));
        assert_eq!(game.generate_range(&GameDifficulty::Mastery), (1000, 10000));
    }

    #[test]
    fn test_range_in_generate_problem_addition() {
        // Addition basic
        let mut game = Game::new();
        let problem = game.generate_problem();
        assert!(
            problem.operand1 >= 1 && problem.operand1 < 10,
            "Addition basic Operand1: {}",
            problem.operand1
        );
        assert!(
            problem.operand2 >= 1 && problem.operand2 < 10,
            "Addition basic Operand2: {}",
            problem.operand2
        );

        // set level to medium
        game.current_difficulty = GameDifficulty::Medium;
        let problem = game.generate_problem();
        assert!(
            problem.operand1 >= 10 && problem.operand1 < 100,
            "Addition medium Operand1: {}",
            problem.operand1
        );
        assert!(
            problem.operand2 >= 10 && problem.operand2 < 100,
            "Addition medium Operand2: {}",
            problem.operand2
        );

        // set level to hard
        game.current_difficulty = GameDifficulty::Hard;
        let problem = game.generate_problem();
        assert!(
            problem.operand1 >= 100 && problem.operand1 < 1000,
            "Addition hard Operand1: {}",
            problem.operand1
        );
        assert!(
            problem.operand2 >= 100 && problem.operand2 < 1000,
            "Addition hard Operand2: {}",
            problem.operand2
        );

        // set level to mastery
        game.current_difficulty = GameDifficulty::Mastery;
        let problem = game.generate_problem();
        assert!(
            problem.operand1 >= 1000 && problem.operand1 <= 10000,
            "Addition mastery Operand1: {}",
            problem.operand1
        );
        assert!(
            problem.operand2 >= 1000 && problem.operand2 <= 10000,
            "Addition mastery Operand2: {}",
            problem.operand2
        );
    }

    #[test]
    fn test_range_in_generate_problem_substraction() {
        // substraction basic
        let mut game = Game::new();
        // set game to substraction
        game.current_type = GameType::Subtraction;
        let problem = game.generate_problem();
        assert!(
            problem.operand1 >= 1 && problem.operand1 < 10,
            "Substraction basic Operand1: {}",
            problem.operand1
        );
        assert!(
            problem.operand2 >= 1 && problem.operand2 < 10,
            "Substraction basic Operand2: {}",
            problem.operand2
        );

        // set game to substraction
        game.current_type = GameType::Subtraction;
        // set level to medium
        game.current_difficulty = GameDifficulty::Medium;
        let problem = game.generate_problem();
        assert!(
            problem.operand1 >= 10 && problem.operand1 < 100,
            "Substraction medium Operand1: {}",
            problem.operand1
        );
        assert!(
            problem.operand2 >= 10 && problem.operand2 < 100,
            "Substraction medium Operand2: {}",
            problem.operand2
        );

        // set game to substraction
        game.current_type = GameType::Subtraction;
        // set level to hard
        game.current_difficulty = GameDifficulty::Hard;
        let problem = game.generate_problem();
        assert!(
            problem.operand1 >= 100 && problem.operand1 < 1000,
            "Substraction hard Operand1: {}",
            problem.operand1
        );
        assert!(
            problem.operand2 >= 100 && problem.operand2 < 1000,
            "Substraction hard Operand2: {}",
            problem.operand2
        );

        // set game to substraction
        game.current_type = GameType::Subtraction;
        // set level to mastery
        game.current_difficulty = GameDifficulty::Mastery;
        let problem = game.generate_problem();
        assert!(
            problem.operand1 >= 1000 && problem.operand1 <= 10000,
            "Substraction mastery Operand1: {}",
            problem.operand1
        );
        assert!(
            problem.operand2 >= 1000 && problem.operand2 <= 10000,
            "Substraction mastery Operand2: {}",
            problem.operand2
        );
    }

    #[test]
    fn test_range_in_generate_problem_multiplication() {
        // multiplication basic
        let mut game = Game::new();
        // set game to multiplication
        game.current_type = GameType::Multiplication;
        let problem = game.generate_problem();
        assert!(
            problem.operand1 >= 1 && problem.operand1 < 10,
            "Multiplication basic Operand1: {}",
            problem.operand1
        );
        assert!(
            problem.operand2 >= 1 && problem.operand2 < 10,
            "Multiplication basic Operand2: {}",
            problem.operand2
        );

        // set game to multiplication
        game.current_type = GameType::Multiplication;
        // set level to medium
        game.current_difficulty = GameDifficulty::Medium;
        let problem = game.generate_problem();
        assert!(
            problem.operand1 >= 10 && problem.operand1 < 100,
            "Multiplication medium Operand1: {}",
            problem.operand1
        );
        assert!(
            problem.operand2 >= 10 && problem.operand2 < 100,
            "Multiplication medium Operand2: {}",
            problem.operand2
        );

        // set game to multiplication
        game.current_type = GameType::Multiplication;
        // set level to hard
        game.current_difficulty = GameDifficulty::Hard;
        let problem = game.generate_problem();
        assert!(
            problem.operand1 >= 100 && problem.operand1 < 1000,
            "Multiplication hard Operand1: {}",
            problem.operand1
        );
        assert!(
            problem.operand2 >= 100 && problem.operand2 < 1000,
            "Multiplication hard Operand2: {}",
            problem.operand2
        );

        // set game to multiplication
        game.current_type = GameType::Multiplication;
        // set level to mastery
        game.current_difficulty = GameDifficulty::Mastery;
        let problem = game.generate_problem();
        assert!(
            problem.operand1 >= 1000 && problem.operand1 <= 10000,
            "Multiplication mastery Operand1: {}",
            problem.operand1
        );
        assert!(
            problem.operand2 >= 1000 && problem.operand2 <= 10000,
            "Multiplication mastery Operand2: {}",
            problem.operand2
        );
    }

    #[test]
    fn test_range_in_generate_problem_division() {
        // For division, the dividend (operand1) will be larger than the range
        // because it's the product of two numbers in the range

        // division basic (range 1-10)
        let mut game = Game::new();
        // set game to division
        game.current_type = GameType::Division;
        let problem = game.generate_problem();
        assert!(
            problem.operand2 >= 1 && problem.operand2 <= 10,
            "Division basic divisor: {}",
            problem.operand2
        );
        assert!(
            problem.answer >= Answer::Integer(1) && problem.answer <= Answer::Integer(10),
            "Division basic quotient: {}",
            problem.answer
        );

        // division medium (range 10-100)
        // set level to medium
        game.current_difficulty = GameDifficulty::Medium;
        let problem = game.generate_problem();
        assert!(
            problem.operand2 >= 10 && problem.operand2 <= 100,
            "Division medium divisor: {}",
            problem.operand2
        );
        assert!(
            problem.answer >= Answer::Integer(10) && problem.answer <= Answer::Integer(100),
            "Division medium quotient: {}. question {:?}",
            problem.answer,
            problem
        );

        // division hard (range 100-1000)
        // set level to hard
        game.current_difficulty = GameDifficulty::Hard;
        let problem = game.generate_problem();
        assert!(
            problem.operand2 >= 100 && problem.operand2 <= 1000,
            "Division hard divisor: {}",
            problem.operand2
        );
        assert!(
            problem.answer >= Answer::Integer(100) && problem.answer <= Answer::Integer(1000),
            "Division hard quotient: {}",
            problem.answer
        );

        // division mastery (range 1000-10000)
        // set level to mastery
        game.current_difficulty = GameDifficulty::Mastery;
        let problem = game.generate_problem();
        assert!(
            problem.operand2 >= 1000 && problem.operand2 <= 10000,
            "Division mastery divisor: {}",
            problem.operand2
        );
        assert!(
            problem.answer >= Answer::Integer(1000) && problem.answer <= Answer::Integer(10000),
            "Division mastery quotient: {}",
            problem.answer
        );
    }

    #[test]
    fn test_addition_basic_problem() {
        let mut game = Game::with_mock_rng(|_,_| (2, 3));
        let mut problem = game.generate_problem();

        assert_eq!(problem.problem, "2 + 3");
        assert_eq!(problem.check_answer(&Answer::Integer(5)), true);
        assert_eq!(problem.check_answer(&Answer::Integer(6)), false);
    }

    #[test]
    fn test_addition_medium_problem() {
        let mut game = Game::with_mock_rng(|_,_| (5, 6));
        game.current_difficulty = GameDifficulty::Medium;
        let mut problem = game.generate_problem();

        assert_eq!(problem.problem, "5 + 6");
        assert_eq!(problem.check_answer(&Answer::Integer(11)), true);
        assert_eq!(problem.check_answer(&Answer::Integer(12)), false);
    }
}
