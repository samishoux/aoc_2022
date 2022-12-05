use std::str::FromStr;

#[derive(Clone)]
enum GameInput {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone)]
enum GameOutcome {
    Win,
    Loss,
    Draw,
}

impl FromStr for GameOutcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameOutcome::Loss),
            "Y" => Ok(GameOutcome::Draw),
            "Z" => Ok(GameOutcome::Win),
            _ => Err(()),
        }
    }
}

impl GameOutcome {
    fn get_point(&self) -> i32 {
        return match self {
            GameOutcome::Win => 6,
            GameOutcome::Loss => 0,
            GameOutcome::Draw => 3,
        };
    }

    fn get_game_points_part1(s: &str) -> i32 {
        let players: Vec<&str> = s.split(" ").collect::<Vec<&str>>();
        let opponent = GameInput::from_str(players[0]).unwrap();
        let first_player = GameInput::from_str(players[1]).unwrap();

        return first_player.get_game_points(&opponent);
    }

    fn get_game_points_part2(s: &str) -> i32 {
        let inputs: Vec<&str> = s.split(" ").collect::<Vec<&str>>();
        let opponent = GameInput::from_str(inputs[0]).unwrap();
        let game_outcome = GameOutcome::from_str(inputs[1]).unwrap();
        let first_player = game_outcome.get_input_needed(&opponent);

        return first_player.get_game_points(&opponent);
    }

    fn get_input_needed(&self, opponent: &GameInput) -> GameInput {
        return match self {
            GameOutcome::Win => match opponent {
                GameInput::Rock => GameInput::Paper,
                GameInput::Paper => GameInput::Scissors,
                GameInput::Scissors => GameInput::Rock,
            },
            GameOutcome::Loss => match opponent {
                GameInput::Rock => GameInput::Scissors,
                GameInput::Paper => GameInput::Rock,
                GameInput::Scissors => GameInput::Paper,
            },
            GameOutcome::Draw => opponent.clone(),
        };
    }
}
impl GameInput {
    fn get_point(&self) -> i32 {
        return match self {
            GameInput::Rock => 1,
            GameInput::Paper => 2,
            GameInput::Scissors => 3,
        };
    }

    fn get_game_points(&self, opponent: &GameInput) -> i32 {
        let game_result = self.get_game_outcome(opponent);
        return game_result.get_point() + self.get_point();
    }

    fn get_game_outcome(&self, opponent: &GameInput) -> GameOutcome {
        return match self {
            GameInput::Rock => match opponent {
                GameInput::Rock => GameOutcome::Draw,
                GameInput::Paper => GameOutcome::Loss,
                GameInput::Scissors => GameOutcome::Win,
            },
            GameInput::Paper => match opponent {
                GameInput::Rock => GameOutcome::Win,
                GameInput::Paper => GameOutcome::Draw,
                GameInput::Scissors => GameOutcome::Loss,
            },
            GameInput::Scissors => match opponent {
                GameInput::Rock => GameOutcome::Loss,
                GameInput::Paper => GameOutcome::Win,
                GameInput::Scissors => GameOutcome::Draw,
            },
        };
    }
}

impl FromStr for GameInput {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(GameInput::Rock),
            "B" => Ok(GameInput::Paper),
            "C" => Ok(GameInput::Scissors),
            "X" => Ok(GameInput::Rock),
            "Y" => Ok(GameInput::Paper),
            "Z" => Ok(GameInput::Scissors),
            _ => Err(()),
        }
    }
}

fn main() {
    let input = include_str!("./input2.prod");
    let all_game_results = input
        .split("\n")
        .map(|game| GameOutcome::get_game_points_part1(game))
        .sum::<i32>();

    println!("Part 1: {:?}", all_game_results);

    let all_game_results = input
        .split("\n")
        .map(|game| GameOutcome::get_game_points_part2(game))
        .sum::<i32>();

    println!("Part 2: {:?}", all_game_results);
}

#[test]
fn test_rock_game_outcome() {
    let input = GameInput::Rock;

    let opponent = GameInput::Paper;
    let result = input.get_game_outcome(&opponent);
    assert!(matches!(result, GameOutcome::Loss));

    let opponent = GameInput::Rock;
    let result = input.get_game_outcome(&opponent);
    assert!(matches!(result, GameOutcome::Draw));

    let opponent = GameInput::Scissors;
    let result = input.get_game_outcome(&opponent);
    assert!(matches!(result, GameOutcome::Win));
}

#[test]
fn test_scissors_game_outcome() {
    let input = GameInput::Scissors;

    let opponent = GameInput::Paper;
    let result = input.get_game_outcome(&opponent);
    assert!(matches!(result, GameOutcome::Win));

    let opponent = GameInput::Rock;
    let result = input.get_game_outcome(&opponent);
    assert!(matches!(result, GameOutcome::Loss));

    let opponent = GameInput::Scissors;
    let result = input.get_game_outcome(&opponent);
    assert!(matches!(result, GameOutcome::Draw));
}

#[test]
fn test_paper_game_outcome() {
    let input = GameInput::Paper;

    let opponent = GameInput::Paper;
    let result = input.get_game_outcome(&opponent);
    assert!(matches!(result, GameOutcome::Draw));

    let opponent = GameInput::Rock;
    let result = input.get_game_outcome(&opponent);
    assert!(matches!(result, GameOutcome::Win));

    let opponent = GameInput::Scissors;
    let result = input.get_game_outcome(&opponent);
    assert!(matches!(result, GameOutcome::Loss));
}

#[test]
fn test_get_game_points() {
    let input = GameInput::Paper;
    let opponent = GameInput::Rock;
    let result = input.get_game_points(&opponent);
    assert!(result == 8);

    let input = GameInput::Rock;
    let opponent = GameInput::Paper;
    let result = input.get_game_points(&opponent);
    assert!(result == 1);

    let input = GameInput::Scissors;
    let opponent = GameInput::Scissors;
    let result = input.get_game_points(&opponent);
    assert!(result == 6);
}

#[test]
fn test_part1_get_game_points_with_input_str() {
    let input = include_str!("./input2.dev");
    let all_game_results = input
        .split("\n")
        .map(|game| GameOutcome::get_game_points_part1(game))
        .sum::<i32>();

    assert!(all_game_results == 15)
}

#[test]
fn test_loss_get_get_input_needed() {
    let outcome = GameOutcome::Loss;

    let opponent = GameInput::Rock;
    let result = outcome.get_input_needed(&opponent);
    assert!(matches!(result, GameInput::Scissors));

    let opponent = GameInput::Paper;
    let result = outcome.get_input_needed(&opponent);
    assert!(matches!(result, GameInput::Rock));

    let opponent = GameInput::Scissors;
    let result = outcome.get_input_needed(&opponent);
    assert!(matches!(result, GameInput::Paper));
}

#[test]
fn test_win_get_get_input_needed() {
    let outcome = GameOutcome::Win;

    let opponent = GameInput::Rock;
    let result = outcome.get_input_needed(&opponent);
    assert!(matches!(result, GameInput::Paper));

    let opponent = GameInput::Paper;
    let result = outcome.get_input_needed(&opponent);
    assert!(matches!(result, GameInput::Scissors));

    let opponent = GameInput::Scissors;
    let result = outcome.get_input_needed(&opponent);
    assert!(matches!(result, GameInput::Rock));
}

#[test]
fn test_draw_get_get_input_needed() {
    let outcome = GameOutcome::Draw;

    let opponent = GameInput::Rock;
    let result = outcome.get_input_needed(&opponent);
    assert!(matches!(result, GameInput::Rock));

    let opponent = GameInput::Paper;
    let result = outcome.get_input_needed(&opponent);
    assert!(matches!(result, GameInput::Paper));

    let opponent = GameInput::Scissors;
    let result = outcome.get_input_needed(&opponent);
    assert!(matches!(result, GameInput::Scissors));
}

#[test]
fn test_part2_get_game_points_with_input_str() {
    let input = include_str!("./input2.dev");
    let all_game_results = input
        .split("\n")
        .map(|game| GameOutcome::get_game_points_part2(game))
        .sum::<i32>();

    assert!(all_game_results == 12)
}
