type Score = i64;

#[derive(Copy, Clone)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn get_score_for_move(&self) -> Score {
        (*self as Score) + 1
    }

    fn get_winning_move(&self) -> Move {
        Move::from_i32(((*self as i32) + 1) % 3)
    }

    fn get_losing_move(&self) -> Move {
        Move::from_i32(((*self as i32) - 1).rem_euclid(3))
    }

    fn from_i32(i: i32) -> Move {
        match i {
            0 => Move::Rock,
            1 => Move::Paper,
            2 => Move::Scissors,
            _ => panic!("Invalid index {i}"),
        }
    }
}

struct Round {
    opponent_move: Move,
    player_move: Move,
}

pub fn calculate_total_score_1(input: &str) -> Score {
    let decoder = |_opponent_move: Move, player_move: String| match player_move.as_str() {
        "X" => Move::Rock,
        "Y" => Move::Paper,
        "Z" => Move::Scissors,
        _ => panic!("Unexpected input: {player_move}"),
    };

    calculate_total_score(input, decoder)
}

pub fn calculate_total_score_2(input: &str) -> Score {
    let decoder = |opponent_move: Move, player_move: String| match player_move.as_str() {
        "X" => opponent_move.get_losing_move(),
        "Y" => opponent_move,
        "Z" => opponent_move.get_winning_move(),
        _ => panic!("Unexpected input: {player_move}"),
    };

    calculate_total_score(input, decoder)
}

fn calculate_total_score<F>(input: &str, decode: F) -> Score
where
    F: Fn(Move, String) -> Move,
{
    input
        .lines()
        .map(|line| {
            let line = line.split_whitespace().collect::<Vec<_>>();
            let opponent_move_str = line[0];
            let player_move_str = line[1];

            let opponent_move = match opponent_move_str {
                "A" => Move::Rock,
                "B" => Move::Paper,
                "C" => Move::Scissors,
                _ => panic!("Unexpected input: {opponent_move_str}"),
            };

            let player_move = decode(opponent_move, player_move_str.to_string());

            let round = Round {
                opponent_move,
                player_move,
            };

            calculate_player_score(round)
        })
        .sum()
}

const WIN_SCORE: Score = 6;
const DRAW_SCORE: Score = 3;
const LOSS_SCORE: Score = 0;

fn calculate_player_score(round: Round) -> Score {
    let moves_diff = ((round.player_move as Score) - (round.opponent_move as Score)).rem_euclid(3);

    let round_outcome_score: Score = if moves_diff == 0 {
        DRAW_SCORE
    } else if moves_diff == 1 {
        WIN_SCORE
    } else {
        // moves_diff == 2
        LOSS_SCORE
    };

    round_outcome_score + round.player_move.get_score_for_move()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_total_score_1_simple_case() {
        let input = "\
A Y
B X
C Z";

        assert_eq!(calculate_total_score_1(input), 15);
    }

    #[test]
    fn calculate_total_score_2_simple_case() {
        let input = "\
A Y
B X
C Z";

        assert_eq!(calculate_total_score_2(input), 12);
    }

    #[test]
    fn calculate_player_score_for_win() {
        let round = Round {
            opponent_move: Move::Rock,
            player_move: Move::Paper,
        };

        assert_eq!(calculate_player_score(round), 8);
    }

    #[test]
    fn calculate_player_score_for_loss() {
        let round = Round {
            opponent_move: Move::Paper,
            player_move: Move::Rock,
        };

        assert_eq!(calculate_player_score(round), 1);
    }

    #[test]
    fn calculate_player_score_for_draw() {
        let round = Round {
            opponent_move: Move::Scissors,
            player_move: Move::Scissors,
        };

        assert_eq!(calculate_player_score(round), 6);
    }
}
