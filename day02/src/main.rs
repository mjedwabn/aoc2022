pub mod day02 {
    use std::io::BufRead;

    pub fn get_total_score_according_to_predictions(input: &mut dyn BufRead) -> i32 {
        return parse_input(input)
            .iter()
            .map(|(p1, p2)| get_round_moves_according_to_predictions(p1, p2))
            .map(|(p1, p2)| get_round_score(p1, p2))
            .sum();
    }

    pub fn get_total_score_according_to_elf_guide(input: &mut dyn BufRead) -> i32 {
        return parse_input(input)
            .iter()
            .map(|(p1, p2)| get_round_moves_according_to_elf_guide(p1, p2))
            .map(|(p1, p2)| get_round_score(p1, p2))
            .sum();
    }

    fn parse_input(input: &mut dyn BufRead) -> Vec<(char, char)> {
        return read_input(input)
            .iter()
            .map(|round| parse_round(round))
            .collect();
    }

    fn read_input(input: &mut dyn BufRead) -> Vec<String> {
        return input.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
    }

    fn parse_round(round: &String) -> (char, char) {
        let parts: Vec<&str> = round.split(" ").collect();
        return (parts[0].chars().nth(0).unwrap(), parts[1].chars().nth(0).unwrap());
    }

    fn get_round_score(p1: Move, p2: Move) -> i32 {
        let player_two_move_score: i32 = get_move_score(p2);
        let round_score: i32 = get_round_result_score(p1, p2);
        return player_two_move_score + round_score
    }

    fn get_move_score(player_move: Move) -> i32 {
        match player_move {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scisors => 3
        }
    }

    fn get_round_result_score(p1: Move, p2: Move) -> i32 {
        let rule = get_move_rule(p2);
        let round_result = rule.get_round_result(p1);
        match round_result {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Lose => 0
        }
    }

    fn get_move_rule(player_move: Move) -> GameRule {
        match player_move {
            Move::Rock => GameRule::new(player_move, Move::Scisors, Move::Paper),
            Move::Paper => GameRule::new(player_move, Move::Rock, Move::Scisors),
            Move::Scisors => GameRule::new(player_move, Move::Paper, Move::Rock)
        }
    }

    #[derive(Copy, Clone)]
    #[derive(PartialEq)]
    enum Move {
        Rock,
        Paper,
        Scisors
    }

    enum RoundResult {
        Win,
        Lose,
        Draw
    }

    struct GameRule {
        player: Move,
        stronger_than: Move,
        weaker_than: Move
    }

    impl GameRule {
        fn new(player: Move, stronger_than: Move, weaker_than: Move) -> GameRule {
            GameRule {
                player,
                stronger_than,
                weaker_than
            }
        }

        fn get_round_result(&self, opponent: Move) -> RoundResult {
            if self.stronger_than == opponent {
                return RoundResult::Win;
            }
            else if self.weaker_than == opponent {
                return RoundResult::Lose;
            }
            else {
                return RoundResult::Draw;
            }
        }

        fn reason_opponent_move(&self, result: RoundResult) -> Move {
            match result {
                RoundResult::Win => self.weaker_than,
                RoundResult::Draw => self.player,
                RoundResult::Lose => self.stronger_than
            }
        }
    }

    fn get_round_moves_according_to_predictions(player_one: &char, player_two: &char) -> (Move, Move) {
        let p1 = match player_one {
            'A' => Ok(Move::Rock),
            'B' => Ok(Move::Paper),
            'C' => Ok(Move::Scisors),
            _ => Err(())
        }.unwrap();
        let p2 = match player_two {
            'X' => Ok(Move::Rock),
            'Y' => Ok(Move::Paper),
            'Z' => Ok(Move::Scisors),
            _ => Err(())
        }.unwrap();
        return (p1, p2);
    }

    fn get_round_moves_according_to_elf_guide(player_one: &char, player_two: &char) -> (Move, Move) {
        let p1 = match player_one {
            'A' => Ok(Move::Rock),
            'B' => Ok(Move::Paper),
            'C' => Ok(Move::Scisors),
            _ => Err(())
        }.unwrap();
        let expected_result = match player_two {
            'X' => Ok(RoundResult::Lose),
            'Y' => Ok(RoundResult::Draw),
            'Z' => Ok(RoundResult::Win),
            _ => Err(())
        }.unwrap();
        let rule = get_move_rule(p1);
        let p2 = rule.reason_opponent_move(expected_result);

        return (p1, p2);
    }
}

#[cfg(test)]
mod tests {
    use crate::day02;
    use std::{fs::File, io::BufReader};

    #[test]
    fn part1_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day02::get_total_score_according_to_predictions(&mut f), 15);
    }

    #[test]
    fn part1_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day02::get_total_score_according_to_predictions(&mut f), 15632);
    }

    #[test]
    fn part2_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day02::get_total_score_according_to_elf_guide(&mut f), 12);
    }

    #[test]
    fn part2_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day02::get_total_score_according_to_elf_guide(&mut f), 14416);
    }
}
